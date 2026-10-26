---
id: SPEC-betrieb
title: Operations and launch readiness (security, backups, abuse, support)
status: Draft
owner: TimerErTim
issues: [44, 45, 46, 47]
superseded_by: null
---

# Betrieb — Operations and launch readiness

## Problem

The product runs as a static frontend plus a Rust API and SQLite database on a VPS
([ADR-0001](../decisions/0001-baseline-architecture.md)). The architecture decision covers the
shape, and the deployment issues cover getting it there. Nothing specifies what must be true for a
public launch beyond "it deploys": security headers, backups, a restore procedure, alerting, abuse
handling, and a support path. Without this, an outage or a lost database has no defined response.

## Outcome

A public launch has a reviewed security baseline, a recovery procedure that has been executed, an
alert that reaches a human, and a support path a user can find. Each is checkable, not aspirational.

## Roles

- [`Systemadministrator`](charter.md#roles): suspends abusive accounts, executes runbooks.
- Operator (the PO, not a product role): owns the VPS, the backup target, and the alert channel.
- [`Benutzer`](charter.md#roles): needs support and a reachable help path.

## Scope

**In scope**

- Security hardening and a pre-launch assessment.
- Backups, a restore drill, and alerting.
- Account suspension and abuse handling.
- Support channel and help content.

**Out of scope**

- Deployment mechanics (issues #14, #15, #24).
- On-call rotation, SLO commitments, incident management tooling.
- Compliance certification.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Wiederherstellungsziel` | RTO/RPO: how long recovery may take and how much data may be lost. |
| `Sperrung` | Suspension: a `Benutzer` cannot log in, data is retained. |
| `Sicherheitsbaseline` | The set of headers, scanning, and rate limits the deployment must satisfy. |

Rules:

1. A launch gate is only satisfied with evidence in the issue, not with a claim in a PR.
2. A procedure that has never been executed is not a procedure.
3. Suspension is never a substitute for deletion and never destroys evidence.

## Security hardening

Issue: #44. Depends on the backend deployment (#15).

Rules:

1. The API sends HSTS, a CSP without inline script or `unsafe-eval` for the static export,
   `X-Content-Type-Options`, `Referrer-Policy`, and frame protection.
2. CI runs dependency and secret scanning for Rust and npm; a high-severity finding blocks merge.
3. Login, registration, recovery, and other costly endpoints are rate limited at the edge and in the
   API. The exact thresholds live in [`konto.md`](konto.md) rule 5.
4. `security.txt` and a disclosure contact are published with a stated response expectation.
5. A penetration test or a structured self-assessment covering auth, sessions, CORS, and the API is
   completed; every finding is closed or accepted in writing by the PO.
6. TLS configuration scores A on a common SSL test, and certificate renewal is automatic.

Open questions:

- [ ] Q-28: Paid penetration test, or documented self-assessment with a second reviewer?
- [ ] Q-29: Who receives security reports?

## Backups

Issue: #45. Depends on the backend deployment (#15).

Rules:

1. Encrypted backups run on a documented schedule to an off-host target with a stated retention.
2. A restore drill is executed and recorded with the measured duration and resulting data-loss
   window, stated as `Wiederherstellungsziel`.
3. A restored database serves a real round-trip: login, read a `Runde`, write a `Schuss`.
4. A failed backup alerts a human channel; a silent multi-day gap is impossible.
5. Certificate expiry, API 5xx rate, and disk usage alert to the same channel.
6. Backup credentials are separate from application credentials and documented in the runbook.

Open questions:

- [ ] Q-30: Backup target and retention, and the acceptable RPO.

## Abuse

Issue: #46.

Rules:

1. A `Benutzer` can be suspended and reactivated with a mandatory reason and an audit entry naming the
   actor.
2. A suspended `Benutzer` cannot log in or use the API; its `Anmelde-Sitzungen` are revoked.
3. Suspension retains data per the retention table in [`datenschutz.md`](datenschutz.md#retention)
   and is distinct from `Benutzer-Löschung`.
4. The suspended user sees an explanation and a support path, not a generic login failure.
5. An appeal path and a review expectation are documented.
6. Only a `Systemadministrator` can suspend; no `Benutzer` can suspend another `Benutzer` by any path.

Open questions:

- [ ] Q-31: Is suspension needed before launch, or only after a real abuse report?

## Support

Issue: #47.

Rules:

1. A published contact channel exists (form or documented address) with spam protection and a
   stated response expectation.
2. Help content covers registration, password reset, email verification, MFA lockout and recovery
   codes, deleting the `Konto`, and exporting data.
3. Every error screen that can block a user links to the relevant help entry.
4. Support requests collect no more personal data than necessary, and the intake channel appears in
   the processor list.
5. A defect report path exists that does not require a GitHub account.

Open questions:

- [ ] Q-32: Support address, and whether a form is hosted given the static frontend.

## Risks and open questions

Q-28 and Q-29 (hardening), Q-30 (backups), Q-31 (abuse), Q-32 (support).

## Verification

Each rule is verified at the layer named in its issue: header and TLS inspection, CI workflow
evidence, a recorded restore drill, alert tests, API end-to-end tests for suspension, and link checks
for support content.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Deferred operational requirements are specified here rather than in the deployment issues. | Agent (Cursor) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from the deferred-requirement tickets #44–#47. | Agent (Cursor) |
