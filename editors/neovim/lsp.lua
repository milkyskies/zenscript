-- Floe LSP setup for Neovim
-- Copy this into your init.lua or a dedicated config file.

-- 1. Register .fl files as floe filetype
vim.filetype.add({
  extension = {
    fl = "floe",
  },
})

-- 2. Start the LSP when a floe file is opened
vim.api.nvim_create_autocmd("FileType", {
  pattern = "floe",
  callback = function()
    vim.lsp.start({
      name = "floe",
      cmd = { "floe", "lsp" },
      root_dir = vim.fs.dirname(
        vim.fs.find({ "floe.toml", ".git" }, { upward = true })[1]
      ),
    })
  end,
})
