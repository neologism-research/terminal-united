/** Client → Server and Server → Client message contracts. */

/** Client → Server */
export type DirInput = "up" | "down" | "left" | "right";

export interface MoveMessage {
  dir: DirInput;
}

export interface ChatSendMessage {
  text: string;
}

export interface BetMessage {
  /** Wager in Credits. */
  amount: number;
}

export interface BbsPostMessage {
  title: string;
  body: string;
}

/** Server → Client */
export interface ChatEvent {
  /** Display name. */
  from: string;
  text: string;
  /** Unix ms. */
  at: number;
  /** "global" or "system". */
  channel: "global" | "system";
}

export interface CasinoResultEvent {
  outcome: "win" | "lose" | "push";
  delta: number;
  /** Resulting wallet balance after settlement. */
  balance: number;
  /** Drawn cards (player), dealer in `dealer`. Blackjack-only. */
  player: number[];
  dealer: number[];
}

/** Sent to client immediately after join with private info. */
export interface WelcomeEvent {
  sessionId: string;
  username: string;
  balance: number;
}
