# Floe for Neovim

## Quick Setup

### 1. File detection

Add to your Neovim config (`init.lua` or a file in `after/ftdetect/`):

```lua
vim.filetype.add({
  extension = {
    zs = "floe",
  },
})
```

### 2. LSP configuration

Using **nvim-lspconfig** (recommended):

```lua
local lspconfig = require("lspconfig")
local configs = require("lspconfig.configs")

-- Register the Floe LSP if not already defined
if not configs.floe then
  configs.floe = {
    default_config = {
      cmd = { "floe", "lsp" },
      filetypes = { "floe" },
      root_dir = lspconfig.util.root_pattern("floe.toml", ".git"),
      settings = {},
    },
  }
end

lspconfig.floe.setup({})
```

Without nvim-lspconfig (built-in `vim.lsp.start`):

```lua
vim.api.nvim_create_autocmd("FileType", {
  pattern = "floe",
  callback = function()
    vim.lsp.start({
      name = "floe",
      cmd = { "floe", "lsp" },
      root_dir = vim.fs.dirname(vim.fs.find({ "floe.toml", ".git" }, { upward = true })[1]),
    })
  end,
})
```

### 3. Syntax highlighting (optional)

For basic highlighting without Tree-sitter, copy `syntax/floe.vim` into
`~/.config/nvim/syntax/floe.vim` (or the equivalent path for your setup).

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

- `floe` must be in your `$PATH` (install via `cargo install floe` or build from source)
- Neovim 0.8+ (for native LSP support)
