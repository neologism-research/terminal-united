---
title: Architecture
description: High-level layout of the monorepo and runtime topology.
---

## Monorepo layout

The repo is a strict pnpm workspace. Both apps depend on the shared packages
through `workspace:*` so types and constants stay in sync.

```text
apps/
  client/       # terminal-kit UI (login, hub renderer, casino UI, BBS UI)
  server/       # Colyseus rooms + HTTP routes + Drizzle repositories
packages/
  shared/       # @tu/shared — Colyseus schemas, constants, message types
  map-engine/   # @tu/map-engine — LDtk parser + collision helpers + hub.json
docs/           # @tu/docs — this Astro Starlight site
```

The shared packages export source `.ts` directly. There is no separate build
step; `tsx` runs both apps from `src/` and Vitest picks up the same paths.

## Runtime topology

```
   ┌──────────────────────────┐         ┌──────────────────────────────┐
   │ Terminal (terminal-kit)  │  HTTP   │ Express                      │
   │ apps/client              ├────────►│  /auth/{login,register}      │
   │                          │         │  /bbs (GET, POST)            │
   │                          │  WS     │ Colyseus                     │
   │                          ├────────►│  Room: hub                    │
   │                          │         │  Room: casino                 │
  └──────────────────────────┘         │ Drizzle → Postgres (Docker)  │
                                        └──────────────────────────────┘
```

The Colyseus server is authoritative for movement, chat history, and casino
outcomes. The client only sends intents (`move`, `chat`, `bet`) and renders
whatever state the server publishes.

## State synchronization

Room state is defined in `packages/shared/src/schemas.ts` using the
`@colyseus/schema` v3 `schema()` builder:

- `RoomState`
  - `roomName: string`
  - `players: MapSchema<PlayerState>`
  - `chat: ArraySchema<ChatMessageState>`
- `PlayerState` — `username`, `x`, `y`, `facing`, `balance`
- `ChatMessageState` — `from`, `text`, `at`

Both `HubRoom` and `CasinoRoom` extend a common `BaseRoom` that handles auth,
chat, and the player schema lifecycle.
