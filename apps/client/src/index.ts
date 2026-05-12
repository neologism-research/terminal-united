import { parseLdtkMap, type ParsedMap } from "@tu/map-engine";
import { ROOM_NAMES, type DirInput, type RoomState } from "@tu/shared";
import { Client, Room } from "colyseus.js";
import { runBbsUI } from "./bbs-ui.js";
import { runCasinoUI } from "./casino-ui.js";
import { SERVER_HTTP, SERVER_WS } from "./config.js";
import { loginFlow } from "./login-prompt.js";
import { HudRenderer } from "./renderer.js";
import { term } from "./term.js";

const KEY_TO_DIR: Record<string, DirInput | undefined> = {
  w: "up",
  s: "down",
  a: "left",
  d: "right",
  UP: "up",
  DOWN: "down",
  LEFT: "left",
  RIGHT: "right",
};

async function fetchHubMap(): Promise<ParsedMap> {
  const res = await fetch(`${SERVER_HTTP}/health`);
  if (!res.ok) throw new Error("server unreachable");
  // The client ships an identical map JSON (no remote fetch needed).
  const { readFileSync } = await import("node:fs");
  const { fileURLToPath } = await import("node:url");
  const { dirname, resolve } = await import("node:path");
  const here = dirname(fileURLToPath(import.meta.url));
  const path = resolve(here, "../../../packages/map-engine/maps/hub.json");
  return parseLdtkMap(JSON.parse(readFileSync(path, "utf-8")));
}

async function main() {
  process.on("SIGINT", shutdown);
  process.on("SIGTERM", shutdown);

  const auth = await loginFlow();
  const hubMap = await fetchHubMap();
  const client = new Client(SERVER_WS);
  const renderer = new HudRenderer();
  renderer.setMap(hubMap);
  let mySessionId: string | null = null;
  let canRender = false;

  let room = await client.joinOrCreate<RoomState>(ROOM_NAMES.hub, {
    token: auth.token,
  });
  bindRoom(room);
  await waitForInitialState(room);

  term.fullscreen(true);
  term.grabInput({ mouse: undefined });
  canRender = true;

  function bindRoom(r: Room<RoomState>) {
    mySessionId = r.sessionId;
    r.onStateChange(() => {
      if (canRender) renderer.draw(r.state, mySessionId);
    });
    r.onMessage("welcome", (_w) => {
      renderer.setStatus(
        "WASD/arrows to move. T to chat. C casino. B BBS. Q quit.",
      );
    });
    r.onMessage("transition", async (msg: { room: string }) => {
      if (msg.room === ROOM_NAMES.casino) {
        await transitionToCasino();
      }
    });
  }

  async function transitionToCasino() {
    canRender = false;
    term.removeListener("key", onKey);
    await room.leave();
    room = await client.joinOrCreate<RoomState>(ROOM_NAMES.casino, {
      token: auth.token,
    });
    room.onMessage("welcome", () => {});
    await waitForInitialState(room);
    await runCasinoUI(room);
    await room.leave();
    room = await client.joinOrCreate<RoomState>(ROOM_NAMES.hub, {
      token: auth.token,
    });
    bindRoom(room);
    await waitForInitialState(room);
    term.fullscreen(true);
    term.on("key", onKey);
    canRender = true;
    renderer.draw(room.state, room.sessionId);
  }

  async function openBbs() {
    term.removeListener("key", onKey);
    await runBbsUI(auth.token);
    term.fullscreen(true);
    term.on("key", onKey);
    renderer.draw(room.state, room.sessionId);
  }

  async function openChat() {
    term.removeListener("key", onKey);
    term.moveTo(1, term.height);
    term.bgBlue.white(" chat > ");
    await new Promise<void>((resolve) =>
      term.inputField({}, (_e: Error | undefined, raw: string | undefined) => {
        const text = (raw ?? "").trim();
        if (text) room.send("chat", { text });
        term.on("key", onKey);
        resolve();
      }),
    );
    renderer.draw(room.state, room.sessionId);
  }

  function onKey(name: string) {
    if (name === "CTRL_C" || name === "q" || name === "Q") {
      shutdown();
      return;
    }
    if (name === "t" || name === "T") {
      void openChat();
      return;
    }
    if (name === "b" || name === "B") {
      void openBbs();
      return;
    }
    const dir = KEY_TO_DIR[name];
    if (dir) room.send("move", { dir });
  }

  bindRoom(room);
  term.on("key", onKey);
  term.on("resize", () => {
    renderer.resize();
    renderer.draw(room.state, room.sessionId);
  });
  renderer.draw(room.state, room.sessionId);
}

function hasInitialState(state: RoomState | undefined): boolean {
  return Boolean(state?.players && state?.chat);
}

async function waitForInitialState(room: Room<RoomState>): Promise<void> {
  if (hasInitialState(room.state)) return;
  await new Promise<void>((resolve) => {
    room.onStateChange.once(() => resolve());
  });
  if (!hasInitialState(room.state)) {
    throw new Error("room state was not initialized by the server");
  }
}

function shutdown() {
  restoreTerminal();
  term("\nGoodbye.\n");
  process.exit(0);
}

function restoreTerminal() {
  term.grabInput(false);
  term.fullscreen(false);
}

main().catch((e) => {
  restoreTerminal();
  term.red(`\n[fatal] ${(e as Error).message}\n`);
  process.exit(1);
});
