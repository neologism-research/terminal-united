---
title: Getting Started
description: Local setup, running the server, and connecting the client.
---

## Prerequisites

This project uses [mise](https://mise.jdx.dev/) to manage tool versions.
Install mise, then trust the repo:

```sh
mise install
mise exec -- node --version   # v24+
mise exec -- pnpm --version   # 11+
```

## Install

```sh
pnpm install
```

The repo enables `allowBuilds` for native packages in `pnpm-workspace.yaml`.
First install will compile esbuild, sharp, and terminal dependencies when
needed.

## Start the local database

```sh
docker compose up -d postgres
cp apps/server/.env.example apps/server/.env
```

The compose file exposes Postgres on `localhost:5432`, and the server env
points at the matching local `terminal_united` database.

## Initialize the database

```sh
pnpm --filter @tu/server db:push
```

This pushes the Drizzle schema in `apps/server/src/db/schema.ts` to the local
Postgres database. Re-run any time you change the schema.

## Run the server

```sh
pnpm dev:server
```

The Colyseus server listens on `:2567` by default (`PORT` env var to change).
HTTP endpoints:

- `POST /auth/register` — `{ username, password }` → `{ token, user }`
- `POST /auth/login` — same shape
- `GET /bbs` — list latest 50 forum posts
- `POST /bbs` — create a post (Bearer token required)

## Run the client

In a second terminal:

```sh
pnpm dev:client
```

You'll see a CLI login prompt. Register or log in, and the terminal fills
with the hub map. Controls:

- `W A S D` or arrow keys — move
- `T` — open chat input
- `B` — open the Clubhouse BBS
- Walk onto a `C` portal tile to enter the Casino
- `Q` / `Ctrl-C` — quit

## Running tests

```sh
pnpm test       # all packages
```

Each package owns its Vitest config (the default discovery in
`tests/**/*.test.ts`). Map parsing, blackjack math, auth, and room handlers
are covered.
