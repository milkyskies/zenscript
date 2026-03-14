use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::checker::Checker;
use crate::diagnostic::{self as zs_diag, Severity};
use crate::parser::Parser;
use crate::parser::ast::*;

// ── Helpers ─────────────────────────────────────────────────────

fn symbol_kind_to_completion(kind: SymbolKind) -> CompletionItemKind {
    match kind {
        SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
        SymbolKind::CONSTANT => CompletionItemKind::CONSTANT,
        SymbolKind::VARIABLE => CompletionItemKind::VARIABLE,
        SymbolKind::TYPE_PARAMETER => CompletionItemKind::CLASS,
        SymbolKind::ENUM_MEMBER => CompletionItemKind::ENUM_MEMBER,
        _ => CompletionItemKind::TEXT,
    }
}

// ── Symbol Index ────────────────────────────────────────────────

/// A symbol defined in a document.
#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    kind: SymbolKind,
    /// Byte offset range in the source
    start: usize,
    end: usize,
    /// The source module for imported symbols
    import_source: Option<String>,
    /// Type signature for hover
    detail: String,
}

/// Index of all symbols in a document.
#[derive(Debug, Clone, Default)]
struct SymbolIndex {
    /// All defined/imported symbols
    symbols: Vec<Symbol>,
}

impl SymbolIndex {
    fn build(program: &Program) -> Self {
        let mut symbols = Vec::new();

        for item in &program.items {
            match &item.kind {
                ItemKind::Import(decl) => {
                    for spec in &decl.specifiers {
                        let name = spec.alias.as_ref().unwrap_or(&spec.name);
                        symbols.push(Symbol {
                            name: name.clone(),
                            kind: SymbolKind::VARIABLE,
                            start: spec.span.start,
                            end: spec.span.end,
                            import_source: Some(decl.source.clone()),
                            detail: format!("import {{ {} }} from \"{}\"", spec.name, decl.source),
                        });
                    }
                }
                ItemKind::Const(decl) => {
                    let name = match &decl.binding {
                        ConstBinding::Name(n) => n.clone(),
                        ConstBinding::Array(names) => format!("[{}]", names.join(", ")),
                        ConstBinding::Object(names) => format!("{{ {} }}", names.join(", ")),
                    };
                    let vis = if decl.exported { "export " } else { "" };
                    let type_ann = decl
                        .type_ann
                        .as_ref()
                        .map(|t| format!(": {}", type_expr_to_string(t)))
                        .unwrap_or_default();
                    symbols.push(Symbol {
                        name: name.clone(),
                        kind: SymbolKind::CONSTANT,
                        start: item.span.start,
                        end: item.span.end,
                        import_source: None,
                        detail: format!("{vis}const {name}{type_ann}"),
                    });

                    // Also index destructured names
                    match &decl.binding {
                        ConstBinding::Array(names) | ConstBinding::Object(names) => {
                            for n in names {
                                symbols.push(Symbol {
                                    name: n.clone(),
                                    kind: SymbolKind::VARIABLE,
                                    start: item.span.start,
                                    end: item.span.end,
                                    import_source: None,
                                    detail: format!("const {{ {n} }}"),
                                });
                            }
                        }
                        ConstBinding::Name(_) => {}
                    }
                }
                ItemKind::Function(decl) => {
                    let vis = if decl.exported { "export " } else { "" };
                    let async_kw = if decl.async_fn { "async " } else { "" };
                    let params: Vec<String> = decl
                        .params
                        .iter()
                        .map(|p| {
                            if let Some(ty) = &p.type_ann {
                                format!("{}: {}", p.name, type_expr_to_string(ty))
                            } else {
                                p.name.clone()
                            }
                        })
                        .collect();
                    let ret = decl
                        .return_type
                        .as_ref()
                        .map(|t| format!(": {}", type_expr_to_string(t)))
                        .unwrap_or_default();
                    symbols.push(Symbol {
                        name: decl.name.clone(),
                        kind: SymbolKind::FUNCTION,
                        start: item.span.start,
                        end: item.span.end,
                        import_source: None,
                        detail: format!(
                            "{vis}{async_kw}function {}({}){ret}",
                            decl.name,
                            params.join(", ")
                        ),
                    });

                    // Index parameters
                    for param in &decl.params {
                        let type_ann = param
                            .type_ann
                            .as_ref()
                            .map(|t| format!(": {}", type_expr_to_string(t)))
                            .unwrap_or_default();
                        symbols.push(Symbol {
                            name: param.name.clone(),
                            kind: SymbolKind::VARIABLE,
                            start: param.span.start,
                            end: param.span.end,
                            import_source: None,
                            detail: format!("parameter {}{type_ann}", param.name),
                        });
                    }
                }
                ItemKind::TypeDecl(decl) => {
                    let vis = if decl.exported { "export " } else { "" };
                    let opaque = if decl.opaque { "opaque " } else { "" };
                    symbols.push(Symbol {
                        name: decl.name.clone(),
                        kind: SymbolKind::TYPE_PARAMETER,
                        start: item.span.start,
                        end: item.span.end,
                        import_source: None,
                        detail: format!("{vis}{opaque}type {}", decl.name),
                    });

                    // Index union variants
                    if let TypeDef::Union(variants) = &decl.def {
                        for variant in variants {
                            symbols.push(Symbol {
                                name: variant.name.clone(),
                                kind: SymbolKind::ENUM_MEMBER,
                                start: variant.span.start,
                                end: variant.span.end,
                                import_source: None,
                                detail: format!("{}.{}", decl.name, variant.name),
                            });
                        }
                    }
                }
                ItemKind::Expr(_) => {}
            }
        }

        Self { symbols }
    }

    fn find_by_name(&self, name: &str) -> Vec<&Symbol> {
        self.symbols.iter().filter(|s| s.name == name).collect()
    }

    fn all_completions(&self) -> Vec<&Symbol> {
        self.symbols.iter().collect()
    }
}

fn type_expr_to_string(ty: &TypeExpr) -> String {
    match &ty.kind {
        TypeExprKind::Named { name, type_args } => {
            if type_args.is_empty() {
                name.clone()
            } else {
                let args: Vec<String> = type_args.iter().map(type_expr_to_string).collect();
                format!("{}<{}>", name, args.join(", "))
            }
        }
        TypeExprKind::Record(fields) => {
            let fs: Vec<String> = fields
                .iter()
                .map(|f| format!("{}: {}", f.name, type_expr_to_string(&f.type_ann)))
                .collect();
            format!("{{ {} }}", fs.join(", "))
        }
        TypeExprKind::Function {
            params,
            return_type,
        } => {
            let ps: Vec<String> = params.iter().map(type_expr_to_string).collect();
            format!(
                "({}) => {}",
                ps.join(", "),
                type_expr_to_string(return_type)
            )
        }
        TypeExprKind::Array(inner) => format!("Array<{}>", type_expr_to_string(inner)),
        TypeExprKind::Tuple(parts) => {
            let ps: Vec<String> = parts.iter().map(type_expr_to_string).collect();
            format!("[{}]", ps.join(", "))
        }
    }
}

// ── Document State ──────────────────────────────────────────────

/// State for an open document.
#[derive(Debug, Clone)]
struct Document {
    content: String,
    index: SymbolIndex,
}

/// The ZenScript Language Server.
pub struct ZenScriptLsp {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, Document>>>,
}

impl ZenScriptLsp {
    fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Parse and type-check a document, update symbol index, publish diagnostics.
    async fn update_document(&self, uri: Url, source: &str) {
        let (diagnostics, index) = match Parser::new(source).parse_program() {
            Err(parse_errors) => {
                let zs_diags = zs_diag::from_parse_errors(&parse_errors);
                (
                    self.convert_diagnostics(source, &zs_diags),
                    SymbolIndex::default(),
                )
            }
            Ok(program) => {
                let index = SymbolIndex::build(&program);
                let check_diags = Checker::new().check(&program);
                (self.convert_diagnostics(source, &check_diags), index)
            }
        };

        self.documents.write().await.insert(
            uri.clone(),
            Document {
                content: source.to_string(),
                index,
            },
        );

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    /// Convert ZenScript diagnostics to LSP diagnostics.
    fn convert_diagnostics(
        &self,
        source: &str,
        zs_diagnostics: &[zs_diag::Diagnostic],
    ) -> Vec<Diagnostic> {
        zs_diagnostics
            .iter()
            .map(|d| {
                let severity = match d.severity {
                    Severity::Error => DiagnosticSeverity::ERROR,
                    Severity::Warning => DiagnosticSeverity::WARNING,
                    Severity::Help => DiagnosticSeverity::HINT,
                };

                let range = offset_to_range(source, d.span.start, d.span.end);

                Diagnostic {
                    range,
                    severity: Some(severity),
                    code: d.code.as_ref().map(|c| NumberOrString::String(c.clone())),
                    source: Some("zenscript".to_string()),
                    message: d.message.clone(),
                    related_information: None,
                    tags: None,
                    code_description: None,
                    data: None,
                }
            })
            .collect()
    }
}

// ── LSP Protocol ────────────────────────────────────────────────

/// ZenScript keywords and builtins for completion.
const KEYWORDS: &[(&str, &str)] = &[
    ("const", "const ${1:name} = ${0:value}"),
    (
        "function",
        "function ${1:name}(${2:params}): ${3:ReturnType} {\n\t$0\n}",
    ),
    ("export", "export "),
    ("import", "import { ${1:name} } from \"${0:module}\""),
    (
        "match",
        "match ${1:expr} {\n\t${2:pattern} -> ${3:body},\n\t_ -> ${0:default},\n}",
    ),
    ("type", "type ${1:Name} = {\n\t${0:field}: ${2:Type},\n}"),
    ("return", "return ${0:expr}"),
    ("if", "if ${1:condition} {\n\t${0:body}\n}"),
    ("async", "async "),
    ("await", "await ${0:expr}"),
    ("opaque", "opaque type ${1:Name} = ${0:BaseType}"),
];

const BUILTINS: &[(&str, &str, &str)] = &[
    ("Ok", "Ok(${0:value})", "Ok(value: T) -> Result<T, E>"),
    ("Err", "Err(${0:error})", "Err(error: E) -> Result<T, E>"),
    ("Some", "Some(${0:value})", "Some(value: T) -> Option<T>"),
    ("None", "None", "None -> Option<T>"),
    ("true", "true", "bool literal"),
    ("false", "false", "bool literal"),
];

#[tower_lsp::async_trait]
impl LanguageServer for ZenScriptLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), "|".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "zenscript-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ZenScript LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        self.update_document(uri, &content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next_back() {
            self.update_document(uri, &change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents
            .write()
            .await
            .remove(&params.text_document.uri);
    }

    // ── Hover ───────────────────────────────────────────────────

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        let offset = position_to_offset(&doc.content, position);
        let word = word_at_offset(&doc.content, offset);

        if word.is_empty() {
            return Ok(None);
        }

        // Check symbol index first
        let symbols = doc.index.find_by_name(word);
        if let Some(sym) = symbols.first() {
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("```zenscript\n{}\n```", sym.detail),
                }),
                range: None,
            }));
        }

        // Fallback to builtin hover
        let hover_text = match word {
            "Ok" => {
                "```zenscript\nOk(value: T) -> Result<T, E>\n```\nWrap a success value in a Result."
            }
            "Err" => {
                "```zenscript\nErr(error: E) -> Result<T, E>\n```\nWrap an error value in a Result."
            }
            "Some" => "```zenscript\nSome(value: T) -> Option<T>\n```\nWrap a value in an Option.",
            "None" => "```zenscript\nNone -> Option<T>\n```\nRepresents the absence of a value.",
            "match" => {
                "```zenscript\nmatch expr { pattern -> body, ... }\n```\nExhaustive pattern matching expression."
            }
            "|>" => {
                "```zenscript\nexpr |> function\n```\nPipe operator: passes left side as first argument to right side."
            }
            _ => return Ok(None),
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_text.to_string(),
            }),
            range: None,
        }))
    }

    // ── Completion ──────────────────────────────────────────────

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        let offset = position_to_offset(&doc.content, position);
        let prefix = word_prefix_at_offset(&doc.content, offset);

        let mut items = Vec::new();

        // Symbols from the current document
        for sym in doc.index.all_completions() {
            if !prefix.is_empty() && !sym.name.starts_with(&prefix) {
                continue;
            }
            items.push(CompletionItem {
                label: sym.name.clone(),
                kind: Some(symbol_kind_to_completion(sym.kind)),
                detail: Some(sym.detail.clone()),
                insert_text: Some(sym.name.clone()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            });
        }

        // Symbols from other open documents (cross-file completions + auto-import)
        for (other_uri, other_doc) in docs.iter() {
            if other_uri == &uri {
                continue;
            }
            for sym in &other_doc.index.symbols {
                if sym.import_source.is_some() {
                    continue; // Don't re-export imports from other files
                }
                if !prefix.is_empty() && !sym.name.starts_with(&prefix) {
                    continue;
                }
                let relative_path = other_uri
                    .path_segments()
                    .and_then(|mut s| s.next_back())
                    .unwrap_or("unknown")
                    .trim_end_matches(".zs");

                // Auto-import: add the import statement as an additional edit
                let import_edit =
                    format!("import {{ {} }} from \"./{}\"\n", sym.name, relative_path);

                items.push(CompletionItem {
                    label: sym.name.clone(),
                    kind: Some(symbol_kind_to_completion(sym.kind)),
                    detail: Some(format!(
                        "{} (auto-import from {})",
                        sym.detail, relative_path
                    )),
                    insert_text: Some(sym.name.clone()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    additional_text_edits: Some(vec![TextEdit {
                        range: Range {
                            start: Position::new(0, 0),
                            end: Position::new(0, 0),
                        },
                        new_text: import_edit,
                    }]),
                    ..Default::default()
                });
            }
        }

        // Keywords
        for (kw, snippet) in KEYWORDS {
            if !prefix.is_empty() && !kw.starts_with(&prefix) {
                continue;
            }
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some(snippet.to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        // Builtins
        for (name, snippet, detail) in BUILTINS {
            if !prefix.is_empty() && !name.starts_with(&prefix) {
                continue;
            }
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some(detail.to_string()),
                insert_text: Some(snippet.to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    // ── Go to Definition ────────────────────────────────────────

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        let offset = position_to_offset(&doc.content, position);
        let word = word_at_offset(&doc.content, offset);

        if word.is_empty() {
            return Ok(None);
        }

        // Search current document
        for sym in doc.index.find_by_name(word) {
            // Skip the symbol at the cursor position itself (don't jump to yourself)
            if offset >= sym.start && offset <= sym.end {
                continue;
            }
            let range = offset_to_range(&doc.content, sym.start, sym.end);
            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: uri.clone(),
                range,
            })));
        }

        // Search other open documents
        for (other_uri, other_doc) in docs.iter() {
            if other_uri == &uri {
                continue;
            }
            for sym in other_doc.index.find_by_name(word) {
                if sym.import_source.is_some() {
                    continue;
                }
                let range = offset_to_range(&other_doc.content, sym.start, sym.end);
                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: other_uri.clone(),
                    range,
                })));
            }
        }

        Ok(None)
    }

    // ── Find References ─────────────────────────────────────────

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        let offset = position_to_offset(&doc.content, position);
        let word = word_at_offset(&doc.content, offset);

        if word.is_empty() {
            return Ok(None);
        }

        let mut locations = Vec::new();

        // Find all occurrences in all open documents
        for (doc_uri, doc) in docs.iter() {
            let source = &doc.content;
            let mut search_from = 0;
            while let Some(pos) = source[search_from..].find(word) {
                let abs_pos = search_from + pos;
                let end_pos = abs_pos + word.len();

                // Check it's a whole word match
                let before_ok = abs_pos == 0 || !is_word_char(source.as_bytes()[abs_pos - 1]);
                let after_ok = end_pos >= source.len() || !is_word_char(source.as_bytes()[end_pos]);

                if before_ok && after_ok {
                    let range = offset_to_range(source, abs_pos, end_pos);
                    locations.push(Location {
                        uri: doc_uri.clone(),
                        range,
                    });
                }

                search_from = abs_pos + 1;
            }
        }

        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(locations))
        }
    }

    // ── Document Symbols ────────────────────────────────────────

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        #[allow(deprecated)]
        let symbols: Vec<SymbolInformation> = doc
            .index
            .symbols
            .iter()
            .filter(|s| s.import_source.is_none()) // Skip imports in outline
            .map(|s| {
                let range = offset_to_range(&doc.content, s.start, s.end);
                SymbolInformation {
                    name: s.name.clone(),
                    kind: s.kind,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range,
                    },
                    container_name: None,
                }
            })
            .collect();

        Ok(Some(DocumentSymbolResponse::Flat(symbols)))
    }
}

// ── Helpers ─────────────────────────────────────────────────────

fn offset_to_range(source: &str, start: usize, end: usize) -> Range {
    let start_pos = offset_to_position(source, start);
    let end_pos = offset_to_position(source, end);
    Range {
        start: start_pos,
        end: end_pos,
    }
}

fn offset_to_position(source: &str, offset: usize) -> Position {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    Position::new(line, col)
}

fn position_to_offset(source: &str, position: Position) -> usize {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, ch) in source.char_indices() {
        if line == position.line && col == position.character {
            return i;
        }
        if ch == '\n' {
            if line == position.line {
                return i;
            }
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    source.len()
}

fn word_at_offset(source: &str, offset: usize) -> &str {
    let bytes = source.as_bytes();
    if offset >= bytes.len() {
        return "";
    }

    let mut start = offset;
    while start > 0 && is_word_char(bytes[start - 1]) {
        start -= 1;
    }

    let mut end = offset;
    while end < bytes.len() && is_word_char(bytes[end]) {
        end += 1;
    }

    &source[start..end]
}

/// Get the word prefix before the cursor (for completion filtering).
fn word_prefix_at_offset(source: &str, offset: usize) -> String {
    let bytes = source.as_bytes();
    let mut start = offset;
    while start > 0 && is_word_char(bytes[start - 1]) {
        start -= 1;
    }
    source[start..offset].to_string()
}

fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Start the LSP server on stdin/stdout.
pub async fn run_lsp() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(ZenScriptLsp::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_to_position_first_line() {
        let source = "const x = 42";
        let pos = offset_to_position(source, 6);
        assert_eq!(pos.line, 0);
        assert_eq!(pos.character, 6);
    }

    #[test]
    fn offset_to_position_second_line() {
        let source = "const x = 42\nconst y = 10";
        let pos = offset_to_position(source, 19);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.character, 6);
    }

    #[test]
    fn offset_to_range_basic() {
        let source = "const x = 42";
        let range = offset_to_range(source, 6, 7);
        assert_eq!(range.start.line, 0);
        assert_eq!(range.start.character, 6);
        assert_eq!(range.end.line, 0);
        assert_eq!(range.end.character, 7);
    }

    #[test]
    fn position_to_offset_roundtrip() {
        let source = "hello\nworld\nfoo";
        let offset = position_to_offset(source, Position::new(1, 3));
        assert_eq!(offset, 9);
    }

    #[test]
    fn word_at_offset_basic() {
        let source = "const hello = 42";
        assert_eq!(word_at_offset(source, 6), "hello");
        assert_eq!(word_at_offset(source, 8), "hello");
    }

    #[test]
    fn word_at_offset_at_boundary() {
        let source = "const x = 42";
        assert_eq!(word_at_offset(source, 0), "const");
    }

    #[test]
    fn word_prefix_at_offset_partial() {
        let source = "const hel";
        assert_eq!(word_prefix_at_offset(source, 9), "hel");
    }

    #[test]
    fn word_prefix_at_offset_empty() {
        let source = "const ";
        assert_eq!(word_prefix_at_offset(source, 6), "");
    }

    #[test]
    fn banned_keyword_produces_parse_error() {
        let source = "let x = 42";
        let parse_result = Parser::new(source).parse_program();
        assert!(parse_result.is_err());
        let errs = parse_result.unwrap_err();
        let zs_diags = zs_diag::from_parse_errors(&errs);
        assert!(!zs_diags.is_empty());
        assert_eq!(zs_diags[0].severity, Severity::Error);
    }

    #[test]
    fn symbol_index_function() {
        let source = "function add(a: number, b: number): number { a + b }";
        let program = Parser::new(source).parse_program().unwrap();
        let index = SymbolIndex::build(&program);
        let syms = index.find_by_name("add");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].kind, SymbolKind::FUNCTION);
        assert!(syms[0].detail.contains("function add"));
    }

    #[test]
    fn symbol_index_const() {
        let source = "const x = 42";
        let program = Parser::new(source).parse_program().unwrap();
        let index = SymbolIndex::build(&program);
        let syms = index.find_by_name("x");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].kind, SymbolKind::CONSTANT);
    }

    #[test]
    fn symbol_index_type() {
        let source = "type User = { name: string, age: number }";
        let program = Parser::new(source).parse_program().unwrap();
        let index = SymbolIndex::build(&program);
        let syms = index.find_by_name("User");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].kind, SymbolKind::TYPE_PARAMETER);
    }

    #[test]
    fn symbol_index_import() {
        let source = r#"import { useState } from "react""#;
        let program = Parser::new(source).parse_program().unwrap();
        let index = SymbolIndex::build(&program);
        let syms = index.find_by_name("useState");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].import_source.as_deref(), Some("react"));
    }

    #[test]
    fn symbol_index_union_variants() {
        let source = "type Color = | Red | Green | Blue";
        let program = Parser::new(source).parse_program().unwrap();
        let index = SymbolIndex::build(&program);
        assert_eq!(index.find_by_name("Color").len(), 1);
        assert_eq!(index.find_by_name("Red").len(), 1);
        assert_eq!(index.find_by_name("Green").len(), 1);
        assert_eq!(index.find_by_name("Blue").len(), 1);
    }

    #[test]
    fn type_expr_to_string_named() {
        let ty = TypeExpr {
            kind: TypeExprKind::Named {
                name: "string".to_string(),
                type_args: vec![],
            },
            span: crate::lexer::span::Span::new(0, 0, 1, 1),
        };
        assert_eq!(type_expr_to_string(&ty), "string");
    }

    #[test]
    fn type_expr_to_string_generic() {
        let ty = TypeExpr {
            kind: TypeExprKind::Named {
                name: "Result".to_string(),
                type_args: vec![
                    TypeExpr {
                        kind: TypeExprKind::Named {
                            name: "User".to_string(),
                            type_args: vec![],
                        },
                        span: crate::lexer::span::Span::new(0, 0, 1, 1),
                    },
                    TypeExpr {
                        kind: TypeExprKind::Named {
                            name: "Error".to_string(),
                            type_args: vec![],
                        },
                        span: crate::lexer::span::Span::new(0, 0, 1, 1),
                    },
                ],
            },
            span: crate::lexer::span::Span::new(0, 0, 1, 1),
        };
        assert_eq!(type_expr_to_string(&ty), "Result<User, Error>");
    }
}
