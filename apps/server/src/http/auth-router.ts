import { Router, type Router as ExpressRouter } from "express";
import { issueToken, loginUser, registerUser } from "../auth.js";

export const authRouter: ExpressRouter = Router();

authRouter.post("/register", async (req, res) => {
  try {
    const { username, password } = req.body ?? {};
    const user = await registerUser(username, password);
    const token = issueToken(user.id);
    res.json({ token, user });
  } catch (e) {
    res.status(400).json({ error: (e as Error).message });
  }
});

authRouter.post("/login", async (req, res) => {
  try {
    const { username, password } = req.body ?? {};
    const user = await loginUser(username, password);
    const token = issueToken(user.id);
    res.json({ token, user });
  } catch (e) {
    res.status(401).json({ error: (e as Error).message });
  }
});
