---
title: Phase 3 — Persistence & Player Accounts
description: Drizzle schemas, CLI login, wallet bootstrap.
---

## Goals

- Initialize Drizzle with the local Docker Postgres database.
- Persist `User`, `Item`, `Inventory` (and `ForumPost` for Phase 4).
- Add a CLI login prompt before the WebSocket connection.
- Have the Colyseus room hydrate `walletBalance` from the database on join.

## Drizzle schema

See `apps/server/src/db/schema.ts`.
Key fields:

```ts
export const users = pgTable("users", {
  id: text("id").primaryKey(),
  username: text("username").notNull().unique(),
  passwordHash: text("password_hash").notNull(),
  walletBalance: integer("wallet_balance").notNull().default(500),
  createdAt: timestamp("created_at", { withTimezone: true })
    .notNull()
    .defaultNow(),
});
```

`DEFAULT_WALLET_BALANCE` lives in `@tu/shared/constants` so the server and
docs can't drift.

## Auth flow

1. The client renders the login prompt (`apps/client/src/login-prompt.ts`)
   and `POST`s to `/auth/login` or `/auth/register`.
2. The server hashes passwords with Node's built-in `scrypt` (no native
   `argon2` dependency — keeps `pnpm install` clean and portable).
3. A random hex token is issued and stored in an in-memory `Map<token,
userId>` (`apps/server/src/auth.ts`).
4. The client passes the token in `client.joinOrCreate("hub", { token })`.
5. `BaseRoom.onAuth` resolves the token, loads the `User`, and Colyseus
   attaches that record as `client.auth`. `onJoin` reads `walletBalance` and
   seeds `PlayerState.balance`.

## AI validation

`apps/server/tests/auth.test.ts` mocks `apps/server/src/db/users.ts` with an
in-memory store and exercises:

- Username/password validators.
- User creation with default wallet balance.
- Duplicate-username rejection.
- Correct vs. incorrect password login.
- Token issuance and lookup.
