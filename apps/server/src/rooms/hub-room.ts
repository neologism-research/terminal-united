import type { Client } from "@colyseus/core";
import { applyDir, isWalkable, portalAt, type ParsedMap } from "@tu/map-engine";
import {
  ROOM_NAMES,
  RoomState,
  type MoveMessage,
  type WelcomeEvent,
} from "@tu/shared";
import { getHubMap } from "../maps.js";
import { BaseRoom } from "./base-room.js";

interface AuthedUser {
  id: string;
  username: string;
  walletBalance: number;
}

export class HubRoom extends BaseRoom {
  private map!: ParsedMap;

  onCreate() {
    this.setState(new RoomState());
    this.state.roomName = ROOM_NAMES.hub;
    this.map = getHubMap();
    this.registerChatHandler();

    this.onMessage<MoveMessage>("move", (client, msg) => {
      this.handleMove(client, msg);
    });
  }

  async onJoin(client: Client, _options: unknown, user: AuthedUser) {
    this.userIds.set(client.sessionId, user.id);
    const spawn = this.map.spawn;
    this.addPlayer(client, user.username, user.walletBalance, spawn.x, spawn.y);
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
  handleMove(client: Client, msg: MoveMessage) {
    const player = this.state.players.get(client.sessionId);
    if (!player) return;
    const dir = msg?.dir;
    if (dir !== "up" && dir !== "down" && dir !== "left" && dir !== "right") {
      return;
    }
    const next = applyDir(player.x, player.y, dir);
    if (!isWalkable(this.map, next.x, next.y)) return; // silently reject

    player.x = next.x;
    player.y = next.y;
    player.facing = dir;

    const portal = portalAt(this.map, next.x, next.y);
    if (portal === "casino") {
      client.send("transition", { room: ROOM_NAMES.casino });
    }
    // clubhouse is handled via HTTP BBS, no transition.
  }
}
