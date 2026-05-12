---
title: Introduction
description: What Terminal United is and how this codebase is structured.
---

**Terminal United** is a low-stakes, high-vibe terminal hangout for developers.
Players load into a centralized hub map, walk around with WASD, chat in a
sidebar, browse a 90s-style BBS, and step into the casino to play blackjack.

This repository contains the playable Phase 1–4 scope from the project
roadmap:

- **Phase 1 — Engine & Netcode Foundation.** pnpm workspace, Vitest, a
  Colyseus `HubRoom`, and a `terminal-kit` client that renders the player
  token and responds to WASD input.
- **Phase 2 — Map Parsing & Layout.** A normalized LDtk-like JSON map for the
  hub, a parser in `@tu/map-engine`, server-authoritative collision, and a
  chat sidebar.
- **Phase 3 — Persistence & Player Accounts.** Drizzle + Postgres, a CLI login
  prompt, and a wallet balance loaded into the room state on join.
- **Phase 4 — Economy & Asynchronous Social.** A `CasinoRoom` reachable via a
  hub portal tile, server-side blackjack with wallet-deductions, and an HTTP
  Clubhouse BBS backed by the `ForumPost` model.

Phases 5 (Dungeons & PvE) and 6 (Monetization & Polish) are out of scope for
this build and are summarized in [Future: Phases 5–6](/phases/future/).
