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
  // `||`, not `??`, so an empty variable is treated as unset. This module cannot
  // reject a missing value: it is evaluated during the build and during prerender,
  // where a throw would break the export for a reason the deploy already prevents.
  // A missing value therefore degrades to a same-origin base URL, which is wrong
  // but never ships: the Pages workflow fails the build without the variable, and
  // `dev:frontend` supplies the local default.
  apiBaseUrl: (process.env.NEXT_PUBLIC_LOGANDAIM_API_URL || "").replace(
    /\/+$/,
    "",
  ),
  dev: process.env.NEXT_PUBLIC_LOGANDAIM_DEV
    ? process.env.NEXT_PUBLIC_LOGANDAIM_DEV === "true"
    : process.env.NODE_ENV === "development",
} as const;
