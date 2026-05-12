import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { isWalkable, portalAt, stepOrThrow } from "../src/collision.js";
import { parseLdtkMap } from "../src/ldtk-parser.js";

const here = dirname(fileURLToPath(import.meta.url));
const hubRaw = JSON.parse(
  readFileSync(resolve(here, "../maps/hub.json"), "utf-8"),
);

describe("parseLdtkMap", () => {
  it("parses hub.json width/height", () => {
    const m = parseLdtkMap(hubRaw);
    expect(m.width).toBe(40);
    expect(m.height).toBe(16);
    expect(m.collisions.length).toBe(16);
    expect(m.collisions[0].length).toBe(40);
  });

  it("throws on width mismatch", () => {
    expect(() =>
      parseLdtkMap({
        width: 3,
        height: 1,
        layers: { visuals: ["abcd"], collisions: ["...."] },
      }),
    ).toThrow();
  });

  it("throws on unknown collision glyph", () => {
    expect(() =>
      parseLdtkMap({
        width: 3,
        height: 1,
        layers: { visuals: ["abc"], collisions: ["?.#"] },
      }),
    ).toThrow(/unknown collision glyph/);
  });
});

describe("collision", () => {
  const map = parseLdtkMap(hubRaw);

  it("treats outer border as wall", () => {
    expect(isWalkable(map, 0, 0)).toBe(false);
    expect(isWalkable(map, 4, 8)).toBe(true);
  });

  it("rejects stepping into wall", () => {
    expect(() => stepOrThrow(map, 1, 1, "up")).toThrow(/move rejected/);
  });

  it("walks into empty tiles", () => {
    const n = stepOrThrow(map, 4, 8, "right");
    expect(n).toEqual({ x: 5, y: 8 });
  });

  it("detects casino portal", () => {
    expect(portalAt(map, 31, 8)).toBe("casino");
  });

  it("detects clubhouse portal", () => {
    expect(portalAt(map, 7, 6)).toBe("clubhouse");
  });
});
