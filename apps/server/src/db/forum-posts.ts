import { desc, eq } from "drizzle-orm";
import { randomUUID } from "node:crypto";
import { db } from "./client.js";
import { forumPosts, users } from "./schema.js";

export interface ForumPostSummary {
  id: string;
  title: string;
  body: string;
  author: string;
  createdAt: Date;
}

export async function listForumPosts(limit = 50): Promise<ForumPostSummary[]> {
  return db
    .select({
      id: forumPosts.id,
      title: forumPosts.title,
      body: forumPosts.body,
      author: users.username,
      createdAt: forumPosts.createdAt,
    })
    .from(forumPosts)
    .innerJoin(users, eq(forumPosts.authorId, users.id))
    .orderBy(desc(forumPosts.createdAt))
    .limit(limit);
}

export async function createForumPost(args: {
  authorId: string;
  title: string;
  body: string;
}): Promise<{ id: string }> {
  const [post] = await db
    .insert(forumPosts)
    .values({
      id: randomUUID(),
      authorId: args.authorId,
      title: args.title,
      body: args.body,
    })
    .returning({ id: forumPosts.id });
  return post;
}