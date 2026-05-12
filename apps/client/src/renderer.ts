import type { ParsedMap } from "@tu/map-engine";
import type { ChatMessageState, PlayerState, RoomState } from "@tu/shared";
import { ScreenBuffer, term } from "./term.js";

const SIDEBAR_W = 32;

/** Renders the map + chat sidebar to a single terminal-kit screen buffer. */
export class HudRenderer {
  private screen: InstanceType<typeof ScreenBuffer>;
  private map: ParsedMap | null = null;
  private statusLine = "";

  constructor() {
    this.screen = new ScreenBuffer({
      dst: term,
      width: term.width,
      height: term.height,
    });
  }

  setMap(map: ParsedMap | null) {
    this.map = map;
  }

  setStatus(line: string) {
    this.statusLine = line;
  }

  resize() {
    this.screen.resize({ x: 0, y: 0, width: term.width, height: term.height });
  }

  draw(state: RoomState, mySessionId: string | null) {
    if (!state.players || !state.chat) return;
    this.screen.fill({ attr: { bgDefaultColor: true }, char: " " });
    this.drawMap(state);
    this.drawSidebar(state, mySessionId);
    this.drawStatus();
    this.screen.draw({ delta: true });
  }

  private drawMap(state: RoomState) {
    if (!this.map) return;
    const w = term.width - SIDEBAR_W - 1;
    const h = term.height - 2;
    for (let y = 0; y < Math.min(this.map.height, h); y++) {
      const row = this.map.visuals[y].join("");
      this.screen.put(
        { x: 0, y, attr: { color: "gray" }, dx: 1, dy: 0, wrap: false },
        row.slice(0, w),
      );
    }
    // Players overlay
    state.players.forEach((p: PlayerState, sid: string) => {
      const ch = sid === state.players.keys().next().value ? "@" : "&";
      if (p.x >= 0 && p.x < w && p.y >= 0 && p.y < h) {
        this.screen.put(
          {
            x: p.x,
            y: p.y,
            attr: { color: "brightGreen", bold: true },
            dx: 1,
            dy: 0,
            wrap: false,
          },
          ch,
        );
      }
    });
    // Vertical divider
    for (let y = 0; y < h; y++) {
      this.screen.put(
        {
          x: term.width - SIDEBAR_W - 1,
          y,
          attr: { color: "blue" },
          dx: 1,
          dy: 0,
          wrap: false,
        },
        "│",
      );
    }
  }

  private drawSidebar(state: RoomState, mySessionId: string | null) {
    const x0 = term.width - SIDEBAR_W;
    let y = 0;
    this.screen.put(
      {
        x: x0,
        y,
        attr: { color: "cyan", bold: true },
        dx: 1,
        dy: 0,
        wrap: false,
      },
      `# ${state.roomName.toUpperCase()}`,
    );
    y += 1;

    const me = mySessionId ? state.players.get(mySessionId) : null;
    if (me) {
      this.screen.put(
        {
          x: x0,
          y,
          attr: { color: "yellow" },
          dx: 1,
          dy: 0,
          wrap: false,
        },
        `${me.username}  ⓒ${me.balance}`,
      );
      y += 1;
    }

    this.screen.put(
      { x: x0, y, attr: { color: "gray" }, dx: 1, dy: 0, wrap: false },
      "─".repeat(SIDEBAR_W),
    );
    y += 1;
    this.screen.put(
      {
        x: x0,
        y,
        attr: { color: "white", bold: true },
        dx: 1,
        dy: 0,
        wrap: false,
      },
      "PLAYERS",
    );
    y += 1;
    state.players.forEach((p: PlayerState) => {
      this.screen.put(
        { x: x0, y, attr: { color: "white" }, dx: 1, dy: 0, wrap: false },
        ` ${p.username}`.slice(0, SIDEBAR_W),
      );
      y += 1;
    });
    y += 1;

    this.screen.put(
      { x: x0, y, attr: { color: "gray" }, dx: 1, dy: 0, wrap: false },
      "─".repeat(SIDEBAR_W),
    );
    y += 1;
    this.screen.put(
      {
        x: x0,
        y,
        attr: { color: "white", bold: true },
        dx: 1,
        dy: 0,
        wrap: false,
      },
      "GLOBAL CHAT  (press t)",
    );
    y += 1;
    const chatLines = sliceChat(state.chat, term.height - y - 2);
    for (const line of chatLines) {
      this.screen.put(
        { x: x0, y, attr: { color: "white" }, dx: 1, dy: 0, wrap: false },
        line.slice(0, SIDEBAR_W),
      );
      y += 1;
    }
  }

  private drawStatus() {
    const y = term.height - 1;
    for (let x = 0; x < term.width; x++) {
      this.screen.put(
        {
          x,
          y,
          attr: { bgColor: "blue", color: "white" },
          dx: 1,
          dy: 0,
          wrap: false,
        },
        " ",
      );
    }
    this.screen.put(
      {
        x: 1,
        y,
        attr: { bgColor: "blue", color: "white" },
        dx: 1,
        dy: 0,
        wrap: false,
      },
      this.statusLine.slice(0, term.width - 2),
    );
  }
}

function sliceChat(chat: ArrayLike<ChatMessageState>, max: number): string[] {
  const arr: ChatMessageState[] = [];
  for (let i = 0; i < chat.length; i++) arr.push(chat[i] as ChatMessageState);
  const tail = arr.slice(-max);
  return tail.map((c) => `${c.from}: ${c.text}`);
}
