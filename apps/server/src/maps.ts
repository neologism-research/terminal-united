import { parseLdtkMap, type ParsedMap } from "@tu/map-engine";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));

/** Path to the bundled hub map within the map-engine package. */
const HUB_PATH = resolve(here, "../../../packages/map-engine/maps/hub.json");

let cached: ParsedMap | null = null;

export function getHubMap(): ParsedMap {
  if (!cached) {
    const raw = JSON.parse(readFileSync(HUB_PATH, "utf-8"));
    cached = parseLdtkMap(raw);
  }
  return cached;
}
