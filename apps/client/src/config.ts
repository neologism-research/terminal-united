/** Centralized client config sourced from env. */

export const SERVER_HTTP =
  process.env.TU_SERVER_HTTP ?? "http://localhost:2567";
export const SERVER_WS = process.env.TU_SERVER_WS ?? "ws://localhost:2567";
