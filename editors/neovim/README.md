# ZenScript for Neovim

## Quick Setup

### 1. File detection

Add to your Neovim config (`init.lua` or a file in `after/ftdetect/`):

```lua
vim.filetype.add({
  extension = {
    zs = "zenscript",
  },
})
```

### 2. LSP configuration

Using **nvim-lspconfig** (recommended):

```lua
local lspconfig = require("lspconfig")
local configs = require("lspconfig.configs")

-- Register the ZenScript LSP if not already defined
if not configs.zenscript then
  configs.zenscript = {
    default_config = {
      cmd = { "zsc", "lsp" },
      filetypes = { "zenscript" },
      root_dir = lspconfig.util.root_pattern("zenscript.toml", ".git"),
      settings = {},
    },
  }
end

lspconfig.zenscript.setup({})
```

Without nvim-lspconfig (built-in `vim.lsp.start`):

```lua
vim.api.nvim_create_autocmd("FileType", {
  pattern = "zenscript",
  callback = function()
    vim.lsp.start({
      name = "zenscript",
      cmd = { "zsc", "lsp" },
      root_dir = vim.fs.dirname(vim.fs.find({ "zenscript.toml", ".git" }, { upward = true })[1]),
    })
  end,
})
```

### 3. Syntax highlighting (optional)

For basic highlighting without Tree-sitter, copy `syntax/zenscript.vim` into
`~/.config/nvim/syntax/zenscript.vim` (or the equivalent path for your setup).

For Tree-sitter support, a grammar will be provided in a future release.

## Features

Once configured, you get:

- **Diagnostics** — parse and type errors shown inline
- **Hover** — type info and documentation on hover (`K`)
- **Completions** — symbols, keywords, builtins, cross-file with auto-import
- **Go to Definition** — jump to symbol definition (`gd`)
- **Find References** — find all usages (`gr`)
- **Document Symbols** — outline view (`:Telescope lsp_document_symbols` or similar)

## Requirements

- `zsc` must be in your `$PATH` (install via `cargo install zenscript` or build from source)
- Neovim 0.8+ (for native LSP support)
