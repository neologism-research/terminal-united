import { Server } from "@colyseus/core";
import { WebSocketTransport } from "@colyseus/ws-transport";
import { ROOM_NAMES } from "@tu/shared";
import express from "express";
import { createServer } from "node:http";
import { loadEnv } from "./env.js";
import { authRouter } from "./http/auth-router.js";
import { bbsRouter } from "./http/bbs-router.js";
import { CasinoRoom } from "./rooms/casino-room.js";
import { HubRoom } from "./rooms/hub-room.js";
loadEnv();

const PORT = Number(process.env.PORT ?? 2567);

const app = express();
app.use(express.json());
app.use("/auth", authRouter);
app.use("/bbs", bbsRouter);
app.get("/health", (_req, res) => res.json({ ok: true }));

const httpServer = createServer(app);

const gameServer = new Server({
  transport: new WebSocketTransport({ server: httpServer }),
});

gameServer.define(ROOM_NAMES.hub, HubRoom);
gameServer.define(ROOM_NAMES.casino, CasinoRoom);

gameServer.listen(PORT).then(() => {
  // eslint-disable-next-line no-console
  console.log(`[terminal-united] server listening on :${PORT}`);
});
