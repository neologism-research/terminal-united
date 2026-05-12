import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import { loadEnv } from "../env.js";
import * as schema from "./schema.js";

loadEnv();

const databaseUrl = process.env.DATABASE_URL;

if (!databaseUrl) {
  throw new Error(
    "DATABASE_URL is required. Start the local database with `docker compose up -d postgres` and copy apps/server/.env.example to apps/server/.env.",
  );
}

export const sql = postgres(databaseUrl, { max: 10 });
export const db = drizzle(sql, { schema });