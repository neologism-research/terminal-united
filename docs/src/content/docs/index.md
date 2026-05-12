---
title: Terminal United
description: Internal development docs for the Terminal United terminal MMORPG.
template: splash
hero:
  tagline: A casual, social MMORPG that lives inside your terminal.
  actions:
    - text: Get Started
      link: /overview/getting-started/
      icon: right-arrow
      variant: primary
    - text: Architecture
      link: /overview/architecture/
      icon: open-book
---

These docs cover the implementation that ships in this repository: a pnpm
monorepo with a Colyseus authoritative server, a `terminal-kit` client, a
Drizzle + Postgres persistence layer, a casino room running blackjack, and an
asynchronous Clubhouse BBS.

Phases 1 through 4 from the roadmap are implemented and covered by tests.
Phases 5 (PvE Dungeons) and 6 (Monetization & Polish) are intentionally left
as scaffolding work for a future iteration.
