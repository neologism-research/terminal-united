import { Router, type Router as ExpressRouter } from "express";
import { resolveToken } from "../auth.js";
import { createForumPost, listForumPosts } from "../db/forum-posts.js";

export const bbsRouter: ExpressRouter = Router();

/** GET /bbs — list latest 50 posts (newest first). */
bbsRouter.get("/", async (_req, res) => {
  const posts = await listForumPosts(50);
  res.json({
    posts: posts.map((p) => ({
      id: p.id,
      title: p.title,
      body: p.body,
      author: p.author,
      at: p.createdAt.getTime(),
    })),
  });
});

/** POST /bbs — create a new BBS post. Requires bearer token. */
bbsRouter.post("/", async (req, res) => {
  const auth = req.header("authorization") ?? "";
  const token = auth.replace(/^Bearer\s+/i, "");
  const userId = resolveToken(token);
  if (!userId) return res.status(401).json({ error: "unauthenticated" });

  const { title, body } = req.body ?? {};
  if (
    typeof title !== "string" ||
    title.length < 1 ||
    title.length > 100 ||
    typeof body !== "string" ||
    body.length < 1 ||
    body.length > 4000
  ) {
    return res.status(400).json({ error: "invalid post" });
  }
  const post = await createForumPost({ authorId: userId, title, body });
  res.json({ id: post.id });
});
