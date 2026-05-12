import type { CasinoResultEvent } from "@tu/shared";
import type { Room } from "colyseus.js";
import { term } from "./term.js";

/** Run a small interactive blackjack betting loop in-place. Returns when
 *  the player presses ESC to leave. */
export async function runCasinoUI(room: Room): Promise<void> {
  term.clear();
  term.bold.magenta("◇ CASINO — Blackjack ◇\n");
  term.gray("Press B to bet, ESC to leave.\n\n");

  const onResult = (msg: CasinoResultEvent) => {
    term(
      `Player ${msg.player.join(",")} (=${total(msg.player)})  |  Dealer ${msg.dealer.join(
        ",",
      )} (=${total(msg.dealer)})\n`,
    );
    const tag =
      msg.outcome === "win"
        ? term.green
        : msg.outcome === "lose"
          ? term.red
          : term.yellow;
    tag(
      `→ ${msg.outcome.toUpperCase()}  Δ${msg.delta >= 0 ? "+" : ""}${msg.delta}  Balance: ${msg.balance}\n\n`,
    );
  };
  const onError = (msg: { error: string }) => {
    term.red(`✘ ${msg.error}\n`);
  };
  room.onMessage("casino:result", onResult);
  room.onMessage("error", onError);

  return new Promise<void>((resolve) => {
    const handler = async (key: string) => {
      if (key === "ESCAPE") {
        term.removeListener("key", handler);
        resolve();
        return;
      }
      if (key === "b" || key === "B") {
        term("Bet amount: ");
        await new Promise<void>((res) =>
          term.inputField(
            {},
            (_e: Error | undefined, raw: string | undefined) => {
              term("\n");
              const amount = Number((raw ?? "").trim());
              if (Number.isFinite(amount) && amount > 0) {
                room.send("bet", { amount });
              } else {
                term.red("invalid amount\n");
              }
              res();
            },
          ),
        );
      }
    };
    term.on("key", handler);
  });
}

function total(cards: number[]): number {
  let t = cards.reduce((s, c) => s + (c === 1 ? 11 : c >= 10 ? 10 : c), 0);
  let aces = cards.filter((c) => c === 1).length;
  while (t > 21 && aces > 0) {
    t -= 10;
    aces -= 1;
  }
  return t;
}
