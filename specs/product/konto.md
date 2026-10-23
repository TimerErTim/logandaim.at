---
id: SPEC-konto
title: Accounts and login (registration, login, recovery, deletion, roles)
status: Draft
owner: TimerErTim
issues: [9, 27, 28, 29, 30, 31, 32, 33, 34, 35, 50]
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

**Out of scope for MVP**

Everything below is deferred by PO decision, not rejected. Each item has its own Draft section
further down and a delivery issue; nothing here is implementable until that section is
`PO-approved`.

- Email verification at registration (PO decision 2026-10-23; [later](#email-verification), #28).
- Changing the account email ([later](#email-change), #29).
- Changing the password while logged in and session management ([later](#account-security), #30).
- OAuth or other login providers (PO decision 2026-10-23; [later](#external-identity-providers), #32).
- 2FA / TOTP ([later](#mfa), #31) and passkeys ([later](#passkeys), #33).
- Authentication audit log and new-device notices ([later](#auth-audit), #35).
- Notification preferences ([later](#notifications), #50).
- GDPR data export (PO decision 2026-10-23; see `datenschutz.md`, #36).
- Password strength meter UI and "remember me" options.
- Operator accounts (`Anlagenbetreiber`) and tournament roles.

## User journeys

1. **Register.** The `Schütze` opens the app, enters email and password, and is logged in
   immediately — no verification email.
2. **Login.** The returning `Schütze` logs in with email and password and lands on the home screen.
3. **Logout.** The `Schütze` logs out; the device forgets the login and further API calls fail.
4. **Reset password.** The `Schütze` who forgot the password requests a reset, receives an email
   link, sets a new password, and logs in; older logins on other devices end.
5. **Delete account.** In account settings the `Schütze` re-enters the password and confirms; the
   `Konto` and all personal data are gone, its `Runden` stay anonymized for aggregate statistics,
   and the session ends.
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
| `Konto-Löschung` | Irreversible self-service removal of the `Konto` and its personal data; its `Runden` stay anonymized. |
| `Anonymisierte Runde` | A `Runde` (with its `Schüsse`) whose link to a `Konto` and whose personal attributes are stripped. Kept for aggregate statistics; no longer personal data. |

Rules:

1. A `Konto` is identified by its normalized `E-Mail-Adresse`, which is unique across all `Konto`.
2. A `Passwort` has at least 8 characters. No composition rules and no periodic rotation.
3. Registration creates a `Konto` with `Rolle` `Schütze` and logs the user in immediately.
   No email verification happens (MVP).
4. Registration, login, and recovery never reveal to third parties whether a `Konto` exists.
   Login failure gives one generic error, whether the email is unknown or the `Passwort` is wrong.
   Registration answers with one identical response whether or not the `E-Mail-Adresse` is taken:
   a taken address creates no `Konto` and no `Anmelde-Sitzung`, and the existing `Konto` receives
   a notice email about the attempted registration instead. The difference shows up only in the
   follow-up (a login succeeds only for the freshly created `Konto`; the notice email goes only to
   an existing one) — never in the registration response itself.
5. Throttles are fixed exactly (Q-4; configurability tracked in issue #26). Per `E-Mail-Adresse`:
   5 consecutive failed logins lock further attempts for 15 minutes (a successful login resets the
   counter); 3 `Passwort-Wiederherstellung` requests per hour. Per client: 20 failed logins per
   15 minutes; 5 recovery requests per hour; 5 registration attempts per hour. Exceeding a
   throttle fails with 429 until the window passes.
6. A `Konto` may hold several `Anmelde-Sitzungen` (multiple devices). Logout ends the current
   `Anmelde-Sitzung`. A completed `Passwort-Wiederherstellung` ends all `Anmelde-Sitzungen`.
7. A `Passwort-Wiederherstellung` token is single-use, bound to one `Konto`, and expires 60 minutes
   after it was requested. Requesting again invalidates the previous token.
8. `Konto-Löschung` requires re-entering the `Passwort` and explicit confirmation. It removes the
   `Konto`, its credentials, all its `Anmelde-Sitzungen`, and every personal attribute — including
   the link from its `Runden` and `Schüsse` to the `Konto` (Q-1). The `Runden` and `Schüsse` stay
   as `Anonymisierte Runde` records for aggregate statistics. Anonymization contract: kept are the
   sporting results, dates, and master-data references; stripped are the `Konto` reference and
   every personal attribute, so nobody can attribute a kept `Runde` to a person. It is
   irreversible. Master data (Anlage, Parcours, Ziel, Pflock) is untouched. The freed
   `E-Mail-Adresse` can register again.
9. Role elevation never happens through self-service. `System-Administrator` accounts are
   provisioned out of band (decision log, Q-2; tracked in issue #34).
10. Personal data is visible only to the owning `Konto`; no user-to-user visibility in MVP
    (charter Q-3 default).
11. Every request for personal data or a personal action requires a logged-in `Konto`. Only
    registration, login, and password recovery work while logged out.

## Requirements

**Functional**

- FR-1: Register a `Konto` from email + `Passwort` and get the user logged in immediately, without
  email verification. The registration response itself is generic (rule 4); the `Anmelde-Sitzung`
  starts with the login step that immediately follows registration.
- FR-2: Reject a `Passwort` shorter than 8 characters with 400 — identically whether or not the
  `E-Mail-Adresse` already has a `Konto` — without creating a `Konto`. An already-used
  `E-Mail-Adresse` creates nothing and gets the generic registration response of rule 4; the
  existing `Konto` receives a notice email.
- FR-3: Log in with email + `Passwort`, with a generic failure for wrong credentials.
- FR-4: Log out by revoking the current `Anmelde-Sitzung` server-side.
- FR-5: Reset a forgotten `Passwort`: request with an `E-Mail-Adresse` always returns the same
  response; an existing `Konto` receives a `Passwort-Wiederherstellung` email; completing it sets
  the new `Passwort` and ends all `Anmelde-Sitzungen`.
- FR-6: Delete the own `Konto` with `Passwort` re-entry and confirmation, removing the `Konto`,
  its credentials, and all sessions and personal data; its `Runden` and `Schüsse` stay anonymized
  per rule 8.
- FR-7: Enforce `Rolle`-based access: a `Schütze` accesses only the own data, a
  `System-Administrator` additionally the admin endpoints. No self-service elevation.
- FR-8: Enforce the throttle thresholds of rule 5 for failed logins, recovery requests, and
  registration bursts, per `E-Mail-Adresse` and per client.

**Non-functional**

- NFR-1: `Passwort` values are stored as salted hashes (argon2id) and never returned or logged.
- NFR-2: The session credential is not readable from JavaScript and is transmitted only over
  HTTPS (see [`../decisions/0002-session-cookie-auth.md`](../decisions/0002-session-cookie-auth.md)).
- NFR-3: Login works from the deployed frontend origin despite the cross-origin deployment
  (see [`../decisions/0002-session-cookie-auth.md`](../decisions/0002-session-cookie-auth.md)).
- NFR-4: Auth endpoints respond within 500 ms p95, excluding email delivery.

## Implementation sketches

Non-normative implementation sketches (data model, API surface) are technical design and live in
[ADR-0002's appendix](../decisions/0002-session-cookie-auth.md#appendix-implementation-sketch-non-normative),
not in this spec. They guide implementation and are not part of the contract; the contract lives
in the rules and acceptance criteria above.

## Acceptance criteria

- AC-1: Given no `Konto` for an email, when registering with that email and an 8-character
  `Passwort`, then the registration response is the generic response of rule 4, a `Konto` with
  `Rolle` `Schütze` exists, and the user is logged in without any verification step.
- AC-2: When registering with an `E-Mail-Adresse` that already has a `Konto`, then the response
  (status and body) is identical to AC-1's registration response, no `Konto` and no
  `Anmelde-Sitzung` are created, the existing `Konto` receives a notice email about the attempted
  registration, and a follow-up login with the submitted `Passwort` fails with the one generic
  login error of rule 4.
- AC-3: When registering or resetting with a `Passwort` shorter than 8 characters, then the request
  fails with 400 and the stored `Passwort` is unchanged — identically whether or not the
  `E-Mail-Adresse` has a `Konto`.
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
  its credentials, its `Anmelde-Sitzungen`, and all personal data are gone; its `Runden` and
  `Schüsse` remain as anonymized records that still count in aggregate statistics but no longer
  reference the deleted `Konto`; login with the old credentials fails; and the `E-Mail-Adresse` can
  register again.
- AC-10: When an unauthenticated request hits any protected endpoint, then it fails with 401.
- AC-11: When a `Schütze` requests an admin-only endpoint, then it fails with 403; the same
  request with a `System-Administrator` `Konto` succeeds.
- AC-12: After 5 consecutive failed logins for one `E-Mail-Adresse`, further attempts for it fail
  with 429 for 15 minutes, and after 20 failed logins per client within 15 minutes further attempts
  from that client fail with 429 until the window passes; a successful login resets the
  `E-Mail-Adresse` counter.
- AC-13: The session credential is not readable from JavaScript in the browser and is only
  transmitted over HTTPS.
- AC-14: When `Passwort-Wiederherstellung` requests exceed 3 per `E-Mail-Adresse` per hour or 5 per
  client per hour, the excess requests fail with 429 until the window passes.
- AC-15: When registration attempts exceed 5 per client per hour, the excess requests fail with 429
  until the window passes.

## Deferred capabilities

The capabilities in this part are **Draft** and not part of the MVP contract above. Each one has a
delivery issue referenced in its heading. They are written now so the deferred requirements are
specified somewhere and do not get relitigated from scratch. Open questions block approval of the
individual section only, not of the MVP part.

### Email delivery

Issue: #27. Resolves Q-3.

Every deferred capability except passkeys depends on an outbound mail channel: verification,
change notices, MFA and device notices, unsubscribe. The MVP reset flow already needs it.

Rules:

1. Outbound mail goes through exactly one configured provider; the sender domain has SPF, DKIM,
   and DMARC records that pass.
2. A message body containing a token is never retained longer than the token lifetime, in the
   provider or in application logs.
3. Hard bounces and complaints land on a suppression list that blocks further sends to that
   address.
4. Development and tests send only to a local mail catcher; no test path can reach a real address.
5. Provider region, data processing agreement, and mail-log retention are recorded in
   `datenschutz.md#processors`.

Open questions:

- [ ] Q-5: Provider and sending domain.
- [ ] Q-6: Is the mail channel a launch gate, or only required before password reset ships?

### Email verification

Issue: #28. Depends on email delivery.

Rules:

1. Registration sends a `E-Mail-Adresse-Bestätigung` with a single-use token, valid 24 hours,
   stored hashed.
2. Confirming a token marks the address verified. It does not create a second `Anmelde-Sitzung` and
   does not change the current one.
3. An unverified `Konto` may read but may not create or modify `Runden`.
4. A resend is throttled per `Konto` and per client; confirming an already-verified address is
   idempotent and sends no mail.
5. `Konten` that exist before this ships follow the transition rule of Q-7.

Open questions:

- [ ] Q-7: Enforcement (hard block on writing) and the fate of pre-existing accounts.

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
6. A change to an address that already has a `Konto` answers generically and never reveals that the
   address is taken.

### Account security

Issue: #30. Prerequisite for MFA.

Rules:

1. `Passwort` change while logged in requires the current `Passwort`; the new one follows rule 2 of
   the MVP rules (at least 8 characters, no composition rules).
2. A completed change ends all `Anmelde-Sitzungen` except the current one.
3. The `Anmelde-Sitzung` list shows creation time, last use, and coarse device/browser, newest
   first, with the current session marked. It never exposes a raw token or its hash.
4. Revocation is immediate: the revoked session fails with 401 on its next request.
5. "Alle Geräte abmelden" revokes every session including the current one.
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
5. Step-up re-authentication is required for `Konto-Löschung`, email change, MFA disable, and
   recovery-code regeneration.
6. A `System-Administrator` may reset MFA for a `Konto` after identity verification off-system; the
   reset is audited and notifies the owner by email.
7. Disabling MFA requires the current factor or a recovery code and notifies the owner by email.

Open questions:

- [ ] Q-8: Is MFA optional forever, or mandatory for `System-Administrator`?
- [ ] Q-9: May an administrator reset MFA at all, or is the recovery-code path the only one?

### External identity providers

Issue: #32. Depends on email verification.

Rules:

1. Login uses OpenID Connect authorization code flow with PKCE. Client secrets stay server-side;
   the static frontend never receives them.
2. A first sign-in creates a `Konto` with the provider's verified address and no local `Passwort`,
   and starts a normal `Anmelde-Sitzung` per [ADR-0002](../decisions/0002-session-cookie-auth.md).
3. Linking a provider identity to an existing `Konto` requires proving the local credential first.
   A provider login whose address already has a `Konto` never links silently.
4. `email_verified` is trusted only when the provider asserts it; otherwise the address follows the
   [email verification](#email-verification) rules.
5. Apple private-relay addresses are accepted, count as unverified for linking, and never merge two
   `Konto`.
6. Unlinking is refused when it would leave the `Konto` without any login method.
7. A provider outage returns a clear error and does not lock the account; the password path keeps
   working.
8. Linking and unlinking are audited with provider and subject identifier, never with tokens.

Open questions:

- [ ] Q-10: Which providers at launch, and is Apple mandatory?
- [ ] Q-11: May an existing password `Konto` add a provider, or is that new-account only?

### Passkeys

Issue: #33. Depends on account security and external identity providers.

Rules:

1. A logged-in `Konto` registers a passkey; the server verifies the attestation and rejects a
   challenge it did not issue.
2. A `Konto` may hold several passkeys with labels; each is independently removable.
3. Login with a discoverable credential works without typing an address and creates a normal
   `Anmelde-Sitzung`.
4. Removing the last credential is refused when no `Passwort` and no provider login remains.
5. Challenge expiry and signature counters are checked; a replayed assertion is refused.

### Administration

Issue: #34. Resolves Q-2.

Rules:

1. An idempotent bootstrap (command or deployment configuration) creates or promotes exactly the
   configured `System-Administrator` address; running it twice changes nothing.
2. No API path lets a `Konto` elevate its own `Rolle`.
3. Role changes are audited with actor, target, previous and new role, timestamp.
4. Demoting or deleting the last `System-Administrator` is refused.
5. Bootstrap never writes a plaintext password into configuration, logs, or the repository.

Open questions:

- [ ] Q-12: Bootstrap mechanism (CLI subcommand, environment variable, or migration seed).

### Auth audit

Issue: #35. Depends on email delivery.

Rules:

1. Login success, login failure, MFA outcome, `Passwort` change, email change, session revocation,
   and `Konto-Löschung` are recorded with time, `Konto` id, truncated IP, and user agent.
2. A `Konto` reads only its own history.
3. A login from a device not seen before sends a notice to the verified address.
4. No `Passwort`, token, cookie value, or recovery code appears in the audit store or logs.
5. Retention is documented and enforced by a cleanup job.
6. Audit entries survive `Konto-Löschung` only in the anonymized form the retention policy defines.

Open questions:

- [ ] Q-13: Retention period, and whether IP addresses are stored at all.

### Notifications

Issue: #50. Depends on email delivery.

Rules:

1. Transactional and security mail (reset, verification, new device, MFA change) cannot be disabled.
2. Each optional mail type is a separate opt-in, off by default where consent is required, settable
   in account settings and revocable there.
3. An unsubscribe link works without login via a signed, expiring token, and stops that mail type
   immediately. The token cannot change another `Konto`'s preferences.
4. Preferences are recorded with a timestamp and honoured across all senders; a manual unsubscribe
   is never silently re-enabled.

Open questions:

- [ ] Q-14: Which optional mail types exist at launch.

## Risks and open questions

- [ ] Q-3: Password recovery needs an outbound email channel even though registration needs no
  verification. Which email delivery for MVP (provider, sender domain)? (PO 2026-10-23: decide
  later; tracked in issue #27.)

Resolved (PO 2026-10-23): Q-1 (`Konto-Löschung` keeps anonymized `Runden`), Q-2 (out-of-band
`System-Administrator` provisioning), Q-4 (exact throttle thresholds in rule 5). See the decision
log.

## Verification

API e2e tests in the backend API test suite (`tests/e2e/`), one case per AC (AC-1 to AC-15 at the
API level, using the throttle thresholds of rule 5). AC-13 additionally as a manual browser
check on the deployed frontend origin. Domain rules (normalization, token lifecycle, anonymization)
get domain unit tests in the backend core.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Login method: email + password. | TimerErTim |
| 2026-10-23 | Email verification not required for MVP. | TimerErTim |
| 2026-10-23 | GDPR: self-service account deletion only; no data export in MVP. | TimerErTim |
| 2026-10-23 | Q-1: `Konto-Löschung` deletes the `Konto`, credentials, and PII but keeps `Runden` and `Schüsse` anonymized so aggregate statistics survive (reverses the draft's hard-delete default). | TimerErTim (PO) |
| 2026-10-23 | Q-2: `System-Administrator` accounts are provisioned out of band; no self-service elevation (tracked in issue #34). | TimerErTim (PO) |
| 2026-10-23 | Q-4: Throttle thresholds fixed exactly in rule 5; configurability tracked in issue #26. | TimerErTim (PO) |
| 2026-10-23 | Registration answers generically (rule 4, review finding 1): duplicate registration notifies the existing `Konto` instead of failing with 409. | Agent (MiMo) |
| 2026-10-23 | Deferred capabilities (email delivery, verification, email change, account security, MFA, external identity providers, passkeys, administration, auth audit, notifications) written as Draft sections with their delivery issues; MVP scope unchanged. | Agent (Cursor) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #9 and PO decisions. | Agent (MiMo) |
| 2026-10-23 | PO decisions Q-1/Q-2/Q-4 (anonymized `Runden` on deletion, out-of-band admins, exact throttle thresholds); registration enumeration oracle closed; data-model and API sketches moved to ADR-0002; review fixes. | Agent (MiMo) |
| 2026-10-23 | Added the "Deferred capabilities" part with one Draft section per deferred requirement, each anchored to its delivery issue; former out-of-scope bullets now link here. MVP contract text unchanged. | Agent (Cursor) |
