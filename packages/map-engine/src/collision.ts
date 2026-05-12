import { TILE } from "@tu/shared";
import type { ParsedMap } from "./types.js";

/** Returns true if `(x,y)` is inside bounds and not a wall. */
export function isWalkable(map: ParsedMap, x: number, y: number): boolean {
  if (x < 0 || y < 0 || x >= map.width || y >= map.height) return false;
  return map.collisions[y][x] !== TILE.wall;
}

/** Returns the portal target room name for tile `(x,y)`, or null if none. */
export function portalAt(map: ParsedMap, x: number, y: number): string | null {
  if (x < 0 || y < 0 || x >= map.width || y >= map.height) return null;
  const code = map.collisions[y][x];
  return map.portals[code] ?? null;
}

export type Dir = "up" | "down" | "left" | "right";

export function applyDir(
  x: number,
  y: number,
  dir: Dir,
): { x: number; y: number } {
  switch (dir) {
    case "up":
      return { x, y: y - 1 };
    case "down":
      return { x, y: y + 1 };
    case "left":
      return { x: x - 1, y };
    case "right":
      return { x: x + 1, y };
  }
}

/** Authoritative step. Throws if the move would enter a wall. */
export function stepOrThrow(
  map: ParsedMap,
  x: number,
  y: number,
  dir: Dir,
): { x: number; y: number } {
  const next = applyDir(x, y, dir);
  if (!isWalkable(map, next.x, next.y)) {
    throw new Error(
      `move rejected: (${next.x},${next.y}) is not walkable in ${dir}`,
    );
  }
  return next;
}
