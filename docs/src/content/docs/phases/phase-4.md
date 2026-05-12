---
title: Phase 4 — Economy & Asynchronous Social
description: CasinoRoom, blackjack betting, Clubhouse BBS.
---

## Goals

- Add a `CasinoRoom` reachable by walking onto the casino portal in the hub.
- Implement server-authoritative blackjack with wallet deductions enforced
  by the Drizzle user repository.
- Add a `ForumPost` model and an HTTP-backed Clubhouse BBS UI.

## Hub → Casino transition

When the player steps onto a `portalCasino` tile, `HubRoom` sends the
`transition` message with `{ room: "casino" }`. The client leaves the hub
room and `joinOrCreate`s the casino room using the same auth token, then
spins up a small interactive blackjack UI (`apps/client/src/casino-ui.ts`).

## Blackjack engine

`apps/server/src/games/blackjack.ts` is a tiny, pure-function module:

- `handTotal(cards)` — best total ≤ 21, treating aces as 11 or 1.
- `playRound(draw)` — deals two cards each, runs hit-on-soft-17, returns
  `outcome ∈ {"win", "lose", "push"}` plus the actual card arrays.
- `settle(bet, outcome)` — returns the wallet delta (`+bet`, `0`, or
  `-bet`).
- `randomDraw()` — produces a `DrawFn` backed by `Math.random()`. Tests
  inject deterministic sequences instead.

## Wallet safety

`CasinoRoom.handleBet` only mutates the database after:

1. Validating the bet shape (integer, `MIN_BET ≤ amount ≤ MAX_BET`).
2. Reading the user's current `walletBalance` through `findUserById`.
3. Ensuring `balance ≥ amount` (otherwise the client gets `error:
"insufficient balance"`).
4. Computing `delta = settle(bet, outcome)`, asserting
   `balance + delta ≥ 0`, and `updateUserWalletBalance(...)` in one call.

This means the database is the source of truth for credits, and the
`PlayerState.balance` field is updated only after a successful write.

## Clubhouse BBS

The clubhouse is an HTTP-only feature — no Colyseus room. Routes live in
`apps/server/src/http/bbs-router.ts`:

- `GET /bbs` — newest 50 posts with author username.
- `POST /bbs` — create a post with `Authorization: Bearer <token>`.

The client overlay (`apps/client/src/bbs-ui.ts`) is opened from the hub by
pressing `B`. It pauses the renderer, prints the post list, and accepts
`N` to compose a new entry.

## AI validation

Two test files cover this phase end-to-end:

- `apps/server/tests/blackjack.test.ts` — ace handling, face value, win /
  lose / push settlement, deterministic rounds (win, push, bust).
- `apps/server/tests/casino-room.test.ts` — wallet credit on a winning
  round, rejection of bets exceeding balance (and no negative balance), and
  rejection of malformed bet payloads. The user repository is mocked with an
  in-memory store and `randomDraw` is replaced with a deterministic sequence.
