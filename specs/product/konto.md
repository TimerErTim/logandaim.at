---
id: SPEC-konto
title: Accounts and login (registration, login, recovery, deletion, roles)
status: Draft
owner: TimerErTim
issues: [9]
superseded_by: null
---

# Konto — Accounts and login

## Problem

Archers need a personal `Konto` to log Runden and review their own history, and the product needs
a `System-Administrator` role to seed master data. The frontend runs as a static export on GitHub
Pages while the backend runs on a VPS (see
[`../decisions/0001-baseline-architecture.md`](../decisions/0001-baseline-architecture.md)), so
login must work across two origins with today's browser cookie and CORS constraints.

## Outcome

A `Schütze` registers with email and password, logs in and out, recovers a forgotten password, and
deletes the account without help. A `System-Administrator` additionally maintains master data.
Personal data belongs to exactly one `Konto` and is visible only to its owner.

## Roles

- [`Schütze`](charter.md#roles): registers, logs in, manages the own `Konto`, logs Runden.
- [`System-Administrator`](charter.md#roles): everything a `Schütze` can do, plus
  admin-only master-data maintenance (Anlage, Parcours, Ziel, Pflock).

## Scope

**In scope**

- Registration with email + password.
- Login and logout.
- Password reset for forgotten passwords (email link).
- Self-service account deletion.
- The `Rolle` attribute (`Schütze`, `System-Administrator`) and role-based access.

**Out of scope**

- Email verification at registration (PO decision 2026-10-23; later milestone).
- OAuth or other login providers (later milestone).
- GDPR data export (PO decision 2026-10-23; no export in MVP).
- Changing the password while logged in (the reset flow covers it).
- Password strength meter UI, 2FA, passkeys, "remember me" options.
- Operator accounts (`Anlagenbetreiber`) and tournament roles.

## User journeys

1. **Register.** The `Schütze` opens the app, enters email and password, and is logged in
   immediately — no verification email.
2. **Login.** The returning `Schütze` logs in with email and password and lands on the home screen.
3. **Logout.** The `Schütze` logs out; the device forgets the login and further API calls fail.
4. **Reset password.** The `Schütze` who forgot the password requests a reset, receives an email
   link, sets a new password, and logs in; older logins on other devices end.
5. **Delete account.** In account settings the `Schütze` re-enters the password and confirms; the
   `Konto` and all personal data are gone and the session ends.
6. **Admin.** The `System-Administrator` logs in and maintains the pilot Anlagen, Parcours, Ziele,
   and Pflöcke (master-data spec).

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Konto` | One user account, identified by its `E-Mail-Adresse`, owning personal data. |
| `E-Mail-Adresse` | Login identifier of a `Konto`. Normalized: trimmed, lowercase. |
| `Passwort` | Secret of a `Konto`. Stored only as a salted hash, never in plain text. |
| `Rolle` | Exactly one of `Schütze`, `System-Administrator` per `Konto`. |
| `Anmelde-Sitzung` | One logged-in state of a `Konto` on one device. Revocable at any time. |
| `Passwort-Wiederherstellung` | Single-use, time-limited token sent by email to set a new `Passwort`. |
| `Konto-Löschung` | Irreversible self-service removal of the `Konto` and its personal data. |

Rules:

1. A `Konto` is identified by its normalized `E-Mail-Adresse`, which is unique across all `Konto`.
2. A `Passwort` has at least 8 characters. No composition rules and no periodic rotation.
3. Registration creates a `Konto` with `Rolle` `Schütze` and logs the user in immediately.
   No email verification happens (MVP).
4. Login failure gives one generic error, whether the email is unknown or the `Passwort` is wrong.
   Registration and recovery never reveal to third parties whether a `Konto` exists.
5. Failed logins and recovery requests are throttled per `Konto` and per client.
6. A `Konto` may hold several `Anmelde-Sitzungen` (multiple devices). Logout ends the current
   `Anmelde-Sitzung`. A completed `Passwort-Wiederherstellung` ends all `Anmelde-Sitzungen`.
7. A `Passwort-Wiederherstellung` token is single-use, bound to one `Konto`, and expires 60 minutes
   after it was requested. Requesting again invalidates the previous token.
8. `Konto-Löschung` requires re-entering the `Passwort` and explicit confirmation. It removes the
   `Konto`, all its `Anmelde-Sitzungen`, and all personal data (Runden, Schüsse). It is
   irreversible. Master data (Anlage, Parcours, Ziel, Pflock) is untouched. The freed
   `E-Mail-Adresse` can register again.
9. Role elevation never happens through self-service. `System-Administrator` accounts are
   provisioned out of band (see open question Q-2).
10. Personal data is visible only to the owning `Konto`; no user-to-user visibility in MVP
    (charter Q-3 default).
11. Every request for personal data or a personal action requires a logged-in `Konto`. Only
    registration, login, and password recovery work while logged out.

## Requirements

**Functional**

- FR-1: Register a `Konto` from email + `Passwort` and start an `Anmelde-Sitzung`, without email
  verification.
- FR-2: Reject registration with an already-used `E-Mail-Adresse` or a `Passwort` shorter than
  8 characters, without creating a `Konto`.
- FR-3: Log in with email + `Passwort`, with a generic failure for wrong credentials.
- FR-4: Log out by revoking the current `Anmelde-Sitzung` server-side.
- FR-5: Reset a forgotten `Passwort`: request with an `E-Mail-Adresse` always returns the same
  response; an existing `Konto` receives a `Passwort-Wiederherstellung` email; completing it sets
  the new `Passwort` and ends all `Anmelde-Sitzungen`.
- FR-6: Delete the own `Konto` with `Passwort` re-entry and confirmation, removing all personal
  data and sessions.
- FR-7: Enforce `Rolle`-based access: a `Schütze` accesses only the own data, a
  `System-Administrator` additionally the admin endpoints. No self-service elevation.
- FR-8: Throttle failed logins and recovery requests per `Konto` and per client.

**Non-functional**

- NFR-1: `Passwort` values are stored as salted hashes (argon2id) and never returned or logged.
- NFR-2: The session credential is not readable from JavaScript and is transmitted only over
  HTTPS (see [`../decisions/0002-session-cookie-auth.md`](../decisions/0002-session-cookie-auth.md)).
- NFR-3: Login works from the deployed frontend origin despite the cross-origin deployment
  (see [`../decisions/0002-session-cookie-auth.md`](../decisions/0002-session-cookie-auth.md)).
- NFR-4: Auth endpoints respond within 500 ms p95, excluding email delivery.

## Data model (sketch)

Non-normative; the implementing issue and ADR-0002 refine it.

| Store | Fields |
|-------|--------|
| `konten` | `id`, `email` (unique, normalized), `passwort_hash`, `rolle` (`Schütze` \| `System-Administrator`), `erstellt_am` |
| `anmelde_sitzungen` | `id`, `konto_id`, `token_hash`, `erstellt_am`, `läuft_ab_am`, `zuletzt_verwendet_am` |
| `passwort_wiederherstellungen` | `id`, `konto_id`, `token_hash`, `läuft_ab_am`, `verbraucht_am` |

Session and recovery tokens are stored hashed; the raw token exists only in the cookie resp. the
recovery link. Personal data (Runden, Schüsse) references `konten.id` and is deleted with the
`Konto`.

## API surface (sketch)

Non-normative; German domain paths as in ADR-0001, JSON keys `camelCase` UTF-8.

| Endpoint | Purpose |
|----------|---------|
| `POST /api/konto/registrierung` | Register (`email`, `passwort`) → creates `Konto`, starts `Anmelde-Sitzung`. |
| `POST /api/konto/anmeldung` | Log in (`email`, `passwort`) → starts `Anmelde-Sitzung`. |
| `POST /api/konto/abmeldung` | Log out → revokes the current `Anmelde-Sitzung`. |
| `POST /api/konto/passwort-vergessen` | Request `Passwort-Wiederherstellung` email (`email`). Same response for unknown emails. |
| `POST /api/konto/passwort-wiederherstellen` | Set new `Passwort` (`token`, `neuesPasswort`) → ends all sessions. |
| `GET /api/konto` | The logged-in `Konto` (`id`, `email`, `rolle`). |
| `DELETE /api/konto` | `Konto-Löschung` (`passwort` re-entry + confirmation). |

Error shape follows the existing structured errors (400 validation, 401 not logged in,
403 wrong `Rolle`, 409 duplicate `E-Mail-Adresse`, 429 throttled).

## Acceptance criteria

- AC-1: Given no `Konto` for an email, when registering with that email and an 8-character
  `Passwort`, then a `Konto` with `Rolle` `Schütze` exists and the user is logged in without any
  verification step.
- AC-2: When registering with an `E-Mail-Adresse` that already has a `Konto`, then registration
  fails with 409 and no `Konto` is created.
- AC-3: When registering or resetting with a `Passwort` shorter than 8 characters, then the request
  fails with 400 and the stored `Passwort` is unchanged.
- AC-4: Given a `Konto`, when logging in with the correct `E-Mail-Adresse` and `Passwort`, then an
  `Anmelde-Sitzung` is established and protected requests succeed.
- AC-5: When logging in with a wrong `Passwort`, then the response equals the response for an
  unknown `E-Mail-Adresse` (401, same body), and no `Anmelde-Sitzung` is established.
- AC-6: When logging out, then the `Anmelde-Sitzung` is revoked and subsequent requests carrying
  it fail with 401.
- AC-7: When requesting `Passwort-Wiederherstellung` for any `E-Mail-Adresse`, then the response is
  identical whether or not a `Konto` exists.
- AC-8: Given a `Konto` with a recovery token inside its 60-minute validity, when setting a new
  `Passwort` with the token, then the `Passwort` changes, all `Anmelde-Sitzungen` end, and the
  token cannot be used a second time; an expired or unknown token fails with 400.
- AC-9: When deleting the account with the correct `Passwort` and confirmation, then the `Konto`,
  its sessions, and its Runden and Schüsse are gone, login with the old credentials fails, and the
  `E-Mail-Adresse` can register again.
- AC-10: When an unauthenticated request hits any protected endpoint, then it fails with 401.
- AC-11: When a `Schütze` requests an admin-only endpoint, then it fails with 403; the same
  request with a `System-Administrator` `Konto` succeeds.
- AC-12: After a configured number of consecutive failed logins for one `Konto`, further attempts
  fail with 429 until the throttle window passes.
- AC-13: The session credential is not readable from JavaScript in the browser and is only
  transmitted over HTTPS.

## Risks and open questions

- [ ] Q-1: `Konto-Löschung` semantics. Draft: hard delete of all personal data. Keep anonymized
  Runden for future statistics instead? (Charter puts global benchmarks behind a privacy design,
  which favors full delete.)
- [ ] Q-2: How are `System-Administrator` accounts provisioned in MVP — deployment config, CLI
  command, or manual database step? Draft assumes out-of-band provisioning.
- [ ] Q-3: Password recovery needs an outbound email channel even though registration needs no
  verification. Which email delivery for MVP (provider, sender domain)?
- [ ] Q-4: Throttle thresholds (failed logins, recovery emails per hour): fixed by this spec or
  operations configuration? Draft assumes operations configuration with a tested minimum.

## Verification

API e2e tests in the backend API test suite (`tests/e2e/`), one case per AC (AC-1 to AC-12 at the
API level; throttle thresholds configured low in tests). AC-13 additionally as a manual browser
check on the deployed frontend origin. Domain rules (normalization, token lifecycle) get domain
unit tests in the backend core.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Login method: email + password. | TimerErTim |
| 2026-10-23 | Email verification not required for MVP. | TimerErTim |
| 2026-10-23 | GDPR: self-service account deletion only; no data export in MVP. | TimerErTim |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #9 and PO decisions. | Agent (MiMo) |
