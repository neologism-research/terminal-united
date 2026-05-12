import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

/** Loads `.env` from the server package root into `process.env` without
 *  external dependencies. Existing variables are not overridden. */
export function loadEnv(file = ".env"): void {
  const path = resolve(process.cwd(), file);
  if (!existsSync(path)) return;
  const text = readFileSync(path, "utf-8");
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const eq = line.indexOf("=");
    if (eq <= 0) continue;
    const key = line.slice(0, eq).trim();
    let val = line.slice(eq + 1).trim();
    if (
      (val.startsWith('"') && val.endsWith('"')) ||
      (val.startsWith("'") && val.endsWith("'"))
    ) {
      val = val.slice(1, -1);
    }
    if (process.env[key] === undefined) process.env[key] = val;
  }
}
