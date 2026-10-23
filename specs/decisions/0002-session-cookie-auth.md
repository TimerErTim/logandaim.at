---
id: ADR-0002
title: Session cookies for auth; explicit-origin CORS
status: Proposed
date: 2026-10-23
deciders: TimerErTim
---

# ADR-0002: Session cookie auth across frontend and API origins

## Context

ADR-0001 places the static Next.js export on GitHub Pages and the Rust API (Poem server, SQLite)
on a private VPS over HTTPS. The two are different origins:

- Frontend: `https://logandaim.at` (custom domain on GitHub Pages). During bring-up and for
  previews the site is also served at `https://<user>.github.io`.
- Backend: `https://api.logandaim.at` (VPS).

The accounts spec ([`../product/konto.md`](../product/konto.md)) needs a login that survives page
reloads, revokes immediately on logout, password recovery, and account deletion, and keeps the
session credential out of JavaScript reach.

Cross-origin cookie constraints decide the mechanism. Browsers scope cookies by *site*
(scheme + registrable domain), not by origin:

- `logandaim.at` and `api.logandaim.at` are **cross-origin but same-site**. A cookie with
  `SameSite=Lax` is sent on `fetch` requests between them; the setup behaves like first-party
  cookies in all browsers.
- `<user>.github.io` and `api.logandaim.at` are **cross-site**. Cookies set by the API are then
  third-party cookies: blocked by Safari and Firefox, and being phased out by Chrome. Login would
  break on mobile Safari — a primary device for archers in the field.

CORS is needed either way: a static page on another origin may only read API responses the API
explicitly allows.

## Decision

1. **Server-side sessions with opaque cookies.** Login creates an `Anmelde-Sitzung` with a random
   256-bit token; the server stores only a hash. The raw token travels in a cookie; the frontend
   never sees it and sends no `Authorization` header.
2. **Cookie attributes.** `HttpOnly`, `Secure`, `SameSite=Lax`, `Path=/`, host-only to
   `api.logandaim.at` (no `Domain` attribute), name prefixed `__Host-`. Set on registration and
   login, cleared on logout.
3. **Server-side revocation.** Every request validates the session against the store. Logout
   revokes the current session; a completed password recovery revokes all sessions of the
   `Konto`; account deletion removes them. Sessions carry an expiry (rolling idle timeout plus an
   absolute cap); expired rows are cleaned up.
4. **Production frontend uses the custom domain.** `https://logandaim.at` keeps frontend and API
   same-site, so `SameSite=Lax` cookies work in every browser. The bare `*.github.io` origin is
   preview-only; login from it is not supported (its requests are cross-site and cookies behave as
   third-party). If cross-site hosting ever becomes a hard requirement, a follow-up ADR replaces
   cookie sessions with bearer tokens or a same-origin proxy.
5. **CORS with explicit origins.** The server echoes `Access-Control-Allow-Origin` only for
   configured origins (`https://logandaim.at`, `http://localhost:3001` for development), always
   together with `Access-Control-Allow-Credentials: true`. Never `*` with credentials, never a
   reflected unvalidated `Origin`. Preflight (`OPTIONS`) and error responses carry the same
   headers, so 401/403 responses stay readable for the frontend.
6. **CSRF defense in depth.** `SameSite=Lax` already withholds cookies from cross-site `fetch`
   requests. Additionally, state-changing requests must carry an `Origin` (or `Referer`) from the
   allowlist, and no state change is reachable via GET.

## Consequences

- Logout, recovery, and deletion take effect immediately everywhere; no token lifetime window
  after revocation.
- XSS cannot read the session cookie (`HttpOnly`). A logged-in XSS could still act as the user;
  the frontend stays a static bundle from one origin to keep that surface small.
- Login is broken on `*.github.io` previews in browsers that block third-party cookies. Known and
  accepted for previews; production on `logandaim.at` is unaffected.
- Deployment must configure the TLS certificate for `api.logandaim.at` and the CORS origin
  allowlist per environment. Development on `http://localhost` needs a dev-only exception for the
  `Secure` attribute or a localhost HTTPS setup.
- The database gains a session table and cleanup for expired sessions.
- M2 (offline) will likely need short-lived tokens for sync workers; that gets its own ADR and can
  build on the same session store.

## Alternatives considered

1. **Bearer tokens (access + refresh) in `Authorization`.** Origin-independent — works from
   `*.github.io` and any future shell. Rejected for MVP: the refresh token must live in
   web-accessible storage (`localStorage`), which is an XSS target; and revocation needs a
   server-side store anyway, which erodes the stateless advantage. Revisit for M2.
2. **Third-party cookies with `SameSite=None; Secure`.** Works today only in browsers that still
   accept third-party cookies. Rejected: Safari and Firefox block it, Chrome deprecates it;
   login must work on phones in the field.
3. **Same-origin reverse proxy** (serve the API as `https://logandaim.at/api` from nginx or a CDN
   in front of the VPS). Removes CORS and keeps cookies strictly first-party. Rejected for MVP as
   an extra deployment component beyond the ADR-0001 baseline; it remains the preferred fallback
   if cross-site hosting becomes mandatory.
4. **JWT as session credential.** Rejected: revocation on logout/deletion needs a server-side
   denylist, which turns JWT back into a stateful session with extra complexity.
