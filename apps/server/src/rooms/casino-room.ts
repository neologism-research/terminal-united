import type { Client } from "@colyseus/core";
import {
  MAX_BET,
  MIN_BET,
  ROOM_NAMES,
  RoomState,
  type BetMessage,
  type CasinoResultEvent,
  type WelcomeEvent,
} from "@tu/shared";
import { findUserById, updateUserWalletBalance } from "../db/users.js";
import { playRound, randomDraw, settle } from "../games/blackjack.js";
import { BaseRoom } from "./base-room.js";

interface AuthedUser {
  id: string;
  username: string;
  walletBalance: number;
}

export class CasinoRoom extends BaseRoom {
  /** Constant spawn inside the casino map (single-room interior). */
  private static SPAWN = { x: 5, y: 3 };

  onCreate() {
    this.setState(new RoomState());
    this.state.roomName = ROOM_NAMES.casino;
    this.registerChatHandler();

    this.onMessage<BetMessage>("bet", async (client, msg) => {
      await this.handleBet(client, msg);
    });
  }

  async onJoin(client: Client, _opts: unknown, user: AuthedUser) {
    this.userIds.set(client.sessionId, user.id);
    this.addPlayer(
      client,
      user.username,
      user.walletBalance,
      CasinoRoom.SPAWN.x,
      CasinoRoom.SPAWN.y,
    );
    const welcome: WelcomeEvent = {
      sessionId: client.sessionId,
      username: user.username,
      balance: user.walletBalance,
    };
    client.send("welcome", welcome);
  }

  onLeave(client: Client) {
    this.removePlayer(client);
  }

  /** Exposed for tests. */
  async handleBet(client: Client, msg: BetMessage) {
    const amount = Number(msg?.amount);
    if (!Number.isInteger(amount) || amount < MIN_BET || amount > MAX_BET) {
      client.send("error", { error: "invalid bet" });
      return;
    }
    const userId = this.userIds.get(client.sessionId);
    const player = this.state.players.get(client.sessionId);
    if (!userId || !player) return;

    // Authoritative balance read.
    const dbUser = await findUserById(userId);
    if (!dbUser) return;
    if (dbUser.walletBalance < amount) {
      client.send("error", { error: "insufficient balance" });
      return;
    }

    const round = playRound(randomDraw());
    const delta = settle(amount, round.outcome);
    const newBalance = dbUser.walletBalance + delta;
    if (newBalance < 0) {
      // Defensive: should never happen, since delta is bounded by bet ≤ balance.
      client.send("error", { error: "balance constraint violated" });
      return;
    }

    await updateUserWalletBalance(userId, newBalance);
    player.balance = newBalance;

    const result: CasinoResultEvent = {
      outcome: round.outcome,
      delta,
      balance: newBalance,
      player: round.player,
      dealer: round.dealer,
    };
    client.send("casino:result", result);
  }
}
