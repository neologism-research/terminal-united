// @ts-check
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";

export default defineConfig({
  integrations: [
    starlight({
      title: "Terminal United — Dev Docs",
      description:
        "Internal development documentation for the Terminal United project.",
      sidebar: [
        {
          label: "Overview",
          items: [
            { label: "Introduction", slug: "overview/introduction" },
            { label: "Architecture", slug: "overview/architecture" },
            { label: "Getting Started", slug: "overview/getting-started" },
          ],
        },
        {
          label: "Phases",
          items: [
            { label: "Phase 1 — Engine & Netcode", slug: "phases/phase-1" },
            { label: "Phase 2 — Map & Layout", slug: "phases/phase-2" },
            { label: "Phase 3 — Persistence", slug: "phases/phase-3" },
            { label: "Phase 4 — Economy & BBS", slug: "phases/phase-4" },
            { label: "Future: Phases 5–6", slug: "phases/future" },
          ],
        },
        {
          label: "Reference",
          items: [
            { label: "Packages", slug: "reference/packages" },
            { label: "Schemas & Messages", slug: "reference/schemas" },
            { label: "HTTP API", slug: "reference/http-api" },
            { label: "Testing", slug: "reference/testing" },
          ],
        },
      ],
    }),
  ],
});
