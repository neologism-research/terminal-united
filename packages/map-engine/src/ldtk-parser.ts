import { TILE, type TileCode } from "@tu/shared";
import {
  COLLISION_GLYPH_TO_TILE,
  type LdtkLikeMap,
  type ParsedMap,
} from "./types.js";

/** Parse an LDtk-like JSON map into a `ParsedMap` usable by client + server. */
export function parseLdtkMap(raw: unknown): ParsedMap {
  const m = assertLdtkLike(raw);
  const { width, height, layers } = m;

  if (layers.visuals.length !== height) {
    throw new Error(
      `visuals row count ${layers.visuals.length} != height ${height}`,
    );
  }
  if (layers.collisions.length !== height) {
    throw new Error(
      `collisions row count ${layers.collisions.length} != height ${height}`,
    );
  }

  const visuals: string[][] = [];
  const collisions: TileCode[][] = [];

  for (let y = 0; y < height; y++) {
    const vRow = layers.visuals[y];
    const cRow = layers.collisions[y];
    if (vRow.length !== width) {
      throw new Error(`visuals row ${y} len ${vRow.length} != width ${width}`);
    }
    if (cRow.length !== width) {
      throw new Error(
        `collisions row ${y} len ${cRow.length} != width ${width}`,
      );
    }
    visuals.push([...vRow]);
    const cells: TileCode[] = [];
    for (let x = 0; x < width; x++) {
      const ch = cRow[x];
      const code = COLLISION_GLYPH_TO_TILE[ch];
      if (code === undefined) {
        throw new Error(`unknown collision glyph "${ch}" at (${x},${y})`);
      }
      cells.push(code);
    }
    collisions.push(cells);
  }

  return {
    width,
    height,
    visuals,
    collisions,
    spawn: m.spawn ?? { x: 1, y: 1 },
    portals: {
      [TILE.portalCasino]: m.portals?.casino ?? "casino",
      [TILE.portalClubhouse]: m.portals?.clubhouse ?? "clubhouse",
    },
  };
}

function assertLdtkLike(v: unknown): LdtkLikeMap {
  if (!v || typeof v !== "object") {
    throw new Error("map: not an object");
  }
  const o = v as Record<string, unknown>;
  if (typeof o.width !== "number" || typeof o.height !== "number") {
    throw new Error("map: width/height must be numbers");
  }
  const layers = o.layers as Record<string, unknown> | undefined;
  if (
    !layers ||
    !Array.isArray((layers as Record<string, unknown>).visuals) ||
    !Array.isArray((layers as Record<string, unknown>).collisions)
  ) {
    throw new Error("map: layers.visuals and layers.collisions are required");
  }
  return v as LdtkLikeMap;
}
