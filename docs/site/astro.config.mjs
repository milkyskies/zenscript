import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  site: "https://milkyskies.github.io",
  base: "/floe",
  vite: {
    ssr: {
      noExternal: ["zod"],
    },
  },
  integrations: [
    starlight({
      title: "Floe",
      description:
        "A Gleam-inspired language that compiles to TypeScript + React",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/milkyskies/floe",
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
