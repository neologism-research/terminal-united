import { Room, type Client } from "@colyseus/core";
import { ArraySchema } from "@colyseus/schema";
import {
  CHAT_HISTORY,
  CHAT_MAX_LEN,
  ChatMessageState,
  PlayerState,
  RoomState,
  type ChatSendMessage,
} from "@tu/shared";
import { resolveToken } from "../auth.js";
import { findUserById } from "../db/users.js";

/** Base behavior shared by HubRoom + CasinoRoom: auth, chat, state. */
export abstract class BaseRoom extends Room<RoomState> {
  /** Map of sessionId → DB userId. */
  protected readonly userIds = new Map<string, string>();

  async onAuth(_client: Client, options: { token?: string }) {
    const token = options?.token ?? "";
    const userId = resolveToken(token);
    if (!userId) throw new Error("unauthenticated");
    const user = await findUserById(userId);
    if (!user) throw new Error("user not found");
    return user; // attached to client.auth
  }

  protected registerChatHandler() {
    this.onMessage<ChatSendMessage>("chat", (client, msg) => {
      const text = String(msg?.text ?? "")
        .slice(0, CHAT_MAX_LEN)
        .trim();
      if (!text) return;
      const player = this.state.players.get(client.sessionId);
      if (!player) return;
      const chat = new ChatMessageState();
      chat.from = player.username;
      chat.text = text;
      chat.at = Date.now();
      const arr = this.state.chat as ArraySchema<ChatMessageState>;
      arr.push(chat);
      while (arr.length > CHAT_HISTORY) arr.shift();
    });
  }

  protected addPlayer(
    client: Client,
    username: string,
    balance: number,
    x: number,
    y: number,
  ): PlayerState {
    const p = new PlayerState();
    p.username = username;
    p.balance = balance;
    p.x = x;
    p.y = y;
    this.state.players.set(client.sessionId, p);
    return p;
  }

  protected removePlayer(client: Client) {
    this.state.players.delete(client.sessionId);
    this.userIds.delete(client.sessionId);
  }
}
