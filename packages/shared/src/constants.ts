/** Shared game constants between client and server. */

export const TICK_RATE = 20; // server ticks per second
export const MAP_TILE_SIZE = 1; // logical tiles, 1 cell each

/** Player movement speed: tiles per input. Movement is grid-based. */
export const PLAYER_STEP = 1;

/** Default starting wallet balance for new users (in Credits). */
export const DEFAULT_WALLET_BALANCE = 500;

/** Identifiers for the Colyseus rooms. */
export const ROOM_NAMES = {
  hub: "hub",
  casino: "casino",
} as const;

export type RoomName = (typeof ROOM_NAMES)[keyof typeof ROOM_NAMES];

/** Tile codes shared by client + server when parsing maps. */
export const TILE = {
  empty: 0,
  wall: 1,
  /** Walking onto a portal triggers a room transition. */
  portalCasino: 10,
  portalClubhouse: 11,
} as const;

export type TileCode = (typeof TILE)[keyof typeof TILE];

/** Limits */
export const CHAT_MAX_LEN = 240;
export const CHAT_HISTORY = 50;

/** Casino limits */
export const MIN_BET = 1;
export const MAX_BET = 500;
