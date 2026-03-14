import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  site: "https://milkyskies.github.io",
  base: "/zenscript",
  vite: {
    ssr: {
      noExternal: ["zod"],
    },
  },
  integrations: [
    starlight({
      title: "ZenScript",
      description:
        "A Gleam-inspired language that compiles to TypeScript + React",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/milkyskies/zenscript",
        },
      ],
      sidebar: [
        {
          label: "Getting Started",
          autogenerate: { directory: "guide" },
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
        {
          label: "Tooling",
          autogenerate: { directory: "tooling" },
        },
      ],
    }),
  ],
});
