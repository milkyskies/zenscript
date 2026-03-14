import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import zenscript from "vite-plugin-zenscript";
import path from "node:path";

export default defineConfig({
  plugins: [
    zenscript({
      compiler: path.resolve(__dirname, "../../target/debug/zsc"),
    }),
    tailwindcss(),
  ],
  resolve: {
    extensions: [".zs", ".ts", ".tsx", ".js", ".jsx"],
  },
  esbuild: {
    include: /\.(tsx?|jsx?|zs)$/,
    loader: "tsx",
  },
});
