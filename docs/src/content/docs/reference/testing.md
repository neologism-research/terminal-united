---
title: Testing strategy
description: How the 27 unit tests stay deterministic without a live server.
---

Every workspace package owns a `tests/` directory with Vitest. Run the full
suite with `pnpm test`.

## Map engine — `packages/map-engine/tests`

Pure functions, no mocking required. Reads the real `hub.json` and asserts
parser invariants (width × height, glyph alphabet, portal coordinates) plus
collision semantics (`isWalkable`, `stepOrThrow`).

## Blackjack — `apps/server/tests/blackjack.test.ts`

The `playRound` function takes an injectable `DrawFn`. Tests build a
deterministic generator that yields a scripted card sequence so we can pin
down win / lose / push and ace-handling cases without `Math.random`.

```ts
const draws = [10, 10, 6, 5, 7]; // player 20, dealer 16 → 23 → win
const draw = () => draws.shift()!;
expect(playRound(draw).outcome).toBe("win");
```

## Auth — `apps/server/tests/auth.test.ts`

A `vi.mock("../src/db/users.js", ...)` replaces the Drizzle user repository
with an in-memory `Map<string, UserRow>`. The same module then drives
validators, duplicate-username rejection, and password verification.

Passwords use Node's built-in `scrypt`, so no native binaries are required
in CI. The hash format is `scrypt$<salt-hex>$<hash-hex>`.

## Rooms — `apps/server/tests/{hub,casino}-room.test.ts`

Colyseus rooms are constructed without a transport by instantiating them
via `Object.create(RoomClass.prototype)` and assigning `state`, `clients`,
and helper fakes manually. The tests then call private handlers
(`handleMove`, `handleBet`) directly. This keeps the suite fast and avoids
WebSocket plumbing.

## Conventions

- **One concern per test.** Helpers and harnesses live next to the test
  file when shared, not in a global setup.
- **No live network calls.** HTTP routes are not exercised in unit tests
  yet; they are covered manually via the client. Adding `supertest` is the
  next obvious step.
- **Determinism.** Anything random takes a function parameter so tests can
  inject a stub.
