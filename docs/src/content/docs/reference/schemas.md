---
title: Schemas & Messages
description: The wire shapes between client and server.
---

## Colyseus state

```ts
PlayerState {
  username: string
  x: number
  y: number
  facing: "up" | "down" | "left" | "right"
  balance: number   // Credits; hydrated from User.walletBalance
}

ChatMessageState { from: string; text: string; at: number }

RoomState {
  roomName: "hub" | "casino"
  players: MapSchema<PlayerState>
  chat: ArraySchema<ChatMessageState>  // capped at CHAT_HISTORY=50
}
```

## Client → Server

| Type   | Payload                                  | Where it goes |
| ------ | ---------------------------------------- | ------------- |
| `move` | `{ dir: "up"\|"down"\|"left"\|"right" }` | Hub only      |
| `chat` | `{ text: string }`                       | Both rooms    |
| `bet`  | `{ amount: number }`                     | Casino only   |

## Server → Client

| Type            | Payload                    | Notes                           |
| --------------- | -------------------------- | ------------------------------- |
| `welcome`       | `WelcomeEvent`             | Sent on join                    |
| `transition`    | `{ room: string }`         | Triggered by portal tiles       |
| `chat`          | n/a — chat flows via state | The renderer reads `state.chat` |
| `casino:result` | `CasinoResultEvent`        | Win/lose/push + new balance     |
| `error`         | `{ error: string }`        | Bet validation failures         |

### Event shapes

```ts
WelcomeEvent       { sessionId, username, balance }
CasinoResultEvent  { outcome: "win"|"lose"|"push", delta, balance,
                     player: number[], dealer: number[] }
```

Card integers use `1..13` where `1 = ace`, `11/12/13` are face cards. The
client mirrors `handTotal()` purely for display.
