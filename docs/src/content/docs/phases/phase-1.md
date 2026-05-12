---
title: Phase 1 — Engine & Netcode Foundation
description: pnpm workspace bootstrap, HubRoom, terminal-kit client, WASD.
---

## Goals

- Initialize a pnpm workspace with `apps/server`, `apps/client`, and shared
  packages.
- Set up Vitest for every workspace.
- Stand up a Colyseus `HubRoom` and connect a `terminal-kit` client.
- Render an `@` player token on a blank grid.
- Drive movement with WASD/arrow keys via Colyseus messages.

## Deliverables in this repo

| Concern                | Path                                                        |
| ---------------------- | ----------------------------------------------------------- |
| pnpm workspace         | [`pnpm-workspace.yaml`](../../../../../pnpm-workspace.yaml) |
| Shared types/schemas   | `packages/shared/src/`                                      |
| Server entry           | `apps/server/src/index.ts`                                  |
| Hub room               | `apps/server/src/rooms/hub-room.ts`                         |
| Base room (auth, chat) | `apps/server/src/rooms/base-room.ts`                        |
| Client entry           | `apps/client/src/index.ts`                                  |
| Renderer               | `apps/client/src/renderer.ts`                               |

## How movement flows

1. Client reads a key via `terminal-kit`'s `term.on("key", ...)`.
2. Key is mapped to a `DirInput` (`up`/`down`/`left`/`right`).
3. Client sends `room.send("move", { dir })`.
4. `HubRoom.handleMove` validates and either mutates the player schema or
   silently rejects the input.
5. State delta is broadcast; the renderer redraws via `r.onStateChange`.

## AI validation

`apps/server/tests/hub-room.test.ts` constructs the `HubRoom` without a real
network transport (by assigning state via `setState` directly) and asserts:

- WASD inputs mutate `PlayerState.x`/`y`/`facing`.
- A move into a wall tile leaves the position unchanged.
- Invalid direction strings are ignored.

## Git checkpoint

Per the roadmap: commit on `feature/phase-1`, merge to `main`.
