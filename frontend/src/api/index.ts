import { getConfig } from "@/lib/config";
import { HttpLogandaimApi } from "./http-client";
import type { LogandaimApi } from "./logandaim-api";

function normalizeApiBaseUrl(url: string): string {
  const trimmed = url.replace(/\/$/, "");
  if (trimmed.endsWith("/api")) {
    return trimmed;
  }
  return `${trimmed}/api`;
}

export async function createLogandaimApi(): Promise<LogandaimApi> {
  const config = await getConfig();
  return new HttpLogandaimApi(normalizeApiBaseUrl(config.remoteApiUrl));
}

/** Kept for provider reload(); instances are not cached module-wide. */
export function resetLogandaimApiCache(): void {}

export async function getLogandaimApi(): Promise<LogandaimApi> {
  return createLogandaimApi();
}
