import { describe, expect, it } from "vitest";
import {
  cardValue,
  handTotal,
  playRound,
  settle,
} from "../src/games/blackjack.js";

describe("blackjack math", () => {
  it("aces count as 11 when safe, 1 when bust", () => {
    expect(handTotal([1, 5])).toBe(16);
    expect(handTotal([1, 9, 5])).toBe(15); // ace=1
    expect(handTotal([1, 1, 9])).toBe(21); // one ace=11, one=1
  });

  it("faces count as 10", () => {
    expect(cardValue(11)).toBe(10);
    expect(cardValue(13)).toBe(10);
    expect(handTotal([13, 10])).toBe(20);
  });

  it("settles win/lose/push", () => {
    expect(settle(10, "win")).toBe(10);
    expect(settle(10, "lose")).toBe(-10);
    expect(settle(10, "push")).toBe(0);
  });

  it("plays a deterministic round (player 20, dealer 19)", () => {
    const seq = [10, 10, 9, 10]; // P:10,10=20; D:9,10=19 → win
    let i = 0;
    const draw = () => seq[i++]!;
    const r = playRound(draw);
    expect(r.playerTotal).toBe(20);
    expect(r.dealerTotal).toBe(19);
    expect(r.outcome).toBe("win");
  });

  it("plays a deterministic push (both 20)", () => {
    const seq = [10, 10, 10, 10];
    let i = 0;
    const r = playRound(() => seq[i++]!);
    expect(r.outcome).toBe("push");
  });

  it("plays a deterministic bust (player>21)", () => {
    // player draws to 22: 10, 7, 5 → 22. Initial: 10,7=17 hits? handTotal<17? 17 not <17 → no hit
    // So force initial 10,6=16, then 10 → 26.
    const seq = [10, 6, 9, 10, 10];
    let i = 0;
    const r = playRound(() => seq[i++]!);
    expect(r.playerTotal).toBeGreaterThan(21);
    expect(r.outcome).toBe("lose");
  });
});
