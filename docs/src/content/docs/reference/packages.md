---
title: Packages
description: What each workspace package contains and how they depend on each other.
---

## `@tu/shared` (`packages/shared`)

Single source of truth for everything both apps need:

- `schemas.ts` — Colyseus `RoomState`, `PlayerState`, `ChatMessageState`
  defined with the `@colyseus/schema` v3 `schema()` builder. No decorators,
  so no special TypeScript or runtime configuration is required.
- `constants.ts` — tick rate, default wallet balance, room names, tile
  codes, chat limits, bet limits.
- `messages.ts` — TypeScript interfaces for every client↔server message
  (`MoveMessage`, `ChatSendMessage`, `BetMessage`, `BbsPostMessage`,
  `CasinoResultEvent`, `WelcomeEvent`, ...).

## `@tu/map-engine` (`packages/map-engine`)

Hub map asset plus parsing/collision helpers.

- `maps/hub.json` — the LDtk-like map shipped with the app.
- `src/types.ts` — `ParsedMap`, `LdtkLikeMap`, glyph → tile-code table.
- `src/ldtk-parser.ts` — `parseLdtkMap(raw)`.
- `src/collision.ts` — `isWalkable`, `portalAt`, `applyDir`, `stepOrThrow`.

## `@tu/server` (`apps/server`)

Colyseus + Express. Composed from small modules so a phase can move without
touching unrelated code:

- `index.ts` — wires Express, the Colyseus `Server`, and the room
  definitions.
- `env.ts` — zero-dependency `.env` loader.
- `db/schema.ts` — Drizzle table definitions for users, items, inventory,
  and forum posts.
- `db/client.ts` — Drizzle/Postgres client bootstrap from `DATABASE_URL`.
- `db/users.ts`, `db/forum-posts.ts` — persistence functions used by auth,
  rooms, and HTTP routes.
- `auth.ts`, `passwords.ts` — registration/login + scrypt password hashing.
- `http/auth-router.ts`, `http/bbs-router.ts` — Express routers.
- `maps.ts` — caches the parsed hub map for `HubRoom`.
- `rooms/base-room.ts` — auth + chat behavior shared by both rooms.
- `rooms/hub-room.ts` — movement, collision, portal transitions.
- `rooms/casino-room.ts` — blackjack betting + wallet writes.
- `games/blackjack.ts` — pure blackjack engine, easy to unit test.

## `@tu/client` (`apps/client`)

Terminal UI driven by `terminal-kit`.

- `term.ts` — single shared `term` + `ScreenBuffer` reference.
- `config.ts` — server endpoints from env (`TU_SERVER_HTTP`, `TU_SERVER_WS`).
- `api.ts` — small typed fetch wrappers for auth + BBS.
- `login-prompt.ts` — CLI login/register loop.
- `renderer.ts` — `HudRenderer` draws the map + sidebar to a screen buffer.
- `casino-ui.ts`, `bbs-ui.ts` — modal experiences for the casino and BBS.
- `index.ts` — the main loop: keys → `room.send`, transitions, redraws.

## `@tu/docs` (`docs/`)

This Astro Starlight site.
