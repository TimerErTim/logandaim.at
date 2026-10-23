import standaloneConfig from "@/standalone-config.json";

function normalizeApiBaseUrl(url: string): string {
  const trimmed = url.replace(/\/+$/, "");
  return trimmed.endsWith("/api") ? trimmed : `${trimmed}/api`;
}

/**
 * Static runtime configuration.
 *
 * Everything is known at build time (a JSON import plus `NEXT_PUBLIC_*` env vars), so
 * there is nothing to resolve asynchronously. Keep it synchronous: no provider state,
 * no loading branch, no reload.
 */
export const config = {
  apiBaseUrl: normalizeApiBaseUrl(
    process.env.NEXT_PUBLIC_LOGANDAIM_API_URL ?? standaloneConfig.remoteApiUrl,
  ),
  dev: process.env.NEXT_PUBLIC_LOGANDAIM_DEV
    ? process.env.NEXT_PUBLIC_LOGANDAIM_DEV === "true"
    : process.env.NODE_ENV === "development",
} as const;
