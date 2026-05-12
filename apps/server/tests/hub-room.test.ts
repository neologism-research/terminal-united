import { PlayerState, RoomState } from "@tu/shared";
import { beforeEach, describe, expect, it } from "vitest";
import { HubRoom } from "../src/rooms/hub-room.js";

/** Build a HubRoom instance with manually-installed state to bypass network. */
function makeRoom(): HubRoom {
  const room = Object.create(HubRoom.prototype) as HubRoom;
  // @ts-expect-error injecting state for unit test
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
  return {
    sessionId,
    send: () => {},
  } as unknown as import("@colyseus/core").Client;
}

describe("HubRoom WASD state updates", () => {
  let room: HubRoom;
  const client = mockClient("s1");

  beforeEach(() => {
    room = makeRoom();
    const p = new PlayerState();
    p.username = "tester";
    p.x = 4;
    p.y = 8;
    p.balance = 100;
    room.state.players.set(client.sessionId, p);
  });

  it("moves right on right input", () => {
    room.handleMove(client, { dir: "right" });
    const p = room.state.players.get(client.sessionId)!;
    expect(p.x).toBe(5);
    expect(p.y).toBe(8);
    expect(p.facing).toBe("right");
  });

  it("WASD up/down/left mutates state", () => {
    room.handleMove(client, { dir: "down" });
    expect(room.state.players.get(client.sessionId)!.y).toBe(9);
    room.handleMove(client, { dir: "up" });
    expect(room.state.players.get(client.sessionId)!.y).toBe(8);
    room.handleMove(client, { dir: "left" });
    expect(room.state.players.get(client.sessionId)!.x).toBe(3);
  });

  it("rejects movement into a wall", () => {
    const p = room.state.players.get(client.sessionId)!;
    p.x = 1;
    p.y = 1;
    room.handleMove(client, { dir: "up" }); // wall row 0
    expect(p.x).toBe(1);
    expect(p.y).toBe(1);
  });

  it("ignores invalid direction silently", () => {
    const p = room.state.players.get(client.sessionId)!;
    // @ts-expect-error
    room.handleMove(client, { dir: "diagonal" });
    expect(p.x).toBe(4);
    expect(p.y).toBe(8);
  });
});
