---
id: SPEC-datenschutz
title: Data protection, privacy, and legal obligations
status: Draft
owner: TimerErTim
issues: [36, 37, 38, 39, 40, 41, 42, 43, 51]
superseded_by: null
---

# Datenschutz — Data protection, privacy, and legal obligations

## Problem

logandaim.at processes personal data of Austrian archers: an `E-Mail-Adresse`, a `Passwort` hash,
login metadata, and a sporting history of `Runden` and `Schüsse`. Austria is an EU member state, so
GDPR applies in full. The accounts spec ([`konto.md`](konto.md)) covers only self-service deletion;
the remaining obligations — information, consent, export, retention, processors, and breach
handling — are not specified anywhere. The product cannot go public without them.

## Outcome

An archer can read what happens to their data, accept it knowingly at registration, download it,
and have it deleted on request. The operator has a documented, reviewable answer for every GDPR
duty, including the ones that only matter during an incident.

This spec is `Draft`. It records obligations and open questions; it does not invent legal advice.
Items marked Q- are blocking for the section they appear in.

## Roles

- [`Schütze`](charter.md#roles): data subject. Reads the notices, gives consent, exports and deletes
  data.
- [`System-Administrator`](charter.md#roles): executes data-subject requests and incident steps.
- Operator (the PO, not a product role): controller under GDPR, accountable for the record of
  processing, the processors, and breach reporting.

## Scope

**In scope**

- Information duties and the legal pages.
- Consent, terms acceptance, and the age gate.
- Self-service data export and deletion propagation.
- Retention per store.
- Record of processing, data processing agreements, and residency.
- Data-subject rights handling beyond self-service.
- Breach and incident response.
- Analytics consent.
- Privacy design for future cross-user aggregates.

**Out of scope**

- Employment, tax, and payment data: the product takes no payments.
- Operator (`Anlagenbetreiber`) data processing for their own purposes; a later milestone needs its
  own controller/processor analysis.
- Marketing consent.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Verantwortlicher` | Controller: the operator of logandaim.at. |
| `Auftragsverarbeiter` | Processor acting on documented instructions (hosting, mail, backups). |
| `Einwilligung` | Consent: a recorded, specific, revocable permission. |
| `Betroffenenanfrage` | Data-subject request: access, rectification, erasure, restriction, objection, portability. |
| `Datenschutzverletzung` | Personal-data breach, potentially reportable within 72 hours. |
| `Aufbewahrungsfrist` | Retention period for one store. |
| `Datenauskunft` | Machine-readable export of all personal data of one `Konto`. |

Rules:

1. Personal data is processed only for the purposes named in the privacy notice.
2. No personal data is transferred to a processor without a recorded agreement.
3. A retention period exists for every store that holds personal data, and expiry is enforced by
   code, not by intention.
4. Consent is opt-in, specific, and as easy to withdraw as to give.
5. Nothing is published to third parties in the MVP; there is no user-to-user visibility.
6. Anonymized records (see [`konto.md`](konto.md) rule 8) are not personal data and are governed by
   the retention table like any other store.
7. Legal text changes are versioned; acceptance refers to a version that stays retrievable.

## Data export

Issue: #36.

Rules:

1. The export contains the `Konto` record, all `Runden`, all `Schüsse`, session metadata without
   tokens, recorded consents, and auth audit entries owned by the `Konto`.
2. The archive contains JSON for full fidelity and a CSV of `Runden` for spreadsheet use, both with
   a documented schema.
3. Generation is asynchronous; the download link is single-purpose and expires after 24 hours.
4. Requesting an export requires a fresh re-authentication and is audited.
5. The export never contains another `Konto`'s data.
6. The export is reachable from account settings and does not depend on the mail channel.
7. Generated archives are deleted after expiry and are not stored at a publicly reachable path.

Open questions:

- [ ] Q-15: Is a human-readable PDF summary also required?
- [ ] Q-16: Retention of a generated archive before it expires.

## Retention

Issue: #37.

Rules:

1. Every store that holds personal data appears in the retention table below with a period and a
   basis.
2. `Konto-Löschung` removes or irreversibly anonymizes rows in every personal store. One test
   inspects each store after deletion.
3. Deleted data leaves backups within the documented backup window; the window and the restore
   caveat are stated in the privacy notice.
4. Expired sessions, tokens, and audit entries are removed by a scheduled cleanup that is observable
   and alerts on failure.
5. Third-party copies (mail suppression lists, error tracking) are listed with their retention and
   their erasure path.
6. Anonymized aggregates that may be kept are named, and re-identification is argued against here.

Draft retention table. Periods are Q-17 and must be confirmed by the PO.

| Store | Content | Draft period | Basis |
|-------|---------|--------------|-------|
| `konten` | credentials, role | until deletion | contract |
| `anmelde_sitzungen` | session metadata | 30 days idle, plus absolute cap | security |
| recovery and verification tokens | hashed tokens | until expiry or use | security |
| auth audit | login metadata | 90 days | security |
| application logs | technical logs | 30 days | operations |
| mail provider records | delivery metadata | provider-side, per agreement | operations |
| backups | full database | 30 days rolling | recovery |
| anonymized `Runden`/`Schüsse` | sporting results | indefinite | not personal data |

Open questions:

- [ ] Q-17: Confirm or adjust every period in the table.
- [ ] Q-18: Confirm that anonymized `Runden` are kept indefinitely.

## Consent

Issue: #38.

Rules:

1. Registration requires accepting the current terms and privacy versions; without acceptance no
   `Konto` is created.
2. The accepted document versions, the timestamp, and the requesting IP are stored with the
   `Konto`.
3. A new version requires re-acceptance at the next login. Declining blocks further use and offers
   account deletion.
4. Optional consents are unchecked by default, listed separately, and revocable in account settings
   without losing access.
5. A minimum age is enforced, and the chosen handling (parental consent or hard block) is specified
   and tested.
6. Granting and withdrawing any consent is recorded with timestamp and document version; withdrawal
   applies to future processing.

Open questions:

- [ ] Q-19: Minimum age and whether parental consent is supported at all.
- [ ] Q-20: Which optional consents exist at launch.

## Legal pages

Issue: #39. Depends on consent.

Rules:

1. An Impressum per ECG §5 and MedienG §25 and a Datenschutzerklärung per GDPR Art 13/14 are
   reachable from every page, including logged-out pages.
2. Terms of use are published with an effective date and a version identifier.
3. The privacy notice names purposes, data categories, recipients and processors, transfers,
   retention periods, user rights, and the complaint path to the Österreichische
   Datenschutzbehörde.
4. Previous document versions stay retrievable; acceptance records refer to a resolvable version.
5. Pages are static-export compatible, readable without JavaScript, and legible on a phone in
   sunlight.

Open questions:

- [ ] Q-21: Legal entity, address, and contact details for the Impressum.
- [ ] Q-22: Are terms of use needed at all for a free logging app?

## Rights process

Issue: #40.

Rules:

1. One intake channel, one owner, and an identity-verification step before any data is released are
   named here.
2. A request log records receipt date, type, deadline, action taken, and response date.
3. Handling is described for access, rectification, erasure, restriction, objection, and
   portability; self-service paths are referenced where they exist.
4. Template replies exist for granting, partially granting, and refusing, including the complaint
   path.
5. The process states what happens when the one-month deadline cannot be met, and who is informed.

Open questions:

- [ ] Q-23: Responsible person and the published contact channel.

## Processors

Issue: #41.

Rules:

1. A record of processing per Art 30 covers account management, scoring data, auth audit, logs,
   mail delivery, and backups, each with purpose, categories, recipients, retention, and technical
   measures.
2. A published sub-processor list names each vendor with role, region, and data categories.
3. A data processing agreement is recorded for hosting, mail, analytics, and backups. A missing
   agreement is a launch blocker, listed as such.
4. Where processing leaves the EU, the transfer mechanism is documented; otherwise EU-only hosting
   is stated explicitly.
5. Adding a vendor requires an entry here and, for a new category of data, a PO decision.
6. The record is reviewed on a stated cadence and the review is logged.

Open questions:

- [ ] Q-24: Final provider list (hosting, mail, backups, analytics) and whether any provider is
  outside the EU.

## Analytics

Issue: #42. Depends on consent.

Rules:

1. No analytics script, non-essential cookie, or third-party request loads before affirmative
   consent.
2. Consent is revocable, persists across visits, and refusal is as easy as acceptance.
3. Measurement is cookieless or self-hosted where feasible; otherwise the trade-off is recorded
   here.
4. IP addresses are truncated before storage and no cross-site profile is built.
5. The session cookie is classified as strictly necessary and is not gated behind consent.
6. A fresh browser without consent makes no analytics request on the core pages.

Open questions:

- [ ] Q-25: Is analytics needed at launch, and which metric decides success?

## Incident response

Issue: #43.

Rules:

1. A triage tree distinguishes a security incident from a reportable `Datenschutzverletzung`, and
   the decision is recorded.
2. Containment steps are written per component (API, database, frontend, mail provider, VPS
   access), naming the commands or dashboards.
3. Evidence preservation is specified before any cleanup.
4. Notification templates exist for the Datenschutzbehörde and for affected users, with
   placeholders for scope, data categories, and measures.
5. Named roles, contacts, and an out-of-band contact path are listed.
6. A timeline template exists and a tabletop exercise is recorded once.

Open questions:

- [ ] Q-26: Incident owner and the published channel for security reports.

## Benchmarks

Issue: #51. Depends on export and consent.

Cross-user comparisons are a charter non-goal for the MVP. This section exists so they can ship
later without a retroactive privacy change.

Rules:

1. Participation is an explicit opt-in with a plain-language explanation of what becomes visible
   and to whom.
2. Aggregates are produced only above a documented minimum cohort size.
3. No filter combination or repeated querying isolates one archer's values.
4. Withdrawal removes the user from future aggregates; what happens to already published aggregates
   is stated here.
5. Aggregates are stored separately from personal `Runden` and appear in the retention table.
6. The privacy notice describes benchmarks in the same terms as the opt-in text.

Open questions:

- [ ] Q-27: Are opt-in benchmarks wanted at all, and what is the minimum cohort size?

## Risks and open questions

Open questions are listed inside each section, where they block approval. Summary:

Q-15 and Q-16 (export), Q-17 and Q-18 (retention), Q-19 and Q-20 (consent), Q-21 and Q-22 (legal
pages), Q-23 (rights process), Q-24 (processors), Q-25 (analytics), Q-26 (incident), Q-27
(benchmarks).

## Verification

Spec review against a GDPR checklist, one item per rule. Later implementation verification is named
in each issue: retention and export need database-level tests, consent and legal pages need browser
checks, processors and incident response are document reviews.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | GDPR: self-service account deletion only; no data export in MVP. | TimerErTim |
| 2026-10-23 | Deletion keeps `Runden`/`Schüsse` anonymized for aggregate statistics. | TimerErTim |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from the deferred-requirement tickets #36–#43 and #51. | Agent (Cursor) |
