---
title: Phase 2 — Map Parsing & Layout
description: LDtk-like hub.json, server-side collision, sidebar chat.
---

## Goals

- Ship an LDtk-style `hub.json` with separate `visuals` and `collisions`
  layers.
- Implement a parser in `packages/map-engine` validated by unit tests.
- Have the server reject movement into wall tiles.
- Render the map in the client using `terminal-kit` screen buffers with a
  sidebar for chat and player roster.

## Map format

`packages/map-engine/maps/hub.json` stores the hub as a JSON document:

```json
{
  "width": 40,
  "height": 16,
  "spawn": { "x": 4, "y": 8 },
  "portals": { "casino": "casino", "clubhouse": "clubhouse" },
  "layers": {
    "visuals": ["########################################", "..."],
    "collisions": ["########################################", "..."]
  }
}
```

The `collisions` layer uses a glyph alphabet:

| Glyph      | Tile              |
| ---------- | ----------------- |
| `.` or ` ` | `empty`           |
| `#`        | `wall`            |
| `C`        | `portalCasino`    |
| `B`        | `portalClubhouse` |

`parseLdtkMap()` returns a `ParsedMap` containing both layers as 2D arrays
and a `portals` lookup from tile-code → room name.

## Collision API

`packages/map-engine/src/collision.ts` exposes:

- `isWalkable(map, x, y)` — bounds + wall check.
- `portalAt(map, x, y)` — returns the room name if the tile is a portal.
- `stepOrThrow(map, x, y, dir)` — authoritative step that **throws** when a
  move enters a wall tile (this is what the AI-validation tests assert).

`HubRoom` uses the non-throwing `isWalkable` for normal traffic and emits a
`transition` message when the player walks onto a casino portal.

## Renderer

`apps/client/src/renderer.ts` keeps a single `ScreenBuffer` and redraws on
every state change. The right-hand 32-column sidebar shows:

- Current room + the player's username and credit balance.
- A live player roster.
- A scrollable global chat tail (last `CHAT_HISTORY = 50` lines).

## AI validation

`packages/map-engine/tests/parser.test.ts` covers:

- Width/height validation and unknown-glyph rejection.
- Outer border treated as wall; interior tiles walkable.
- `stepOrThrow` throws on wall entry.
- Portal detection for the casino and clubhouse tiles.
