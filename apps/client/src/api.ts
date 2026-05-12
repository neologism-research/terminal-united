import { SERVER_HTTP } from "./config.js";

export interface AuthResult {
  token: string;
  user: { id: string; username: string; walletBalance: number };
}

async function call(path: string, body: unknown): Promise<AuthResult> {
  const res = await fetch(`${SERVER_HTTP}${path}`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
  const data = (await res.json()) as { error?: string } & AuthResult;
  if (!res.ok) throw new Error(data.error ?? `HTTP ${res.status}`);
  return data;
}

export const login = (username: string, password: string) =>
  call("/auth/login", { username, password });

export const register = (username: string, password: string) =>
  call("/auth/register", { username, password });

export interface BbsPost {
  id: string;
  title: string;
  body: string;
  author: string;
  at: number;
}

export async function listPosts(): Promise<BbsPost[]> {
  const res = await fetch(`${SERVER_HTTP}/bbs`);
  const data = (await res.json()) as { posts: BbsPost[] };
  return data.posts;
}

export async function createPost(
  token: string,
  title: string,
  body: string,
): Promise<void> {
  const res = await fetch(`${SERVER_HTTP}/bbs`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ title, body }),
  });
  if (!res.ok) {
    const data = (await res.json()) as { error?: string };
    throw new Error(data.error ?? `HTTP ${res.status}`);
  }
}
