/** Tiny Blackjack engine used by CasinoRoom. Cards are integers 1..13. */

export type Card = number;

export interface BlackjackOutcome {
  /** "win" pays 2x bet, "push" returns 1x bet, "lose" returns 0. */
  outcome: "win" | "lose" | "push";
  player: Card[];
  dealer: Card[];
  playerTotal: number;
  dealerTotal: number;
}

export function cardValue(card: Card): number {
  if (card === 1) return 11;
  if (card >= 10) return 10;
  return card;
}

/** Best total ≤ 21 if possible, else minimum total. */
export function handTotal(cards: Card[]): number {
  let total = cards.reduce((s, c) => s + cardValue(c), 0);
  let aces = cards.filter((c) => c === 1).length;
  while (total > 21 && aces > 0) {
    total -= 10; // count one ace as 1 instead of 11
    aces -= 1;
  }
  return total;
}

export type DrawFn = () => Card;

/** Deal a single round. `draw` returns a fresh card each call. */
export function playRound(draw: DrawFn): BlackjackOutcome {
  const player: Card[] = [draw(), draw()];
  const dealer: Card[] = [draw(), draw()];

  // Player hits until 17+.
  while (handTotal(player) < 17) player.push(draw());

  const pt = handTotal(player);
  if (pt > 21) {
    return finalize(player, dealer, "lose");
  }

  while (handTotal(dealer) < 17) dealer.push(draw());
  const dt = handTotal(dealer);

  let outcome: "win" | "lose" | "push";
  if (dt > 21) outcome = "win";
  else if (pt > dt) outcome = "win";
  else if (pt < dt) outcome = "lose";
  else outcome = "push";

  return finalize(player, dealer, outcome);
}

function finalize(
  player: Card[],
  dealer: Card[],
  outcome: "win" | "lose" | "push",
): BlackjackOutcome {
  return {
    outcome,
    player,
    dealer,
    playerTotal: handTotal(player),
    dealerTotal: handTotal(dealer),
  };
}

/** Settle a bet: returns the delta to apply to wallet (positive on win). */
export function settle(bet: number, outcome: "win" | "lose" | "push"): number {
  if (outcome === "win") return bet;
  if (outcome === "push") return 0;
  return -bet;
}

/** Build a draw function backed by Math.random for runtime use. */
export function randomDraw(): DrawFn {
  return () => 1 + Math.floor(Math.random() * 13);
}
