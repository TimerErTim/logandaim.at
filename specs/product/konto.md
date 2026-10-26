---
id: SPEC-konto
title: Benutzer and Konto (registration, login, recovery, deletion, roles)
status: PO-approved
owner: TimerErTim
issues: [9, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 50, 68, 69]
superseded_by: null
---

# Konto — Benutzer, registration, and login

## Problem

People need an identity to log Runden and review their own history, and the product needs a
superuser who can create and manage everything. The frontend runs as a static export on GitHub
Pages while the backend runs on a VPS (see
[`../decisions/0001-baseline-architecture.md`](../decisions/0001-baseline-architecture.md)), so
login must work across two origins.

A person is also not the same thing as the way they log in. Today a `Benutzer` proves who they are
with an email address and a password. Later they may additionally sign in with a provider or a
passkey. If the credential were the identity, every one of those additions would rewrite the
product's data model, so this spec separates the two from the start.

## Outcome

A person registers with an email address and a password, confirms that the address is reachable, and
from then on logs in and out, recovers a forgotten password, and deletes the account without help.
Registration is started by the address itself: the app only creates the `Benutzer` once the mailed
confirmation link has been opened, so an address that nobody can receive mail at never becomes an
account.

Authority inside an `Anlage` — as `Administrator` or `Mitarbeiter` — is an `AnlageVerwaltung` granted by
someone inside that `Anlage`, and it changes what the person may do there, never who they are. A
`Systemadministrator` additionally creates and manages every `Anlage` and every `Benutzer`.
Personal data belongs to exactly one `Benutzer` and is visible only to its owner.

## Ubiquitous language

These five terms are not interchangeable. Code, API, UI, and specs use each in exactly one sense.

| Term | Definition | Owns |
|------|------------|------|
| `Person` | The human being in the real world. Never stored; exists only in prose. | — |
| `Benutzer` | The logical identity the product knows: one person's account in logandaim.at, owning personal data, `Runden`, and `AnlageVerwaltungen`. Exactly one per person. | Personal data, `Runden`, `AnlageVerwaltungen` |
| `Konto` | One way to authenticate a `Benutzer`: an email address plus a `Passwort`, or later a provider identity or passkey. A `Benutzer` has at least one, and several once provider or passkey login ships. | `E-Mail-Adresse`, `Passwort`, credentials |
| `AnlageVerwaltung` | A `Benutzer`'s role inside one `Anlage`: `Administrator` or `Mitarbeiter`. Belongs to the `Anlage`, never to the `Benutzer`. | — |
| `Anmelde-Sitzung` | One logged-in state of a `Benutzer` on one device, made of an access and a refresh token. Revocable at any time. | Token pair lifetime |

`Mitgliedschaft` is not used: it reads as club membership (*Vereinsmitglied*), while an
`AnlageVerwaltung` means "I work here and manage this". The name carries its scope — the `Anlage`
prefix says what is administered — and the role it grants is `Administrator` or `Mitarbeiter`. The
former `Anlagenbetreiber` role does not exist.

The frontend and the API speak the same German terms. UI copy says `Benutzer`, `Konto`, `Anlage`,
and `AnlageVerwaltung`; it never says "Account" and "Nutzer" for the same thing.

**Auth-technical vocabulary stays English.** German engineering usage names these in English, and
so does the code; hard-translating them ("Erneuerungs-Token") invents words no reader uses. Product
nouns — the things a `Benutzer` sees and owns — stay German. The same concept therefore has one
product name and one code name, and neither is a synonym for a different concept:

| Product term (spec, UI) | Code term (identifiers) | Meaning |
|-------------------------|-------------------------|---------|
| `Anmelde-Sitzung` | `Session` | One logged-in state of a `Benutzer` on one device. |
| `Zugriffs-Token` | `AccessToken` | Short-lived signed access credential, a JWT. |
| `Erneuerungs-Token` | `RefreshToken` | Long-lived opaque credential, accepted exactly once. |
| *(the mailed link value)* | `SecretToken` | Single-use random value, mailed and never stored raw. |
| *(the stored form)* | `TokenHash` | SHA-256 of a `SecretToken`; the only form persisted. |
| `Signierschlüssel` | `SigningKey` | Ed25519 key that signs access tokens; public half is a row. |
| *(the stored form)* | `PasswordHash` | argon2id hash of a `Passwort`; the only form persisted. |

The mapping is one-to-one in both directions. A `TokenHash` is never a `SecretToken`, and neither is
a `Session`: the pair of tokens belongs to a `Session`, they are not the `Session`.

## Roles

- **`Benutzer`** — every registered person, including those who hold an `AnlageVerwaltung`
  somewhere. Registers, logs in, manages the own `Benutzer`, logs Runden. There is no separate
  account role for an operator; whom they may manage comes from the `AnlageVerwaltung`.
- **`Systemadministrator`** — a `Benutzer` whose account-level authority spans the product: creates
  and manages every `Anlage`, every `Benutzer`, and every `AnlageVerwaltung`.

Account-level authority therefore has exactly two values (`Benutzer`, `Systemadministrator`),
carried on the `Benutzer`; everything else is an `AnlageVerwaltung` inside one `Anlage`. See
[`charter.md#roles`](charter.md#roles).

## Scope

**In scope**

- Registration in two steps: address first, then `Passwort`, gated by the mailed confirmation link.
- `E-Mail-Adresse-Bestätigung`.
- Login, logout, and session renewal.
- Password reset for a forgotten `Passwort` (email link).
- Self-service deletion of the own `Benutzer`.
- Sending transaction mail through a configured SMTP server; Mailpit in development and tests.
- The distinction between `Benutzer` and `Konto` with one `Konto` per `Benutzer` in MVP.
- Account-level authority (`Benutzer`, `Systemadministrator`) and the superuser's cross-product
  reach.

**Out of scope for MVP**

Everything below is deferred by PO decision, not rejected. Each item has its own Draft section
further down and a delivery issue; nothing here is implementable until that section is
`PO-approved`.

- Changing the account email ([later](#email-change), #29).
- Changing the `Passwort` while logged in and session management ([later](#account-security), #30).
- OAuth or other login providers (PO decision 2026-10-23; [later](#external-identity-providers), #32).
- 2FA / TOTP ([later](#mfa), #31) and passkeys ([later](#passkeys), #33).
- Authentication audit log and new-device notices ([later](#auth-audit), #35).
- Notification preferences ([later](#notifications), #50).
- GDPR data export (PO decision 2026-10-23; see `datenschutz.md`, #36).
- Password strength meter UI and "remember me" options.
- `AnlageVerwaltung` management inside an `Anlage` (master-data spec, issue #10); this spec only defines
  the account-level authority those roles attach to.
- A `Benutzer` requesting a new `Anlage`; the request flow is its own spec
  ([`anlage-anfrage.md`](anlage-anfrage.md), #68).

## User journeys

1. **Register, step 1 — address.** The visitor enters an `E-Mail-Adresse`. The app answers the same
   way whether or not that address is free (rule 4). If it is free, the app mails an
   `E-Mail-Adresse-Bestätigung` with a single-use token. No `Benutzer` exists yet.
2. **Register, step 2 — `Passwort`.** The visitor opens the mailed link, which leads to a frontend
   route that exchanges the token with the API (Q-17). The app accepts the token, rejects an expired,
   used, or unknown one, and accepts a `Passwort` of at least 8 characters. Only then are the
   `Benutzer` and the `Konto` created, the address marked confirmed, and the `Benutzer` logged in.
3. **Login.** The returning `Benutzer` logs in with address and `Passwort`, receives an access token
   and a refresh token, and lands on the home screen.
4. **Stay logged in.** Before the access token expires the frontend renews it with the refresh token.
   Renewal rotates the refresh token: the used one is spent (rules 12, 13).
5. **Logout.** The `Benutzer` logs out; the refresh token is revoked server-side and further calls
   with the access token fail.
6. **Reset password.** The `Benutzer` who forgot the `Passwort` requests a reset, receives an email
   link, sets a new `Passwort`, and logs in; every other session ends.
7. **Delete `Benutzer`.** In settings the `Benutzer` re-enters the `Passwort` and confirms; the
   `Benutzer`, its `Konto`, its sessions, and its personal data are gone, its `Runden` stay
   anonymized for aggregate statistics, and the login ends.
8. **Superuser.** The `Systemadministrator` logs in and creates an `Anlage`, its Parcours, Ziele, and
   Pflöcke, and grants another `Benutzer` the `Administrator` `AnlageVerwaltung` (master-data spec).

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Benutzer` | The logical identity; owns personal data, `Runden`, and `AnlageVerwaltungen`. |
| `Konto` | One way to authenticate a `Benutzer`. MVP: an `E-Mail-Adresse` plus a `Passwort`, one per `Benutzer`. |
| `E-Mail-Adresse` | Login identifier of a `Konto`. Normalized: trimmed, lowercase. Unique across all `Konto`. |
| `Passwort` | Secret of a `Konto`. Stored only as a salted hash, never in plain text or logs. |
| `Rolle` | Account-level authority of a `Benutzer`: `Benutzer` or `Systemadministrator`. |
| `AnlageVerwaltung` | A `Benutzer`'s role in one `Anlage`: `Administrator` or `Mitarbeiter`. Belongs to the `Anlage`, not to the `Benutzer`. |
| `Anmelde-Sitzung` | One logged-in state of a `Benutzer` on one device, represented by a pair of access and refresh tokens. |
| `Zugriffs-Token` | Short-lived signed access credential (JWT) presented on every API request. Valid one day. |
| `Erneuerungs-Token` | Long-lived opaque credential, valid one month and usable exactly once, exchanged for a new token pair. |
| `E-Mail-Adresse-Bestätigung` | Single-use, time-limited token mailed to an address to prove it is reachable; gates registration. |
| `Passwort-Wiederherstellung` | Single-use, time-limited token sent by email to set a new `Passwort`. |
| `Benutzer-Löschung` | Irreversible self-service removal of the `Benutzer`, its `Konto`, and its personal data; its `Runden` stay anonymized. |
| `Anonymisierte Runde` | A `Runde` (with its `Schüsse`) whose link to a `Benutzer` and whose personal attributes are stripped. Kept for aggregate statistics; no longer personal data. |

Rules:

1. A `Benutzer` is identified by a surrogate id. Every `Konto` of that `Benutzer` is a way to
   authenticate it; in MVP there is exactly one `Konto` per `Benutzer`, created together with it.
2. A `Konto` is identified by its normalized `E-Mail-Adresse`, which is unique across all `Konto`. A
   `Passwort` has at least 8 characters; no composition rules and no periodic rotation.
3. A `Benutzer` is created only by a completed registration: an accepted `E-Mail-Adresse-Bestätigung`
   token plus a valid `Passwort` (journey 2). Registering does not create anything before that step,
   so at no point does an unconfirmed `Benutzer` or `Konto` exist. The successful step-2 request logs
   the `Benutzer` in and starts the first `Anmelde-Sitzung`.
4. Registration discloses nothing about which addresses are already registered. Step 1 answers
   identically — same status, same body, same timing envelope — whether the address is free or
   already belongs to a `Konto`. A free address receives a confirmation mail; a taken one receives
   nothing, and its owner is not mailed either. Step 2 with a token that was never issued, or was
   issued for an address that gained a `Konto` in the meantime, fails with 400 and creates nothing.
   Because the address is confirmed before the `Passwort` is set, no `Benutzer` ever exists whose
   address is unreachable, and there is no "already taken" error at registration at all. Login and
   `Passwort-Wiederherstellung` stay equally silent about existence.
5. Throttles are scoped per client (IP plus client fingerprint), never per `E-Mail-Adresse`: there
   is no address lockout, so nobody can lock an owner out by guessing their address (Q-16 resolved).
   Fixed thresholds (Q-4; configurability tracked in issue #26): 20 failed logins per 15 minutes,
   5 step-1 registrations per hour, 5 step-2 `Passwort` submissions per hour, 5 recovery requests per
   hour, 5 renewal attempts per hour. Exceeding a throttle fails with 429 until the window passes. A
   successful login resets that client's failure counter.
6. A `Benutzer` may hold several `Anmelde-Sitzungen` (multiple devices). Logout ends the current
   `Anmelde-Sitzung` by revoking its refresh token. A completed `Passwort-Wiederherstellung` or a
   `Benutzer-Löschung` ends every `Anmelde-Sitzung` of the `Benutzer`.
7. A `Passwort-Wiederherstellung` token is single-use, bound to one `Konto`, and expires 60 minutes
   after it was requested. Requesting again invalidates the previous token.
8. `Benutzer-Löschung` requires re-entering the `Passwort` and explicit confirmation. It removes the
   `Benutzer`, every `Konto` of it, all its credentials, all its `Anmelde-Sitzungen`, and every
   personal attribute — including the link from its `Runden` and `Schüsse` to the `Benutzer` (Q-1).
   The `Runden` and `Schüsse` stay as `Anonymisierte Runde` records for aggregate statistics.
   Anonymization contract: kept are the sporting results, dates, and master-data references;
   stripped are the `Benutzer` reference and every personal attribute, so nobody can attribute a kept
   `Runde` to a person. It is irreversible. Master data (Anlage, Parcours, Ziel, Pflock) is
   untouched. The freed `E-Mail-Adresse` can register again.
9. Account-level authority never rises through self-service. A `Systemadministrator` is provisioned
   out of band (decision log, Q-2; tracked in issue #34). No API path promotes a `Benutzer`.
10. Personal data is visible only to the owning `Benutzer`; no user-to-user visibility in MVP
    (charter Q-3 default).
11. Every request for personal data or a personal action requires a valid `Zugriffs-Token`. Only the
    two registration steps, the address confirmation, login, password recovery, and token renewal
    work without one.
12. An `Anmelde-Sitzung` is a pair of tokens. The `Zugriffs-Token` is a JWT valid 24 hours from
    issue and travels in the `Authorization` header; the frontend keeps it in memory only, never in
    `localStorage` or a cookie. The `Erneuerungs-Token` is opaque, valid one month from issue, and
    accepted exactly once: renewing spends it, revokes the rest of that `Anmelde-Sitzung`'s old token
    pair, and returns a new pair, so a stolen refresh token is usable at most once and its use is
    detectable. It travels in a `__Host-` cookie scoped to the API origin, `HttpOnly`, `Secure`,
    `SameSite=Lax`, so JavaScript can never read it and a page reload can still renew (rule 12a). An
    `Anmelde-Sitzung` whose refresh token expired cannot be renewed; the `Benutzer` is logged out and
    logs in again. Every revocation in rule 6 takes effect on the next request, including while an
    access token is still inside its 24 hours (rule 15).
12a. The refresh cookie is scoped to `api.logandaim.at` and carries `SameSite=Lax`, so it is sent to
    the same-site production frontend `logandaim.at` but not to any other site. Cross-site hosting,
    including `*.github.io` previews, receives no refresh capability: previews work with an access
    token only and log the user out on reload. A cross-origin renewal sets the cookie again on every
    rotation, and the API answers renewal with credentials allowed for configured origins only.
13. An `E-Mail-Adresse-Bestätigung` token is single-use, bound to one requested address, and expires
    24 hours after it was sent. A second use fails with 400. Requesting a new one invalidates the
    previous token and is throttled per client (rule 5).
14. Address possession is the gate for everything that writes: a `Benutzer` exists only with a
    confirmed address (rule 3), so no rule here needs to define what an unconfirmed one may do. A
    `AnlageVerwaltung` may only be granted to an existing `Benutzer`, which is therefore always confirmed
    (Q-18 resolved).
15. Every API request presenting a `Zugriffs-Token` checks the token against the server: the signing
    key it claims must be a known, active public key, and its `Anmelde-Sitzung` must not be revoked.
    Both are database rows, not clocks: the key table says which key signed which session and
    whether the key is still active, and the session row says whether that session is revoked.
    Revocation is a row state, not a token expiry, so logout, recovery, and deletion are immediate
    (rule 6). A request whose session is revoked, or whose `kid` names a retired key, fails with 401
    even if the JWT is otherwise valid and unexpired. Retiring a key needs no overlapping validity
    window: it is a row update, and the only time-based part is the cleanup of retired rows.
16. `AnlageVerwaltungen` grant authority over one `Anlage`, never over the product. An
    `Administrator` manages that `Anlage`'s `AnlageVerwaltungen` and master data; a `Mitarbeiter` its
    master data and statistics. Neither may see the `Runden` or `Schüsse` of another `Benutzer`, and
    neither may touch another `Anlage`. A `Systemadministrator` may do both for every `Anlage`. An
    `AnlageVerwaltung` is written in the `Anlage` and never follows a `Benutzer` between Anlagen.
17. Authorisation is decided by each use case from the aggregates it loads — the `Benutzer` and its
    `Rolle`, the `Anlage` and its `AnlageVerwaltungen`. There is no permission table, no role
    middleware, and no endpoint whose access is decided by a permission string.

## Requirements

**Functional**

- FR-1: Start registration with an `E-Mail-Adresse` alone, answering identically for free and taken
  addresses, and mail an `E-Mail-Adresse-Bestätigung` only for a free one (rule 4).
- FR-2: Complete registration from a valid confirmation token plus a `Passwort` of at least 8
  characters: create the `Benutzer` and its one `Konto`, mark the address confirmed, start the first
  `Anmelde-Sitzung`, and return an access and a refresh token (rule 3). A `Passwort` shorter than 8
  characters fails with 400 and creates nothing.
- FR-3: Log in with address + `Passwort`, returning an access and a refresh token, with one generic
  failure for any wrong credentials (rule 4).
- FR-4: Renew an `Anmelde-Sitzung` from its refresh token, rotating the pair (rule 12).
- FR-5: Log out by revoking the current `Anmelde-Sitzung` server-side, effective on the next request
  (rule 15).
- FR-6: Reset a forgotten `Passwort`: request with an `E-Mail-Adresse` always returns the same
  response; an existing `Konto` receives a `Passwort-Wiederherstellung` email; completing it sets the
  new `Passwort` and ends all `Anmelde-Sitzungen`.
- FR-7: Delete the own `Benutzer` with `Passwort` re-entry and confirmation, removing the `Benutzer`,
  its `Konto`, credentials, sessions, and personal data; its `Runden` and `Schüsse` stay anonymized
  per rule 8.
- FR-8: Reject any request presenting an expired, unknown-key, or revoked-session access token with
  401 (rule 15).
- FR-9: Decide every authorised action from the loaded aggregates (rule 17): a `Benutzer` reaches
  only the own data and the `Anlagen` they hold an `AnlageVerwaltung` in; a `Systemadministrator` reaches
  every `Anlage` and every `Benutzer`. No promotion path exists (rule 9).
- FR-10: Enforce the client-scoped thresholds of rule 5 for failed logins, registration steps,
  recovery requests, and renewals. Never throttle or lock an `E-Mail-Adresse` as such.
- FR-11: Send every transaction mail — the address confirmation and the password recovery — through
  one configured SMTP endpoint. Development and tests target Mailpit; no test path reaches a real
  address.

**Non-functional**

- NFR-1: `Passwort` values are stored as salted hashes (argon2id) and never returned or logged.
- NFR-2: The refresh token is never readable from JavaScript — it is delivered in an `HttpOnly`
  cookie — and no token ever travels in a URL. The access token lives in memory only. All tokens are
  transmitted over HTTPS. Token shape and storage are decided in
  [`../decisions/0004-jwt-refresh-auth.md`](../decisions/0004-jwt-refresh-auth.md).
- NFR-3: Login works from the deployed frontend origin across the cross-origin deployment.
- NFR-4: Auth endpoints respond within 500 ms p95, excluding email delivery.
- NFR-5: Mail is sent asynchronously: a slow or failing SMTP server never delays or fails a
  registration, login, or recovery response.

## Implementation sketches

Non-normative implementation sketches (data model, API surface) are technical design and live in
[ADR-0004's appendix](../decisions/0004-jwt-refresh-auth.md#appendix-implementation-sketch-non-normative),
not in this spec. They guide implementation and are not part of the contract; the contract lives in
the rules and acceptance criteria above.

## Acceptance criteria

- AC-1: Given no `Konto` for an address, when starting registration with that address, then the
  response equals the response for an address that already has a `Konto` (same status, same body),
  no `Benutzer` and no `Konto` exist yet, and exactly one `E-Mail-Adresse-Bestätigung` is mailed to
  it.
- AC-2: When starting registration with an address that already has a `Konto`, then the response is
  indistinguishable from AC-1 and no mail is sent to that address.
- AC-3: Given a valid unexpired confirmation token, when submitting a `Passwort` of at least 8
  characters with it, then the `Benutzer` and its one `Konto` exist with `Rolle` `Benutzer`, the
  address is confirmed, and the response starts an `Anmelde-Sitzung` with an access and a refresh
  token without a separate login step.
- AC-4: When completing registration with an expired, already-used, or never-issued token, or with a
  `Passwort` shorter than 8 characters, then the request fails with 400, no `Benutzer` is created,
  and the token stays spent only if it was already spent.
- AC-5: Given a `Benutzer`, when logging in with the correct address and `Passwort`, then an
  `Anmelde-Sitzung` is established and protected requests carrying the access token succeed.
- AC-6: When logging in with a wrong `Passwort`, then the response equals the response for an unknown
  address (401, same body), and no `Anmelde-Sitzung` is established.
- AC-7: Given a valid access token, when the frontend renews with the refresh token, then a new
  access and refresh token are returned, the old refresh token fails on its next use, the access
  token from before the renewal carries on working until it expires, and the new refresh cookie
  replaces the old one.
- AC-7a: The refresh cookie is `HttpOnly`, `Secure`, `__Host-` prefixed, scoped to the API origin,
  and `document.cookie` does not expose it; the access token appears in no cookie and no web
  storage.
- AC-8: Given a refresh token older than one month, or a spent one, when renewing, then the request
  fails with 401 and the `Anmelde-Sitzung` can no longer be renewed.
- AC-9: When logging out, then the refresh token is revoked and post-revocation requests carrying the
  access token fail with 401, even though that token has not expired.
- AC-10: When requesting `Passwort-Wiederherstellung` for any address, then the response is identical
  whether or not a `Konto` exists.
- AC-11: Given a `Konto` with a recovery token inside its 60-minute validity, when setting a new
  `Passwort` with the token, then the `Passwort` changes, all `Anmelde-Sitzungen` end including their
  access tokens, and the token cannot be used a second time; an expired or unknown token fails with
  400.
- AC-12: When deleting the `Benutzer` with the correct `Passwort` and confirmation, then the
  `Benutzer`, its `Konto`, its credentials, its `Anmelde-Sitzungen`, and all personal data are gone;
  its `Runden` and `Schüsse` remain as anonymized records that still count in aggregate statistics
  but no longer reference the deleted `Benutzer`; login with the old credentials fails; and the
  address can register again.
- AC-13: When an unauthenticated request hits any protected endpoint, then it fails with 401.
- AC-14: When a `Benutzer` without an `AnlageVerwaltung` requests an action on an `Anlage`, then it fails
  with 403; the same request succeeds for that `Anlage`'s `Administrator`, its `Mitarbeiter` where
  the action is theirs, and for a `Systemadministrator`. A `Benutzer` requesting another `Benutzer`'s
  `Runden` fails with 403 for every role but the `Systemadministrator`.
- AC-15: After 20 failed logins from one client within 15 minutes, further attempts from that client
  fail with 429 for the rest of the window, and no attempt ever locks or throttles an
  `E-Mail-Adresse` for anyone else.
- AC-16: When registration step 1 exceeds 5 requests per client per hour, or step 2 exceeds 5 per
  client per hour, or recovery exceeds 5 per client per hour, or renewal exceeds 5 per client per
  hour, then the excess requests fail with 429 until the window passes.
- AC-17: Given a valid refresh token and the signing key that issued it, when the session is not
  revoked and the key row is active, then the request is accepted; when the session row is revoked,
  then the same unexpired access token is rejected with 401; when the key row is retired or the
  `kid` is unknown, then it is rejected with 401.
- AC-18: Given an access token that has passed its 24-hour validity, when sending any request with
  it, then the request fails with 401 and the frontend is expected to renew.
- AC-19: With Mailpit running and the API pointed at it, when starting a registration or requesting a
  recovery, then the mail arrives in Mailpit with a working link; no test and no local run contacts
  any other SMTP server.
- AC-20: The API and the database schema contain no unconfirmed `Benutzer` state: every stored
  `Benutzer` has a confirmed address, and no accept path exists that creates one before step 2.

## Deferred capabilities

The capabilities in this part are **Draft** and not part of the MVP contract above. Each one has a
delivery issue referenced in its heading. They are written now so the deferred requirements are
specified somewhere and do not get relitigated from scratch. Open questions block approval of the
individual section only, not of the MVP part.

### Multiple credentials per `Benutzer`

Issue: #32 and #33. Depends on this spec. `Konto` exists for this future: a `Benutzer` gains a
second `Konto` (a provider identity or a passkey) instead of a parallel account concept.

Rules:

1. Every `Konto` belongs to exactly one `Benutzer`; a `Benutzer` must keep at least one usable
   `Konto` at all times.
2. Adding a `Konto` requires proving an existing one of the same `Benutzer` first.
3. Removing the last usable `Konto` is refused.
4. All `Konto` of one `Benutzer` share its `Anmelde-Sitzungen` and personal data; there is exactly
   one `Benutzer` row regardless of how many `Konto` it has.

### Email delivery

Issue: #27. The MVP needs the SMTP channel and Mailpit; this section keeps the production-provider
requirements for the follow-up that replaces Mailpit with real email delivery.

Rules:

1. Mail leaves the product only through the one SMTP endpoint in configuration (MVP, FR-11). Moving
   to a hosted provider changes that endpoint and nothing else in the product.
2. The sender domain of the production endpoint has SPF, DKIM, and DMARC records that pass.
3. A message body containing a token is never retained longer than the token lifetime, in the
   provider or in application logs.
4. Hard bounces and complaints land on a suppression list that blocks further sends to that address.
   A suppressed address makes registration step 1 fail the same silent way as a taken address
   (rule 4), so a bounce cannot be turned into a probing oracle.
5. Development and tests send only to Mailpit; no test path can reach a real address.
6. Provider region, data processing agreement, and mail-log retention are recorded in
   `datenschutz.md#processors`.

Open questions:

- [ ] Q-5: Provider and sending domain for production.
- [ ] Q-6: Is the production mail channel a launch gate, or may the MVP launch against a
  self-hosted SMTP relay?

### Email verification

Issue: #28. Delivered as part of MVP registration, not as a separate feature.

Rules:

1. Registration step 1 sends an `E-Mail-Adresse-Bestätigung` with a single-use token, valid 24 hours
   (rule 13).
2. Confirming a token does not itself create the `Benutzer`; it authorises step 2, which creates it
   (rule 3). An expired or replayed token fails with 400 and mails nothing.
3. A resend is throttled per client; a token issued for an address that gained a `Konto` in the
   meantime is spent without creating anything.

Resolved: with verification gating registration there is no unconfirmed state, so the former Q-7
(transition rule) and Q-18 (may an unconfirmed `Benutzer` hold an `AnlageVerwaltung`) are obsolete.

### Email change

Issue: #29. Depends on email verification.

Rules:

1. A change requires the current `Passwort` (or a step-up per [MFA](#mfa)) and sends a verification
   token to the new address.
2. The new address becomes the login identifier only after its token is confirmed; until then the
   old address still works.
3. Both addresses receive a notice after a completed change.
4. Uniqueness of the normalized `E-Mail-Adresse` is enforced at request and again at confirmation;
   a concurrent confirmation cannot produce two `Konto` with one address.
5. A completed change ends all `Anmelde-Sitzungen` except the current one.
6. A change to an address that already has a `Konto` fails with 409 and never sends a token to it;
   the change itself is not started (matches rule 2).

### Account security

Issue: #30. Prerequisite for MFA.

Rules:

1. `Passwort` change while logged in requires the current `Passwort`; the new one follows rule 2 of
   the MVP rules (at least 8 characters, no composition rules).
2. A completed change ends all `Anmelde-Sitzungen` except the current one.
3. The `Anmelde-Sitzung` list shows creation time, last use, and coarse device/browser, newest
   first, with the current session marked. It never exposes a token or its hash.
4. Revocation is immediate: the revoked session's access token fails with 401 on its next request
   (rule 15).
5. "Alle Geräte abmelden" revokes every `Anmelde-Sitzung` including the current one.
6. Password change and session revocation are audited without secrets.

### MFA

Issue: #31. Depends on email delivery and account security.

Rules:

1. MFA is TOTP. Enrollment requires `Passwort` re-entry and completes only after a valid code from
   the newly generated secret.
2. Enrollment issues ten single-use recovery codes, shown once, stored hashed, regenerable; the
   previous set is invalidated on regeneration.
3. With MFA enabled, a correct `Passwort` login yields an MFA challenge, not a usable
   `Anmelde-Sitzung`; no protected endpoint accepts the pre-challenge state.
4. A code is accepted inside a documented clock-skew window; replay of the same code inside that
   window is refused.
5. Step-up re-authentication is required for `Benutzer-Löschung`, email change, MFA disable, and
   recovery-code regeneration.
6. A `Systemadministrator` may reset MFA for a `Benutzer` after identity verification off-system; the
   reset is audited and notifies the owner by email.
7. Disabling MFA requires the current factor or a recovery code and notifies the owner by email.

Open questions:

- [ ] Q-8: Is MFA optional forever, or mandatory for `Systemadministrator`?
- [ ] Q-9: May an administrator reset MFA at all, or is the recovery-code path the only one?

### External identity providers

Issue: #32. Depends on [multiple credentials](#multiple-credentials-per-benutzer).

Rules:

1. Login uses OpenID Connect authorization code flow with PKCE. Client secrets stay server-side;
   the static frontend never receives them.
2. A first sign-in creates a `Benutzer` with one provider `Konto` and no `Passwort`, and starts a
   normal `Anmelde-Sitzung` per [ADR-0004](../decisions/0004-jwt-refresh-auth.md).
3. Linking a provider identity to an existing `Benutzer` requires proving an existing `Konto` first.
   A provider login whose address already belongs to a `Benutzer` never links silently.
4. `email_verified` is trusted only when the provider asserts it; otherwise the address follows the
   [email verification](#email-verification) rules.
5. Apple private-relay addresses are accepted, count as unverified for linking, and never merge two
   `Benutzer`.
6. Unlinking is refused when it would leave the `Benutzer` without any login method.
7. A provider outage returns a clear error and does not lock the account; the password path keeps
   working.
8. Linking and unlinking are audited with provider and subject identifier, never with tokens.

Open questions:

- [ ] Q-10: Which providers at launch, and is Apple mandatory?
- [ ] Q-11: May an existing password `Konto` add a provider, or is that new-account only?

### Passkeys

Issue: #33. Depends on account security and external identity providers.

Rules:

1. A logged-in `Benutzer` registers a passkey as an additional `Konto`; the server verifies the
   attestation and rejects a challenge it did not issue.
2. A `Benutzer` may hold several passkeys with labels; each is independently removable.
3. Login with a discoverable credential works without typing an address and creates a normal
   `Anmelde-Sitzung`.
4. Removing the last credential is refused when no `Passwort` and no provider login remains.
5. Challenge expiry and signature counters are checked; a replayed assertion is refused.

### Administration

Issue: #34. Resolves Q-2.

Rules:

1. An idempotent bootstrap (command or deployment configuration) creates or promotes exactly the
   configured `Systemadministrator` address; running it twice changes nothing.
2. No API path lets a `Benutzer` elevate its own `Rolle`.
3. Role changes are audited with actor, target, previous and new role, timestamp.
4. Demoting or deleting the last `Systemadministrator` is refused.
5. Bootstrap never writes a plaintext password into configuration, logs, or the repository.

Open questions:

- [ ] Q-12: Bootstrap mechanism (CLI subcommand, environment variable, or migration seed).

### Auth audit

Issue: #35. Depends on email delivery.

Rules:

1. Login success, login failure, MFA outcome, `Passwort` change, email change, session revocation, and
   `Benutzer-Löschung` are recorded with time, `Benutzer` id, truncated IP, and user agent.
2. A `Benutzer` reads only its own history.
3. A login from a device not seen before sends a notice to the confirmed address.
4. No `Passwort`, token, cookie value, or recovery code appears in the audit store or logs.
5. Retention is documented and enforced by a cleanup job.
6. Audit entries survive `Benutzer-Löschung` only in the anonymized form the retention policy
   defines.

Open questions:

- [ ] Q-13: Retention period, and whether IP addresses are stored at all.

### Notifications

Issue: #50. Depends on email delivery.

Rules:

1. Transactional and security mail (reset, verification, new device, MFA change) cannot be disabled.
2. Each optional mail type is a separate opt-in, off by default where consent is required, settable
   in settings and revocable there.
3. An unsubscribe link works without login via a signed, expiring token, and stops that mail type
   immediately. The token cannot change another `Benutzer`'s preferences.
4. Preferences are recorded with a timestamp and honoured across all senders; a manual unsubscribe is
   never silently re-enabled.

Open questions:

- [ ] Q-14: Which optional mail types exist at launch.

## Risks and open questions

- [x] Q-15: **Resolved 2026-10-26.** Token pair instead of an idle timeout: access JWT 24 hours,
  refresh token one month and single-use, renewal rotates both; no refresh means logged out.
- [x] Q-16: **Resolved 2026-10-26.** No per-address lockout. Throttles are client-scoped only
  (rule 5).
- [x] Q-17: **Resolved 2026-10-26.** A frontend route handles the confirmation: it receives the
  token, calls the API to validate it, and continues to the `Passwort` step. The token therefore
  never sits in an API-only URL, and the exchange keeps it out of the app's own routes.
- [x] Q-18: **Resolved 2026-10-26.** Not a question any more: registration creates a `Benutzer` only
  after the address is confirmed, so no unconfirmed `Benutzer` exists to hold an `AnlageVerwaltung`
  (rules 3, 14).
- [x] Q-19: **Resolved 2026-10-26.** The refresh token travels in an `HttpOnly`, `__Host-`,
  `SameSite=Lax` cookie scoped to the API origin; the access token stays in memory. JavaScript never
  reaches the long-lived credential, and a reload can still renew (rule 12, rule 12a, NFR-2). The
  cost is accepted: prefixes on `*.github.io` get no refresh capability and work access-token-only.
- [x] Q-20: **Resolved 2026-10-26.** Signing keys are rows, not validity windows. The key table
  records which key signed which session and whether the key is active, so retiring a key is a row
  update and needs no overlap period; only expired rows are cleaned up (rule 15).

Resolved (PO 2026-10-23): Q-1 (`Benutzer-Löschung` keeps anonymized `Runden`), Q-2 (out-of-band
`Systemadministrator` provisioning), Q-4 (exact throttle thresholds). Resolved 2026-10-26: Q-3 (SMTP
endpoint and Mailpit now, provider later), Q-7 (obsolete: verification gates registration), Q-15 to
Q-20. No open question remains in the MVP part of this spec; Q-5, Q-6, and Q-8 to Q-14 sit in
deferred sections and block only those sections, not this approval.

## Approval

`PO-approved` on 2026-10-26 by TimerErTim, after the answers to Q-15 to Q-20. Approval covers the
MVP part of this spec: the ubiquitous language, roles, journeys, rules 1–17, requirements FR-1 to
FR-11 and NFR-1 to NFR-5, and AC-1 to AC-20. It does not approve any "Deferred capabilities" section,
which stays `Draft` per its own header. The delivery issue is #69.

## Verification

API e2e tests in the backend API test suite (`tests/e2e/`), one case per AC (AC-1 to AC-20 at the
API level, using the throttle thresholds of rule 5). AC-17 and AC-18 get explicit token tests: an
unexpired token after revocation, an unknown signing key, and an expired access token. AC-19 runs
against Mailpit started by the test harness. Domain rules (normalization, token lifecycle,
anonymization, authorisation decisions) get domain unit tests in the backend core. Delivery: issue
#69.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Login method: email + password. | TimerErTim |
| 2026-10-23 | GDPR: self-service account deletion only; no data export in MVP. | TimerErTim |
| 2026-10-23 | Q-1: `Benutzer-Löschung` deletes the `Konto`, credentials, and PII but keeps `Runden` and `Schüsse` anonymized so aggregate statistics survive. | TimerErTim (PO) |
| 2026-10-23 | Q-2: `Systemadministrator` accounts are provisioned out of band; no self-service elevation (tracked in issue #34). | TimerErTim (PO) |
| 2026-10-23 | Q-4: Throttle thresholds fixed exactly; configurability tracked in issue #26. | TimerErTim (PO) |
| 2026-10-23 | Registration answers generically (review finding 1): duplicate registration notifies the existing `Konto` instead of failing. **Superseded 2026-10-26** by the address-first flow. | Agent (MiMo) |
| 2026-10-23 | Deferred capabilities written as Draft sections with their delivery issues; MVP scope unchanged. | Agent (Cursor) |
| 2026-10-26 | Registration itself starts the first `Anmelde-Sitzung`. **Superseded** in form by the two-step flow, which keeps the same property at step 2. | TimerErTim (PO) |
| 2026-10-26 | Registration discloses account existence: a taken `E-Mail-Adresse` answers 409. **Reversed** by the address-first flow of 2026-10-26 below, which discloses nothing. | TimerErTim (PO) |
| 2026-10-26 | Account roles reduced to `Benutzer` and `Systemadministrator`; authority inside an `Anlage` is an `AnlageVerwaltung` (`Administrator`, `Mitarbeiter`) stored in the `Anlage`. Authorisation lives in the use cases, not in a permission table (rules 16, 17). | TimerErTim (PO) |
| 2026-10-26 | `E-Mail-Adresse-Bestätigung` is in MVP and gates registration: address first, `Passwort` second, `Benutzer` created only on success (rules 3, 4, 14). A taken address is silently swallowed, reversing the 409 decision and removing the need for a notice email. | TimerErTim (PO) |
| 2026-10-26 | Mail goes through a configured SMTP endpoint, Mailpit in development and tests; the production provider is the follow-up (FR-11, AC-19, #27). | TimerErTim (PO) |
| 2026-10-26 | Q-15: JWT access token valid 24 hours, single-use refresh token valid one month, renewal rotates both, no renewal means logged out. Replaces the idle-timeout session model. | TimerErTim (PO) |
| 2026-10-26 | Q-16: no per-address lockout; throttles are client-scoped only (rule 5). | TimerErTim (PO) |
| 2026-10-26 | Q-17: a frontend route handles the confirmation API call. | TimerErTim (PO) |
| 2026-10-26 | Revocation is a server-side check: every request validates the access token's signing key and its session's revoked state (rule 15). | TimerErTim (PO) |
| 2026-10-26 | `Benutzer` is the logical identity, `Konto` is one way to authenticate it. German term for the per-`Anlage` relationship is `AnlageVerwaltung`, with roles `Administrator` and `Mitarbeiter`, no `Anlagen-` prefix on the role names. | TimerErTim (PO) |
| 2026-10-26 | Q-19: the refresh token travels in an `HttpOnly` `__Host-` cookie scoped to the API origin; the access token stays in memory. Cross-site previews get no refresh capability. | TimerErTim (PO) |
| 2026-10-26 | Q-20: signing keys are rows; the key table records which key signed which session and whether it is active. Retiring a key is a row update with no overlap period. | TimerErTim (PO) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #9 and PO decisions. | Agent (MiMo) |
| 2026-10-23 | PO decisions Q-1/Q-2/Q-4; registration enumeration oracle closed; data-model and API sketches moved to ADR-0002; review fixes. | Agent (MiMo) |
| 2026-10-23 | Added the "Deferred capabilities" part with one Draft section per deferred requirement. | Agent (Cursor) |
| 2026-10-26 | Registration starts the session; 409 on a taken address; 30-day idle timeout; address confirmation in MVP. | Agent (Cursor) |
| 2026-10-26 | Restructured to `Benutzer` + `Konto`; two-step address-first registration replaces all unconfirmed-state rules (AC-1 to AC-4, AC-20); session model replaced by a JWT access token plus a single-use rotating refresh token with server-side revocation (rules 12, 15, AC-7 to AC-9, AC-17, AC-18); throttles client-scoped only (rule 5, AC-15, AC-16); `Mitgliedschaft` renamed to `AnlageVerwaltung` with roles `Administrator` and `Mitarbeiter`; ADR-0002 replaced by ADR-0004. | Agent (Cursor) |
| 2026-10-26 | Q-19 and Q-20 resolved: refresh token in an `HttpOnly` `__Host-` cookie, access token in memory (rule 12, rule 12a, AC-7a); signing keys are rows with an active flag, retired by row update (rule 15). Status set to `PO-approved`. | TimerErTim (PO) |
| 2026-10-26 | Per-`Anlage` relationship renamed from `Verwaltung` to `AnlageVerwaltung`, so the term itself names its scope. Role names stay `Administrator` and `Mitarbeiter`. | TimerErTim (PO) |
| 2026-10-26 | Recorded the product/code vocabulary split in the ubiquitous language: product nouns stay German, auth-technical terms are English (`Session`, `AccessToken`, `RefreshToken`, `SecretToken`, `TokenHash`, `SigningKey`, `PasswordHash`), one concept per row. No rule or AC changed. | Agent (Cursor) |
