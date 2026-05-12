import { randomBytes, scryptSync, timingSafeEqual } from "node:crypto";

const KEYLEN = 64;

/** Hash a plaintext password with scrypt + random salt. Returns
 *  `scrypt$<salt-hex>$<hash-hex>` for storage. */
export function hashPassword(password: string): string {
  const salt = randomBytes(16);
  const hash = scryptSync(password, salt, KEYLEN);
  return `scrypt$${salt.toString("hex")}$${hash.toString("hex")}`;
}

/** Verify a plaintext password against a stored hash. */
export function verifyPassword(password: string, stored: string): boolean {
  const parts = stored.split("$");
  if (parts.length !== 3 || parts[0] !== "scrypt") return false;
  const salt = Buffer.from(parts[1], "hex");
  const expected = Buffer.from(parts[2], "hex");
  const actual = scryptSync(password, salt, expected.length);
  if (actual.length !== expected.length) return false;
  return timingSafeEqual(actual, expected);
}
