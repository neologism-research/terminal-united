import termkitDefault from "terminal-kit";

/** Re-exports the shared terminal-kit `term` instance. */
const tk =
  (termkitDefault as unknown as { default?: typeof termkitDefault }).default ??
  termkitDefault;
export const term = tk.terminal;
export const ScreenBuffer = tk.ScreenBuffer;
export type TermType = typeof term;
