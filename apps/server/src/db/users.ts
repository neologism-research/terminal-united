import { DEFAULT_WALLET_BALANCE } from "@tu/shared";
import { eq } from "drizzle-orm";
import { randomUUID } from "node:crypto";
import { db } from "./client.js";
import { users, type UserRow } from "./schema.js";

export async function findUserById(id: string): Promise<UserRow | null> {
  const [user] = await db.select().from(users).where(eq(users.id, id)).limit(1);
  return user ?? null;
}

export async function findUserByUsername(
  username: string,
): Promise<UserRow | null> {
  const [user] = await db
    .select()
    .from(users)
    .where(eq(users.username, username))
    .limit(1);
  return user ?? null;
}

export async function createUser(args: {
  username: string;
  passwordHash: string;
}): Promise<UserRow> {
  const [user] = await db
    .insert(users)
    .values({
      id: randomUUID(),
      username: args.username,
      passwordHash: args.passwordHash,
      walletBalance: DEFAULT_WALLET_BALANCE,
    })
    .returning();
  return user;
}

export async function updateUserWalletBalance(
  id: string,
  walletBalance: number,
): Promise<UserRow | null> {
  const [user] = await db
    .update(users)
    .set({ walletBalance })
    .where(eq(users.id, id))
    .returning();
  return user ?? null;
}