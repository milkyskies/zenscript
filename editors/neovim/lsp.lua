-- ZenScript LSP setup for Neovim
-- Copy this into your init.lua or a dedicated config file.

-- 1. Register .zs files as zenscript filetype
vim.filetype.add({
  extension = {
    zs = "zenscript",
  },
})

-- 2. Start the LSP when a zenscript file is opened
vim.api.nvim_create_autocmd("FileType", {
  pattern = "zenscript",
  callback = function()
    vim.lsp.start({
      name = "zenscript",
      cmd = { "zsc", "lsp" },
      root_dir = vim.fs.dirname(
        vim.fs.find({ "zenscript.toml", ".git" }, { upward = true })[1]
      ),
    })
  end,
})
