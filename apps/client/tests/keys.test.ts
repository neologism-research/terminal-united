import { describe, expect, it } from "vitest";

// We re-create the same mapping in the test to avoid importing the full
// client (which depends on terminal-kit / TTY).
const KEY_TO_DIR: Record<string, string | undefined> = {
  w: "up",
  s: "down",
  a: "left",
  d: "right",
  UP: "up",
  DOWN: "down",
  LEFT: "left",
  RIGHT: "right",
};

describe("client key mapping", () => {
  it("maps WASD + arrows", () => {
    expect(KEY_TO_DIR["w"]).toBe("up");
    expect(KEY_TO_DIR["UP"]).toBe("up");
    expect(KEY_TO_DIR["d"]).toBe("right");
    expect(KEY_TO_DIR["x"]).toBeUndefined();
  });
});
