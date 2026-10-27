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
     `iss`, `aud`, `iat`, `exp`, `jti`. `iss` and `aud` come from
     configuration and are verified on every request, so a token minted for another deployment or
     audience is refused even with a valid signature; they were added during implementation and are
     therefore not in the originally approved text.
   - **Refresh token:** an opaque random 256-bit value, valid **one month**, stored only as a salted
     hash. Accepted **exactly once**: a renewal spends it and returns a new pair.
2. **The session row carries the state.** `sessions` holds the refresh-token hash, the public key
   that issued the access token, and a `revoked_at` column. Every authenticated request does two
   checks beyond the signature: the signature must verify against the public key stored on the row,
   and the row named by `sid` must exist, be unexpired, and not be revoked. There is no key table and
   no key id, so a row is its own key directory. A valid signature
   over a revoked session is still **401**. This is how logout, password recovery, and
   `Benutzer-Löschung` take effect on the next request instead of after 24 hours (SPEC-konto rules
   6, 15; AC-9, AC-17).
3. **Rotation is mandatory.** A renewal validates the presented refresh token against the session
   row, revokes the row's old token (marks it spent via `spent_at`, not the session), and issues a
   new access and refresh token on a **new session row**. Reuse of a spent refresh token fails with
   401 and revokes the whole session, because a spent token appearing twice means it leaked.
4. **Revocation is a row update, never a delete.** `revoked_at` is set; rows are cleaned up after
   their refresh expiry. Deleting rows would make a replayed token indistinguishable from an unknown
   one and would lose the audit trail.
5. **The public key travels on the session row.** Signing is Ed25519 behind a `Signing`
   port; the private key comes from deployment configuration (or is generated per process when
   none is configured) and never reaches the database. Each `sessions` row stores the hex public
   key its access token was issued under, so verification loads the session and checks the
   signature against that row's key. There is no key table and no join, which means a token whose
   session row is gone is a 401 without a second lookup (AC-17). Rotating a key therefore means a
   new session per client, and old sessions stop verifying once their rows are purged.
6. **Transport and storage.** `Authorization: Bearer` over HTTPS only; no token in a URL, no token in
   a query string, no token in a log line. The access token stays in frontend memory and is never
   written to `localStorage` or a cookie. The refresh token travels in a `__Host-` cookie scoped to
   the API origin with `HttpOnly`, `Secure`, and `SameSite=Lax`, so JavaScript cannot read it and a
   page reload can still renew. Each rotation re-sets the cookie.
7. **CORS with credentials for the same-site frontend.** Explicit origin allowlist
   (`https://logandaim.at`, plus the dev server's documented ports `3000` and `4173` on
   `localhost`/`127.0.0.1`) emitting `Access-Control-Allow-Headers: Authorization, Content-Type`,
   `Access-Control-Allow-Credentials: true` for those origins, and `Access-Control-Allow-Origin`
   never `*` and never an unvalidated reflection. The credential is the refresh cookie, sent only by
   the same-site production frontend; `*.github.io` previews are cross-site, receive no cookie, and
   run access-token-only. Preflight and error responses carry the same headers so 401/403 stay
   readable. Implemented: `allowed_origins` in `backend/server/src/main.rs` builds the allowlist
   from the configured `frontendBasisUrl` plus the dev server's documented ports; the earlier
   any-`localhost` prefix test is gone, because an origin that merely *starts* with a loopback
   prefix is a different origin.

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
- Deployment gains key management: an Ed25519 private key in configuration, or a generated
  per-process key for a throwaway run (decision 5).
- The database gains one `sessions` row per renewal, each carrying its own public key, plus
  cleanup for expired ones. Rows are never deleted before their refresh lifetime ends, because
  replay detection needs to find a spent token's row.
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
| `benutzer` | `id`, `rolle` (`Benutzer` \| `Systemadministrator`), `created_at` |
| `konten` | `id`, `benutzer_id`, `email` (unique, normalized), `password_hash`, `bestätigt_am`, `created_at` |
| `sessions` | `id`, `benutzer_id`, `refresh_token_hash`, `public_key`, `created_at`, `expires_at`, `spent_at`, `revoked_at` |
| `email_bestätigungen` | `id`, `email` (normalized, **not** a `konto_id` — the `Konto` does not exist yet), `token_hash`, `expires_at`, `spent_at` |
| `passwort_wiederherstellungen` | `id`, `konto_id`, `token_hash`, `expires_at`, `spent_at` |

Implemented in `crates/logandaim-sqlite/src/migrations/v0002_konto.rs`. The table and column names
are English because they name the technical objects above (`Session`, `SigningKey`), not product
nouns; `benutzer`, `konten` and `passwort_wiederherstellungen` keep their German names because those
*are* product nouns. Every row carries `_version` for optimistic concurrency, as the other tables do.

`email_bestätigungen` is keyed by address, not by account, because registration creates the
`Konto` only in step 2 (SPEC-konto rule 3). A confirmation row for an address that gained a `Konto`
in the meantime is spent without creating anything (rule 4).

`Runden` and `Schüsse` reference `benutzer.id` optionally. On `Benutzer-Löschung` (SPEC-konto rule
8) the reference and every personal attribute are stripped; the rows stay as anonymized records.
The `benutzer` and `konten` rows are deleted, and their `sessions` too.

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
| `POST /api/benutzer/löschung` | `Benutzer-Löschung` (`passwort` re-entry + confirmation). |

Error shape follows the existing structured errors: 400 validation or bad token, 401 not logged in
or revoked session, 403 wrong authority, 409 state conflict, 429 throttled. Registration has **no**
409: a taken address is swallowed at step 1.

### Token verification (sketch)

Per authenticated request, in order: parse the `Authorization` header; pin `alg` to `EdDSA`; load
`sessions` by `sid` (no row → 401); verify the Ed25519 signature over the ASCII `header.payload` with
the `public_key` on that row; require `revoked_at IS NULL` and `expires_at > now`; then load the
`Benutzer` for `sub`. The session lookup is one indexed read, and a token cannot outlive the key it
was signed with because the key travels with the row.

### Crypto is domain code (sketch)

Password hashing (argon2id), token hashing (SHA-256), and Ed25519 signing live in the `konto`
supporting subdomain, not in an adapter: they are domain decisions about what a credential is, not
I/O. What stays a port is the *key material* — the signing key arrives as configuration and the
verifier looks keys up through a repository — plus mail delivery and throttling, which touch the
outside world. This keeps the rules testable without a server, a database, or a clock.

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
