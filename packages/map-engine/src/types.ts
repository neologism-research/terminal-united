import { TILE, type TileCode } from "@tu/shared";

/** A parsed map ready for runtime. */
export interface ParsedMap {
  width: number;
  height: number;
  /** Row-major visual glyphs. */
  visuals: string[][];
  /** Row-major collision tile codes (0=empty, 1=wall, 10/11=portals). */
  collisions: TileCode[][];
  /** Optional spawn point. */
  spawn: { x: number; y: number };
  /** Portals indexed by tile code → target room name. */
  portals: Record<number, string>;
}

/** Minimal LDtk-like schema we accept. We don't pull the full LDtk format
 *  to stay lightweight; instead we accept a normalized JSON containing
 *  `visuals` (array of strings, one row per line) and `collisions`
 *  (array of strings, '.' empty, '#' wall, 'C' casino portal, 'B' BBS portal).
 */
export interface LdtkLikeMap {
  width: number;
  height: number;
  spawn?: { x: number; y: number };
  layers: {
    visuals: string[];
    collisions: string[];
  };
  portals?: Record<string, string>;
}

export const COLLISION_GLYPH_TO_TILE: Record<string, TileCode> = {
  ".": TILE.empty,
  " ": TILE.empty,
  "#": TILE.wall,
  C: TILE.portalCasino,
  B: TILE.portalClubhouse,
};
