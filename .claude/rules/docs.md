# Documentation Updates

When adding or modifying language syntax, **always update all three**:

1. **`docs/design.md`** — the compiler spec. Update the relevant section (AST nodes, codegen table, type checker rules, etc.)
2. **`docs/site/`** — the user-facing docs. Update the relevant pages (guide, reference, examples, etc.)
3. **`docs/llms.txt`** — the LLM quick reference. Update syntax examples, compilation tables, and rules.

These serve different audiences:
- `design.md` is for compiler developers (agents and contributors)
- `site/` is for language users (developers writing Floe)
- `llms.txt` is for LLMs writing Floe code (concise syntax + codegen reference)

Never update one without the others. If a syntax change touches any of them, update all three in the same PR.
