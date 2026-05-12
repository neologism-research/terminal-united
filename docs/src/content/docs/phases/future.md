---
title: Future — Phases 5 & 6
description: What's intentionally not yet implemented.
---

The roadmap defines two phases beyond the current build. They are out of
scope for this iteration but are summarized here so the surface area is
clear.

## Phase 5 — Dungeons & PvE

- Integrate `rot-js` for procedural dungeon matrices in
  `packages/map-engine` (BSP / Cellular Automata).
- Add a `DungeonRoom` Colyseus instance that spins up a unique map on demand.
- Use the `pathfinding` library for AI enemy movement (A\*).
- Validate with unit tests asserting valid maps and non-clipping paths.

A natural extension point: add `DungeonRoom` next to `HubRoom`/`CasinoRoom`
under `apps/server/src/rooms/` and gate it behind a new portal tile (e.g.
`D`) in `hub.json`.

## Phase 6 — Monetization & Polish

- Detect terminal image capabilities (Sixel / Kitty graphics protocol).
- Render `.png` digital billboards inline with an ASCII fallback.
- Finalize a deployment story (Docker Compose, env-driven config).
- Tag a `v1.0.0` release.

The Express/Colyseus split in the current server is already deploy-friendly:
a single Node process exposes both HTTP and the WebSocket transport on the
same port, which is enough for a small Docker Compose stack with a Postgres
volume.
