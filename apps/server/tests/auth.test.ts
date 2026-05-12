import { beforeEach, describe, expect, it, vi } from "vitest";

type User = {
  id: string;
  username: string;
  passwordHash: string;
  walletBalance: number;
};
const users = new Map<string, User>();

vi.mock("../src/db/users.js", () => {
  return {
    findUserByUsername: async (username: string) =>
      [...users.values()].find((u) => u.username === username) ?? null,
    createUser: async ({
      username,
      passwordHash,
    }: {
      username: string;
      passwordHash: string;
    }) => {
      const id = `u${users.size + 1}`;
      const user: User = {
        id,
        username,
        passwordHash,
        walletBalance: 500,
      };
      users.set(id, user);
      return user;
    },
  };
});

import { DEFAULT_WALLET_BALANCE } from "@tu/shared";
import {
  issueToken,
  isValidPassword,
  isValidUsername,
  loginUser,
  registerUser,
  resolveToken,
} from "../src/auth.js";

beforeEach(() => users.clear());

describe("auth + wallet", () => {
  it("validates input shape", () => {
    expect(isValidUsername("ab")).toBe(false);
    expect(isValidUsername("good_one")).toBe(true);
    expect(isValidPassword("123")).toBe(false);
    expect(isValidPassword("longenough")).toBe(true);
  });

  it("creates a user with default wallet balance", async () => {
    const u = await registerUser("alice", "password");
    expect(u.username).toBe("alice");
    expect(u.walletBalance).toBe(DEFAULT_WALLET_BALANCE);
  });

  it("rejects duplicate username", async () => {
    await registerUser("bob", "password");
    await expect(registerUser("bob", "password")).rejects.toThrow(/taken/);
  });

  it("logs in with correct password and rejects bad password", async () => {
    await registerUser("carol", "password");
    const ok = await loginUser("carol", "password");
    expect(ok.username).toBe("carol");
    await expect(loginUser("carol", "wrong")).rejects.toThrow();
  });

  it("issues and resolves tokens", async () => {
    const u = await registerUser("dave", "password");
    const tok = issueToken(u.id);
    expect(resolveToken(tok)).toBe(u.id);
    expect(resolveToken("nope")).toBe(null);
  });
});
