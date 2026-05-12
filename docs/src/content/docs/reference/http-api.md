---
title: HTTP API
description: REST endpoints exposed alongside the Colyseus WebSocket.
---

The Express app and the Colyseus transport share one Node process, so all
HTTP and WS traffic hits the same port (defaults to `2567`).

## Auth

### `POST /auth/register`

```json
{ "username": "alice", "password": "password" }
```

Validation:

- `username` matches `^[a-zA-Z0-9_]{3,20}$`.
- `password` length is `6..200`.

Responses:

- `200` → `{ token, user: { id, username, walletBalance } }`
- `400` → `{ error: "invalid username" | "invalid password" | "username taken" }`

### `POST /auth/login`

Same payload. `401` on bad credentials; `200` returns a fresh token bound to
the user. Tokens are in-memory and rotate on every login.

## Clubhouse BBS

### `GET /bbs`

Returns the latest 50 posts:

```json
{
  "posts": [
    {
      "id": "...",
      "title": "...",
      "body": "...",
      "author": "alice",
      "at": 1730000000000
    }
  ]
}
```

### `POST /bbs`

```http
Authorization: Bearer <token>
Content-Type: application/json

{ "title": "Hello", "body": "First post." }
```

Validation:

- `title` length `1..100`.
- `body` length `1..4000`.

Returns `{ id }` or `401` / `400`.

## Health

`GET /health` → `{ ok: true }`. Used by the client during startup to verify
the server is reachable.
