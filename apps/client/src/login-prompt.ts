import { login, register, type AuthResult } from "./api.js";
import { term } from "./term.js";

function ask(prompt: string, mask = false): Promise<string> {
  return new Promise((resolve, reject) => {
    term(prompt);
    term.inputField(
      { echo: !mask },
      (err: Error | undefined, input: string | undefined) => {
        term("\n");
        if (err) return reject(err);
        resolve((input ?? "").trim());
      },
    );
  });
}

export async function loginFlow(): Promise<AuthResult> {
  term.bold.cyan("Terminal United\n");
  term.gray("─".repeat(40) + "\n");
  // eslint-disable-next-line no-constant-condition
  while (true) {
    term("[L]ogin or [R]egister? ");
    const mode = (await ask("")).toLowerCase();
    const action = mode.startsWith("r") ? register : login;
    const username = await ask("Username: ");
    const password = await ask("Password: ", true);
    try {
      const res = await action(username, password);
      term.green(
        `Welcome, ${res.user.username}! Balance: ${res.user.walletBalance} Credits\n`,
      );
      return res;
    } catch (e) {
      term.red(`✘ ${(e as Error).message}\n`);
    }
  }
}
