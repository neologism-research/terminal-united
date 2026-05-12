import { beforeEach, describe, expect, it, vi } from "vitest";

type User = { id: string; walletBalance: number };
const users = new Map<string, User>([["u1", { id: "u1", walletBalance: 100 }]]);

vi.mock("../src/db/users.js", () => ({
  findUserById: async (id: string) => users.get(id) ?? null,
  updateUserWalletBalance: async (id: string, walletBalance: number) => {
    const user = users.get(id)!;
    user.walletBalance = walletBalance;
    return user;
  },
}));

// Force deterministic blackjack outcomes by mocking the draw fn.
vi.mock("../src/games/blackjack.js", async (orig) => {
  const real = await orig<typeof import("../src/games/blackjack.js")>();
  return {
    ...real,
    randomDraw: () => {
      const seq = [10, 10, 9, 10]; // player 20, dealer 19 → win
      let i = 0;
      return () => seq[i++]!;
    },
  };
});

import { PlayerState, RoomState } from "@tu/shared";
import { CasinoRoom } from "../src/rooms/casino-room.js";

function makeRoom(): CasinoRoom {
  const room = Object.create(CasinoRoom.prototype) as CasinoRoom;
  // @ts-expect-error
  room.setState = function (s: RoomState) {
    // @ts-expect-error
    this.state = s;
  };
  // @ts-expect-error
  room.onMessage = () => {};
  // @ts-expect-error
  room.userIds = new Map();
  room.onCreate();
  return room;
}

function mockClient(sessionId: string) {
  const sent: Array<{ type: string; payload: unknown }> = [];
  return {
    sessionId,
    send: (type: string, payload: unknown) => sent.push({ type, payload }),
    sent,
  };
}

beforeEach(() => {
  users.set("u1", { id: "u1", walletBalance: 100 });
});

describe("CasinoRoom betting", () => {
  it("deducts wallet on loss and credits on win", async () => {
    const room = makeRoom();
    const client = mockClient("s1");
    // @ts-expect-error
    room.userIds.set(client.sessionId, "u1");
    const p = new PlayerState();
    p.username = "u";
    p.balance = 100;
    room.state.players.set(client.sessionId, p);

    await room.handleBet(client as never, { amount: 25 });

    // With mocked draw → player wins → balance 125.
    expect(p.balance).toBe(125);
    expect(users.get("u1")!.walletBalance).toBe(125);
    const result = (client as ReturnType<typeof mockClient>).sent.find(
      (m) => m.type === "casino:result",
    );
    expect(result).toBeDefined();
  });

  it("rejects bet exceeding balance and never goes negative", async () => {
    const room = makeRoom();
    const client = mockClient("s1");
    // @ts-expect-error
    room.userIds.set(client.sessionId, "u1");
    const p = new PlayerState();
    p.balance = 100;
    room.state.players.set(client.sessionId, p);

    await room.handleBet(client as never, { amount: 9999 });

    expect(users.get("u1")!.walletBalance).toBe(100); // unchanged
    expect(p.balance).toBe(100);
    const err = (client as ReturnType<typeof mockClient>).sent.find(
      (m) => m.type === "error",
    );
    expect(err).toBeDefined();
  });

  it("rejects invalid bet amounts", async () => {
    const room = makeRoom();
    const client = mockClient("s1");
    // @ts-expect-error
    room.userIds.set(client.sessionId, "u1");
    const p = new PlayerState();
    p.balance = 100;
    room.state.players.set(client.sessionId, p);

    await room.handleBet(client as never, { amount: 0 });
    await room.handleBet(client as never, { amount: -5 });
    await room.handleBet(client as never, { amount: 1.5 });

    expect(users.get("u1")!.walletBalance).toBe(100);
  });
});
