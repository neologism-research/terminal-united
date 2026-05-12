import { ArraySchema, MapSchema, Schema, schema } from "@colyseus/schema";

/** Networked player state. */
export const PlayerState = schema({
  username: "string",
  x: "number",
  y: "number",
  facing: "string",
  balance: "number",
});
export type PlayerState = InstanceType<typeof PlayerState>;

/** A single chat line as it appears in room history. */
export const ChatMessageState = schema({
  from: "string",
  text: "string",
  at: "number",
});
export type ChatMessageState = InstanceType<typeof ChatMessageState>;

/** Shared room state used by Hub + Casino rooms. */
export const RoomState = schema({
  roomName: "string",
  players: { map: PlayerState },
  chat: [ChatMessageState],
});
export type RoomState = InstanceType<typeof RoomState>;

export { ArraySchema, MapSchema, Schema };
