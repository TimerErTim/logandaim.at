/**
 * Static runtime configuration.
 *
 * Everything is known at build time (`NEXT_PUBLIC_*` env vars), so there is nothing
 * to resolve asynchronously. Keep it synchronous: no provider state, no loading
 * branch, no reload.
 *
 * `apiBaseUrl` is the full base URL including any path prefix, exactly as
 * configured. Deciding where the API lives belongs to configuration, not to this
 * module.
 */
export const config = {
  // `||`, not `??`: an empty string is "unset", and an empty base URL would
  // silently point the frontend at its own origin. The deploy workflow refuses to
  // build without a value, and `dev:frontend` sets the local one.
  apiBaseUrl: (process.env.NEXT_PUBLIC_LOGANDAIM_API_URL || "").replace(
    /\/+$/,
    "",
  ),
  dev: process.env.NEXT_PUBLIC_LOGANDAIM_DEV
    ? process.env.NEXT_PUBLIC_LOGANDAIM_DEV === "true"
    : process.env.NODE_ENV === "development",
} as const;
