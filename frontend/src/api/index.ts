import { getConfig } from "@/lib/config";
import { HttpYamsApi } from "./http-client";
import type { YamsApi } from "./yams-api";

function normalizeApiBaseUrl(url: string): string {
  const trimmed = url.replace(/\/$/, "");
  if (trimmed.endsWith("/api")) {
    return trimmed;
  }
  return `${trimmed}/api`;
}

export async function createYamsApi(): Promise<YamsApi> {
  const config = await getConfig();
  return new HttpYamsApi(normalizeApiBaseUrl(config.remoteApiUrl));
}

/** Kept for provider reload(); instances are not cached module-wide. */
export function resetYamsApiCache(): void {}

export async function getYamsApi(): Promise<YamsApi> {
  return createYamsApi();
}
