//! Module resolver for cross-file import resolution.
//!
//! Resolves relative `.fl` imports by parsing the imported file and extracting
//! exported symbols (types, functions, for-block functions, consts).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::parser::Parser;
use crate::parser::ast::*;

/// Symbols exported from a resolved module.
#[derive(Debug, Clone, Default)]
pub struct ResolvedImports {
    /// Exported type declarations: name -> TypeDecl
    pub type_decls: Vec<TypeDecl>,
    /// Exported function declarations: name -> FunctionDecl
    pub function_decls: Vec<FunctionDecl>,
    /// Exported for-block declarations
    pub for_blocks: Vec<ForBlock>,
    /// Exported const names (we just need to know they exist, typed as Unknown)
    pub const_names: Vec<String>,
    /// Exported trait declarations
    pub trait_decls: Vec<TraitDecl>,
}

/// Resolve all relative imports for a given file.
///
/// Returns a map from import source (e.g., `"./types"`) to its resolved symbols.
/// Non-relative imports (npm packages) are skipped.
/// Circular imports are handled by tracking visited files.
pub fn resolve_imports(file_path: &Path, program: &Program) -> HashMap<String, ResolvedImports> {
    let mut results = HashMap::new();
    let mut visited = HashSet::new();

    // The file's own path should be in the visited set to prevent circular imports
    if let Ok(canonical) = file_path.canonicalize() {
        visited.insert(canonical);
    } else {
        visited.insert(file_path.to_path_buf());
    }

    let base_dir = file_path.parent().unwrap_or(Path::new("."));

    for item in &program.items {
        if let ItemKind::Import(decl) = &item.kind {
            // Skip npm/non-relative imports
            if !decl.source.starts_with('.') {
                continue;
            }

            // Already resolved this source
            if results.contains_key(&decl.source) {
                continue;
            }

            if let Some(resolved) = resolve_single_import(base_dir, &decl.source, &mut visited) {
                results.insert(decl.source.clone(), resolved);
            }
        }
    }

    results
}

/// Resolve a single import source to its exported symbols.
fn resolve_single_import(
    base_dir: &Path,
    source: &str,
    visited: &mut HashSet<PathBuf>,
) -> Option<ResolvedImports> {
    let resolved_path = resolve_path(base_dir, source)?;

    // Check for circular imports
    let canonical = resolved_path
        .canonicalize()
        .unwrap_or(resolved_path.clone());
    if visited.contains(&canonical) {
        // Circular import — return empty to avoid infinite recursion
        return Some(ResolvedImports::default());
    }
    visited.insert(canonical);

    let source_code = std::fs::read_to_string(&resolved_path).ok()?;
    let program = Parser::new(&source_code).parse_program().ok()?;

    let mut imports = ResolvedImports::default();

    // Also resolve transitive imports from the imported file
    let transitive_dir = resolved_path.parent().unwrap_or(Path::new("."));
    let transitive = resolve_imports_inner(transitive_dir, &program, visited);

    // Collect transitive type decls, for-blocks, and trait decls so the checker can register them
    for resolved in transitive.values() {
        imports
            .type_decls
            .extend(resolved.type_decls.iter().cloned());
        imports
            .for_blocks
            .extend(resolved.for_blocks.iter().cloned());
        imports
            .function_decls
            .extend(resolved.function_decls.iter().cloned());
        imports
            .trait_decls
            .extend(resolved.trait_decls.iter().cloned());
    }

    for item in &program.items {
        match &item.kind {
            ItemKind::TypeDecl(decl) if decl.exported => {
                imports.type_decls.push(decl.clone());
            }
            ItemKind::Function(decl) if decl.exported => {
                imports.function_decls.push(decl.clone());
            }
            ItemKind::ForBlock(block) => {
                // Only include exported for-block functions
                let mut exported_block = block.clone();
                exported_block.functions.retain(|f| f.exported);
                if !exported_block.functions.is_empty() {
                    imports.for_blocks.push(exported_block);
                }
            }
            ItemKind::TraitDecl(decl) if decl.exported => {
                imports.trait_decls.push(decl.clone());
            }
            ItemKind::Const(decl) if decl.exported => {
                if let ConstBinding::Name(name) = &decl.binding {
                    imports.const_names.push(name.clone());
                }
            }
            _ => {}
        }
    }

    Some(imports)
}

/// Internal version that accepts a mutable visited set for transitive resolution.
fn resolve_imports_inner(
    base_dir: &Path,
    program: &Program,
    visited: &mut HashSet<PathBuf>,
) -> HashMap<String, ResolvedImports> {
    let mut results = HashMap::new();

    for item in &program.items {
        if let ItemKind::Import(decl) = &item.kind {
            if !decl.source.starts_with('.') {
                continue;
            }
            if results.contains_key(&decl.source) {
                continue;
            }
            if let Some(resolved) = resolve_single_import(base_dir, &decl.source, visited) {
                results.insert(decl.source.clone(), resolved);
            }
        }
    }

    results
}

/// Resolve a relative import path to an actual file path.
/// Tries `source.fl` first, then `source/index.fl`.
fn resolve_path(base_dir: &Path, source: &str) -> Option<PathBuf> {
    let relative = PathBuf::from(source);
    let candidate = base_dir.join(&relative).with_extension("fl");
    if candidate.is_file() {
        return Some(candidate);
    }

    // Try source/index.fl
    let index_candidate = base_dir.join(&relative).join("index.fl");
    if index_candidate.is_file() {
        return Some(index_candidate);
    }

    None
}
