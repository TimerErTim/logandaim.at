import standaloneConfig from "@/standalone-config.json";

/**
 * Static runtime configuration.
 *
 * Everything is known at build time (a JSON import plus `NEXT_PUBLIC_*` env vars), so
 * there is nothing to resolve asynchronously. Keep it synchronous: no provider state,
 * no loading branch, no reload.
 *
 * `apiBaseUrl` is the full base URL including any path prefix, exactly as configured.
 * Deciding where the API lives belongs to configuration, not to this module.
 */
export const config = {
  apiBaseUrl: (
    process.env.NEXT_PUBLIC_LOGANDAIM_API_URL ?? standaloneConfig.remoteApiUrl
  ).replace(/\/+$/, ""),
  dev: process.env.NEXT_PUBLIC_LOGANDAIM_DEV
    ? process.env.NEXT_PUBLIC_LOGANDAIM_DEV === "true"
    : process.env.NODE_ENV === "development",
} as const;
