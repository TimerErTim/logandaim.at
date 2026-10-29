---
id: ADR-0002
title: Session cookies for auth; explicit-origin CORS
status: Superseded by ADR-0004
date: 2026-10-23
deciders: TimerErTim
---

# ADR-0002: Session cookie auth across frontend and API origins

> **Superseded 2026-10-26 by [ADR-0004](0004-jwt-refresh-auth.md).** The PO chose JWT access tokens
> with single-use rotating refresh tokens, so the opaque cookie session decided here is no longer
> the mechanism. The cross-site cookie analysis and the CORS allowlist reasoning below still explain
> the constraints; the decision itself does not stand. Product rules live in
> [`../product/konto.md`](../product/konto.md), not here.

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
   `Konto`; account deletion removes them. Sessions carry an expiry: a 30-day idle timeout
   (SPEC-konto rule 12) plus an absolute cap whose length is still open there (Q-15); expired rows
   are cleaned up.
4. **Production frontend uses the custom domain.** `https://logandaim.at` keeps frontend and API
   same-site, so `SameSite=Lax` cookies work in every browser. The bare `*.github.io` origin is
   preview-only; login from it is not supported (its requests are cross-site and cookies behave as
   third-party). If cross-site hosting ever becomes a hard requirement, a follow-up ADR replaces
   cookie sessions with bearer tokens or a same-origin proxy.
5. **CORS with explicit origins.** The server echoes `Access-Control-Allow-Origin` only for
   configured origins (`https://logandaim.at`, `http://localhost:3001` for development), always
   together with `Access-Control-Allow-Credentials: true`. Never `*` with credentials, never a
   reflected unvalidated `Origin`. Preflight (`OPTIONS`) and error responses carry the same
   headers, so 401/403 responses stay readable for the frontend. The frontend sets
   `credentials: "include"` on every API call; without it the browser neither stores nor sends the
   cookie cross-origin. The current server CORS layer (`backend/server/src/main.rs`) still allows
   any `localhost`/`127.0.0.1` port from an inline prefix check and sets no
   `Allow-Credentials`; it must move to a configured allowlist before the first auth endpoint
   ships.
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
  allowlist per environment. Development runs on `http://localhost`, which browsers treat as a
  trustworthy origin that may set `Secure` cookies, so the production cookie shape — including
  `Secure` and the `__Host-` prefix — applies unchanged in development; no dev-only exception is
  needed. Plain-HTTP development on any other host is unsupported; use localhost HTTPS (tunnel or
  local certificate) if that ever is required.
- The database gains a session table and cleanup for expired sessions.
- M2 (offline) will likely need short-lived tokens for sync workers; that gets its own ADR and can
  build on the same session store.

## Alternatives considered

1. **Bearer tokens (access + refresh) in `Authorization`.** Origin-independent — works from
   `*.github.io` and any future shell. Rejected for MVP, deciding reason: revocation (logout,
   recovery, deletion) needs a server-side store anyway, which erodes the stateless advantage.
   Weighing against it as well: the usual SPA trade-off keeps the refresh token in web-accessible
   storage (`localStorage`), an XSS target, while the `HttpOnly`-cookie refresh variant trades the
   origin-independence back to the third-party-cookie constraints this ADR already rejects.
   Revisit for M2.
2. **Third-party cookies with `SameSite=None; Secure`.** Works today only in browsers that still
   accept third-party cookies. Rejected: Safari and Firefox block it, Chrome deprecates it;
   login must work on phones in the field.
3. **Same-origin reverse proxy** (serve the API as `https://logandaim.at/api` from nginx or a CDN
   in front of the VPS). Removes CORS and keeps cookies strictly first-party. Rejected for MVP as
   an extra deployment component beyond the ADR-0001 baseline; it remains the preferred fallback
   if cross-site hosting becomes mandatory.
4. **JWT as session credential.** Rejected: revocation on logout/deletion needs a server-side
   denylist, which turns JWT back into a stateful session with extra complexity.

## Appendix: implementation sketch (non-normative)

Moved here from [`../product/konto.md`](../product/konto.md) on review (2026-10-23): technical
design stays out of product specs (`specs/README.md`). This sketch guides implementation and is
not part of the contract; the contract lives in SPEC-konto's rules and acceptance criteria.

### Data model (sketch)

| Store | Fields |
|-------|--------|
| `konten` | `id`, `e_mail_adresse` (unique, normalized), `passwort_hash`, `rolle` (`Benutzer` \| `Systemadministrator`), `e_mail_bestätigt_am` (nullable), `erstellt_am` |
| `anmelde_sitzungen` | `id`, `konto_id`, `token_hash`, `erstellt_am`, `läuft_ab_am`, `zuletzt_verwendet_am` |
| `passwort_wiederherstellungen` | `id`, `konto_id`, `token_hash`, `läuft_ab_am`, `verbraucht_am` |
| `e_mail_bestätigungen` | `id`, `konto_id`, `token_hash`, `läuft_ab_am`, `verbraucht_am` |

Session, recovery, and confirmation tokens are stored hashed; the raw token exists only in the
cookie resp. the mailed link. `Runden` and `Schüsse` reference `konten.id` optionally. On
`Konto-Löschung` (SPEC-konto rule 8) the reference and every personal attribute are stripped, and
the rows stay as anonymized records for aggregate statistics; the `Konto` row itself is deleted.

`Mitgliedschaften` do not live here. They belong to the `Anlage` aggregate (master-data spec,
issue #10): the loaded `Anlage` answers whether a `Konto` is its `Anlagen-Administrator` or
`Anlagen-Mitarbeiter`, which is why authorisation needs no lookup outside the aggregate a use case
already loads (SPEC-konto rule 16).

### API surface (sketch)

German domain paths as in ADR-0001, JSON keys `camelCase` UTF-8.

| Endpoint | Purpose |
|----------|---------|
| `POST /api/konto/registrierung` | Register (`eMailAdresse`, `passwort`) → creates the `Konto` and starts the first `Anmelde-Sitzung`; a taken address answers 409 and sets no cookie (SPEC-konto rule 4). |
| `POST /api/konto/anmeldung` | Log in (`eMailAdresse`, `passwort`) → starts `Anmelde-Sitzung`. |
| `POST /api/konto/abmeldung` | Log out → revokes the current `Anmelde-Sitzung`. |
| `POST /api/konto/passwort-vergessen` | Request `Passwort-Wiederherstellung` email (`eMailAdresse`). Same response for unknown emails. |
| `POST /api/konto/passwort-wiederherstellen` | Set new `Passwort` (`token`, `neuesPasswort`) → ends all sessions. |
| `POST /api/konto/e-mail-bestätigung` | Confirm the address with the mailed `token`; idempotent, 400 on expired or superseded (SPEC-konto rule 13). |
| `POST /api/konto/e-mail-bestätigung/anfordern` | Send a new confirmation mail; throttled per `Konto` and client (rule 14). |
| `GET /api/konto` | The logged-in `Konto` (`id`, `eMailAdresse`, `rolle`, `eMailBestätigt`). |
| `POST /api/konto/löschung` | `Konto-Löschung` (`passwort` re-entry + confirmation). |

Registration needs no follow-up login call: the request that creates the `Konto` starts its first
`Anmelde-Sitzung` in the response itself (SPEC-konto journey 1, rule 3, AC-1). A taken
`E-Mail-Adresse` answers 409 and sets no cookie (rule 4, AC-2).

**Account existence is disclosed by registration, deliberately.** Because registration logs the
user in, a response that also stayed silent about a taken address would be impossible: the cookie
itself answers the question. SPEC-konto rule 4 therefore makes the disclosure the contract — 409
with "address already in use" — instead of hiding one channel and leaking another. What an attacker
gains is the set of registered addresses; login and recovery stay generic, the
5-registrations-per-client-hour throttle (rule 5) keeps bulk probing slow, and no email is sent to
an address that did not ask for one. Error shape follows the existing structured errors (400
validation, 409 address taken, 401 not logged in, 403 wrong role, 429 throttled).

### Mail port (sketch)

Transaction mail leaves through one outbound port in the core; the delivery adapter is chosen by
configuration and is the only place that knows an SMTP host. Development and tests point at
Mailpit (`localhost:1025`, web UI on `8025`); the deployment points at the configured relay. A
failing mail server never fails a request: the port is called after the use case's own work and its
failure is logged, not propagated (SPEC-konto NFR-5).

### Authorisation (sketch)

No permission strings and no role middleware. A use case loads what it needs — the calling
`Konto` (with `rolle`), and the `Anlage` it acts on (with its `Mitgliedschaften`) — and decides
inside its own body. Reads of another `Konto`'s data are refused unless the caller is a
`Systemadministrator`. This keeps one rule per use case, testable at the domain layer without
spinning up a router.
