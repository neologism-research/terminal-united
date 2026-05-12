import { randomBytes } from "node:crypto";
import { createUser, findUserByUsername } from "./db/users.js";
import { hashPassword, verifyPassword } from "./passwords.js";

/** Issued auth tokens (in-memory). Tokens are bound to user.id. */
const tokens = new Map<string, string>();

export interface AuthedUser {
  id: string;
  username: string;
  walletBalance: number;
}

export function isValidUsername(s: string): boolean {
  return /^[a-zA-Z0-9_]{3,20}$/.test(s);
}

export function isValidPassword(s: string): boolean {
  return typeof s === "string" && s.length >= 6 && s.length <= 200;
}

export async function registerUser(
  username: string,
  password: string,
): Promise<AuthedUser> {
  if (!isValidUsername(username)) throw new Error("invalid username");
  if (!isValidPassword(password)) throw new Error("invalid password");
  const existing = await findUserByUsername(username);
  if (existing) throw new Error("username taken");
  const user = await createUser({
    username,
    passwordHash: hashPassword(password),
  });
  return {
    id: user.id,
    username: user.username,
    walletBalance: user.walletBalance,
  };
}

export async function loginUser(
  username: string,
  password: string,
): Promise<AuthedUser> {
  const user = await findUserByUsername(username);
  if (!user || !verifyPassword(password, user.passwordHash)) {
    throw new Error("invalid credentials");
  }
  return {
    id: user.id,
    username: user.username,
    walletBalance: user.walletBalance,
  };
}

export function issueToken(userId: string): string {
  const tok = randomBytes(24).toString("hex");
  tokens.set(tok, userId);
  return tok;
}

export function resolveToken(token: string): string | null {
  return tokens.get(token) ?? null;
}

export function revokeToken(token: string): void {
  tokens.delete(token);
}
