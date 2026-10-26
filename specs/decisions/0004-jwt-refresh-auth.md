---
id: ADR-0004
title: JWT access tokens with single-use rotating refresh tokens; server-checked revocation
status: Proposed
date: 2026-10-26
deciders: TimerErTim
---

# ADR-0004: JWT access tokens with rotating refresh tokens

Supersedes [ADR-0002](0002-session-cookie-auth.md), which chose opaque session cookies. The
accounts model changed on 2026-10-26 (see [`../product/konto.md`](../product/konto.md)): tokens are
now the session mechanism, so the earlier cross-site cookie reasoning no longer decides anything.

## Context

ADR-0001 places the static Next.js export on GitHub Pages and the Rust API (Poem server, SQLite) on
a private VPS over HTTPS. Frontend and backend are different origins:

- Frontend: `https://logandaim.at` (custom domain on GitHub Pages), plus `https://<user>.github.io`
  for pull-request previews.
- Backend: `https://api.logandaim.at` (VPS).

ADR-0002 rejected bearer tokens because logout, password recovery, and account deletion all need
immediate revocation, and a stateless JWT cannot be revoked. That objection is correct and this ADR
keeps it: the token is stateless for signature checking and **stateful for revocation**, decided by
one row lookup. What changed is the PO's preference for the token model and its independence from
cookie policy — `*.github.io` previews and any future native shell work without third-party cookie
constraints.

The product also stores its operating parameters. Access tokens are short enough that a stolen one
dies on its own; refresh tokens live long enough to matter, so they must be rotated and detectable
on reuse.

## Decision

1. **Two tokens per `Anmelde-Sitzung`.**
   - **Access token:** a JWT (Ed25519), valid **24 hours** from issue, sent on every API request as
     `Authorization: Bearer <token>`. Claims: `sub` (Benutzer id), `sid` (Anmelde-Sitzung id),
     `kid` (signing key id), `iat`, `exp`, `jti`.
   - **Refresh token:** an opaque random 256-bit value, valid **one month**, stored only as a salted
     hash. Accepted **exactly once**: a renewal spends it and returns a new pair.
2. **The session row carries the state.** `anmelde_sitzungen` holds the refresh-token hash, the
   signing key that issued the access token, and a `widerrufen_am` column. Every authenticated
   request does two checks beyond the signature: the JWT's `kid` must be a configured, active public
   key, and the row named by `sid` must exist, be unexpired, and not be revoked. A valid signature
   over a revoked session is still **401**. This is how logout, password recovery, and
   `Benutzer-Löschung` take effect on the next request instead of after 24 hours (SPEC-konto rules
   6, 15; AC-9, AC-17).
3. **Rotation is mandatory.** A renewal validates the presented refresh token against the session
   row, revokes the row's old token (marks it spent, not the session), and issues a new access and
   refresh token on a **new session row**. Reuse of a spent refresh token fails with 401 and revokes
   the whole session, because a spent token appearing twice means it leaked.
4. **Revocation is a row update, never a delete.** `widerrufen_am` is set; rows are cleaned up after
   their refresh expiry. Deleting rows would make a replayed token indistinguishable from an unknown
   one and would lose the audit trail.
5. **Signing keys are rows, not validity windows.** Signing is Ed25519 behind a `SignierSchluessel`
   port. A `signier_schluessel` table holds each key's id, public key, and active flag, and each
   `anmelde_sitzungen` row records which key signed its access token. Verification looks up the
   token's `kid`: an unknown or inactive row means 401 (AC-17). Retiring a key is therefore a row
   update with no overlap period to reason about — a token signed before retirement stops being
   accepted the moment the row is deactivated, and the only time-based concern is cleaning up rows
   whose sessions have all expired. The private key itself comes from deployment configuration and
   is never stored in the database.
6. **Transport and storage.** `Authorization: Bearer` over HTTPS only; no token in a URL, no token in
   a query string, no token in a log line. The access token stays in frontend memory and is never
   written to `localStorage` or a cookie. The refresh token travels in a `__Host-` cookie scoped to
   the API origin with `HttpOnly`, `Secure`, and `SameSite=Lax`, so JavaScript cannot read it and a
   page reload can still renew. Each rotation re-sets the cookie.
7. **CORS with credentials for the same-site frontend.** Explicit origin allowlist
   (`https://logandaim.at`, `http://localhost:3001` for development) emitting
   `Access-Control-Allow-Headers: Authorization, Content-Type`,
   `Access-Control-Allow-Credentials: true` for those origins, and `Access-Control-Allow-Origin`
   never `*` and never an unvalidated reflection. The credential is the refresh cookie, sent only by
   the same-site production frontend; `*.github.io` previews are cross-site, receive no cookie, and
   run access-token-only. Preflight and error responses carry the same headers so 401/403 stay
   readable. The current server layer (`backend/server/src/main.rs`) allows any `localhost` port via
   an inline prefix check and must move to the configured allowlist before the first auth endpoint
   ships.

## Consequences

- Revocation is immediate (one indexed row read per request) while the access token still saves a
  hash comparison on the hot path: the signature is verified without touching the database's token
  data, then one row is read for `sid`.
- A stolen access token is usable until it expires **unless** its session is revoked, and a stolen
  refresh token is usable at most once. Both limits are bounded and testable.
- Cross-origin policy stops deciding authentication: the access token works from `*.github.io`
  previews and any future shell. The **refresh cookie** is the one part that stays same-site, so
  previews run access-token-only and log the user out on reload.
- The long-lived credential is not visible to JavaScript, so XSS cannot exfiltrate a one-month
  token; it can still act as the user for the current access token's lifetime. The frontend stays a
  static bundle from one origin to keep that surface small.
- Deployment gains key management: an Ed25519 private key in configuration and a key table with an
  active flag, retired by row update (rule 15, decision 5).
- The database gains `signier_schluessel` and `anmelde_sitzungen` rows per renewed session, plus
  cleanup for expired ones.
- Refresh rotation costs one row insert per renewal, so a long-lived tab writes one row per month
  per device at a 30-day refresh lifetime and a rotation per renewal.

## Alternatives considered

1. **Opaque session cookie (ADR-0002).** Revocation is immediate and the credential is
   `HttpOnly`. Rejected: cross-site cookies are blocked in Safari and Firefox for `*.github.io`, and
   a token model was chosen instead so the frontend origin and future shells stay unconstrained.
2. **Stateless JWT, no session check.** Cheapest per request, but logout, recovery, and deletion
   would leave access tokens valid for up to 24 hours. Rejected: SPEC-konto rules 6 and 8 promise
   immediate effect; AC-9 and AC-17 test it.
3. **Short access tokens (15 minutes) instead of a revocation check.** Shrinks the window rather
   than closing it, and multiplies renewal traffic by 96 per day per device. Rejected as a
   substitute; it may complement the check later for other reasons.
4. **Both tokens in `localStorage`.** Renewal works from any origin, including store previews, and
   CORS stays credential-free. Rejected (Q-19, 2026-10-26): any XSS reads a one-month credential, and
   rotation only bounds how long it stays useful. Keeping the refresh token `HttpOnly` costs only
   the preview capability, which is acceptable for a preview.
5. **Access and refresh tokens both in `HttpOnly` cookies.** Simplest browser story, one transport.
   Rejected: the API would then depend on cookies for every call, which puts the access token's
   cross-origin behaviour back under cookie policy and loses the origin independence the token model
   was chosen for.

## Appendix: implementation sketch (non-normative)

Technical design stays out of product specs (`specs/README.md`). This sketch guides implementation
and is not part of the contract; the contract lives in SPEC-konto's rules and acceptance criteria.

### Data model (sketch)

| Store | Fields |
|-------|--------|
| `benutzer` | `id`, `rolle` (`Benutzer` \| `Systemadministrator`), `erstellt_am` |
| `konten` | `id`, `benutzer_id`, `e_mail_adresse` (unique, normalized), `passwort_hash`, `bestaetigt_am`, `erstellt_am` |
| `signier_schluessel` | `kid`, `oeffentlicher_schluessel`, `aktiv`, `erstellt_am`, `ausser_betrieb_am` |
| `anmelde_sitzungen` | `id`, `benutzer_id`, `erneuerungs_token_hash`, `signier_schluessel_id`, `erstellt_am`, `laeuft_ab_am`, `zuletzt_verwendet_am`, `verbraucht_am`, `widerrufen_am` |
| `e_mail_bestaetigungen` | `id`, `e_mail_adresse` (normalized, **not** a `konto_id` — the `Konto` does not exist yet), `token_hash`, `laeuft_ab_am`, `verbraucht_am` |
| `passwort_wiederherstellungen` | `id`, `konto_id`, `token_hash`, `laeuft_ab_am`, `verbraucht_am` |

`e_mail_bestaetigungen` is keyed by address, not by account, because registration creates the
`Konto` only in step 2 (SPEC-konto rule 3). A confirmation row for an address that gained a `Konto`
in the meantime is spent without creating anything (rule 4).

`Runden` and `Schüsse` reference `benutzer.id` optionally. On `Benutzer-Löschung` (SPEC-konto rule
8) the reference and every personal attribute are stripped; the rows stay as anonymized records.
The `benutzer` and `konten` rows are deleted, and their `anmelde_sitzungen` too.

`AnlageVerwaltungen` do not live here. They belong to the `Anlage` aggregate (master-data spec, issue
#10): the loaded `Anlage` answers whether a `Benutzer` is its `Administrator` or `Mitarbeiter`,
which is why authorisation needs no lookup outside the aggregate a use case already loads
(SPEC-konto rule 17).

### API surface (sketch)

German domain paths as in ADR-0001, JSON keys `camelCase` UTF-8.

| Endpoint | Purpose |
|----------|---------|
| `POST /api/registrierung/start` | Step 1 (`eMailAdresse`) → identical response for free and taken addresses; mails a confirmation for a free one (rule 4). |
| `POST /api/registrierung/abschluss` | Step 2 (`token`, `passwort`) → creates `Benutzer` + `Konto`, marks confirmed, returns the first token pair (rule 3). |
| `POST /api/anmeldung` | Log in (`eMailAdresse`, `passwort`) → access token in the body, refresh token as the `__Host-` cookie. |
| `POST /api/anmeldung/erneuerung` | Renew (refresh cookie) → new token pair, old refresh spent, cookie re-set (rule 12). |
| `POST /api/anmeldung/abmeldung` | Log out → revokes the current session and clears the refresh cookie. |
| `POST /api/passwort-vergessen` | Request recovery mail; same response for unknown addresses. |
| `POST /api/passwort-wiederherstellen` | Set new `Passwort` (`token`, `neuesPasswort`) → ends all sessions. |
| `GET /api/benutzer` | The logged-in `Benutzer` (`id`, `eMailAdresse`, `rolle`). |
| `POST /api/benutzer/loeschung` | `Benutzer-Löschung` (`passwort` re-entry + confirmation). |

Error shape follows the existing structured errors: 400 validation or bad token, 401 not logged in
or revoked session, 403 wrong authority, 409 state conflict, 429 throttled. Registration has **no**
409: a taken address is swallowed at step 1.

### Token verification (sketch)

Per authenticated request, in order: parse the `Authorization` header; verify the Ed25519 signature
with the public key named by `kid` (unknown or retired `kid` → 401); check `exp`; load
`anmelde_sitzungen` by `sid` and require `widerrufen_am IS NULL` and `laeuft_ab_am > now`; then load
the `Benutzer` for `sub`. Cache the key set in memory; the session lookup is one indexed read and
may be cached per session id for a few seconds if NFR-4 needs it, at the cost of that much
revocation latency.

### Mail port (sketch)

Transaction mail leaves through one outbound port in the core; the delivery adapter is chosen by
configuration and is the only place that knows an SMTP host. Development and tests point at Mailpit
(`localhost:1025`, web UI on `8025`); the deployment points at the configured relay. A failing mail
server never fails a request: the port is called after the use case's own work and its failure is
logged, not propagated (SPEC-konto NFR-5).

### Authorisation (sketch)

No permission strings and no role middleware. A use case loads what it needs — the calling
`Benutzer` (with `rolle`), and the `Anlage` it acts on (with its `AnlageVerwaltungen`) — and decides inside
its own body. Reads of another `Benutzer`'s data are refused unless the caller is a
`Systemadministrator`. One rule per use case, testable at the domain layer without a router.
