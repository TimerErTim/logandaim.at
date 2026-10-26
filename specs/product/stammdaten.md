---
id: SPEC-stammdaten
title: Master data (Anlage, Parcours, Ziel, Pflock) and Anlage administration
status: Draft
owner: TimerErTim
issues: [10, 68]
superseded_by: null
---

# Stammdaten — Master data and `Anlage` administration

## Problem

Archers log Runden against a `Parcours` at an `Anlage`, so that master data has to exist before
anyone logs anything. Two questions are open. Who maintains it, given that a `Benutzer` may never
create an `Anlage`? And how does authority inside one `Anlage` work, when account-level authority
has only two values (`Benutzer`, `Systemadministrator`, see [`konto.md`](konto.md))?

The answer to both is the `AnlageVerwaltung`: authority over an `Anlage` is a row stored in the `Anlage`
naming one `Benutzer` and one role, not a role on the `Benutzer`. This spec defines the master data
itself and the `AnlageVerwaltung` that governs it.

## Outcome

A `Systemadministrator` creates an `Anlage` and seeds its master data. Inside that `Anlage`, its
`Administratoren` manage the `AnlageVerwaltungen` of others and the master data, and its `Mitarbeiter`
maintain Parcours, Ziele, and Pflöcke and read statistics. A `Benutzer` with no `AnlageVerwaltung` in an
`Anlage` sees it and its Parcours, logs Runden, and changes nothing.

## Roles

- [`Systemadministrator`](charter.md#roles): creates Anlagen, assigns the first `Administrator`, and
  may act in any `Anlage`.
- [`Administrator`](charter.md#roles): a `Benutzer` who manages one `Anlage`'s `AnlageVerwaltungen` and
  master data.
- [`Mitarbeiter`](charter.md#roles): a `Benutzer` who manages one `Anlage`'s Parcours, Ziele, and
  Pflöcke and reads its statistics.
- [`Benutzer`](charter.md#roles): logs Runden; no master-data authority unless an `AnlageVerwaltung` grants
  it.

`Administrator` and `Mitarbeiter` are roles **inside an `Anlage`**, never account roles. The role
names stay bare — "the `Anlage`'s `Administrator`" — because the `Anlage` prefix belongs to
`AnlageVerwaltung`, which already names the scope; `Anlagen-Administrator` would read as a
product-wide role.

## Scope

**In scope**

- The `Anlage` aggregate: identity, name, location, contact.
- The `AnlageVerwaltung`: which `Benutzer` holds which role in which `Anlage`.
- `Parcours`, `Ziel`, `Pflock`, and their ordering.
- Who may create, edit, and remove each of these.
- Seeding the 2–3 pilot Anlagen.

**Out of scope**

- Requesting a new `Anlage` as a `Benutzer` ([`anlage-anfrage.md`](anlage-anfrage.md), #68).
- Logging a `Runde` and the distance slider (their own specs).
- Statistics and reports beyond "an `AnlageVerwaltung` may read them".
- Deleting an `Anlage` that already has `Runden` (open question Q-4).
- Import of master data from a file (open question Q-1).

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Anlage` | Physical site of a club or commercial operator, owning its Parcours and `AnlageVerwaltungen`. |
| `AnlageVerwaltung` | A `Benutzer`'s role inside one `Anlage`: `Administrator` or `Mitarbeiter`. Belongs to the `Anlage`. |
| `Parcours` | Ordered sequence of Ziele at an `Anlage`. |
| `Ziel` | One 3D animal target with target group 1–4 and its Pflöcke. |
| `Pflock` | A shooting position at one `Ziel`, ordered far to near, with an optional configured distance. |

`AnlageVerwaltung` is the deliberate term: `Mitgliedschaft` reads as club membership
(*Vereinsmitglied*), while this relationship means "I work here and manage this". The `Anlage` prefix
carries the scope, so a bare `Verwaltung` is not used for it.

Rules:

1. An `Anlage` is created only by a `Systemadministrator` or through an accepted request
   ([`anlage-anfrage.md`](anlage-anfrage.md)). No `Benutzer` creates one directly.
2. An `AnlageVerwaltung` belongs to the `Anlage`. It names one `Benutzer` and one role, and an `Anlage`
   holds at most one `AnlageVerwaltung` per `Benutzer`.
3. Only a `Systemadministrator` or an `Administrator` of that `Anlage` grants, changes, or revokes a
   `AnlageVerwaltung`. An `Administrator` cannot revoke their own last `AnlageVerwaltung` when that would leave
   the `Anlage` without one.
4. Only a `Systemadministrator` or an `Administrator` of that `Anlage` may grant or revoke the
   `Administrator` role. The named `Benutzer` must already exist — and therefore already have a
   confirmed address (see [`konto.md`](konto.md) rules 3, 14).
5. `Parcours`, `Ziel`, and `Pflock` are created, reordered, edited, and removed by a
   `Systemadministrator`, an `Administrator`, or a `Mitarbeiter` of that `Anlage`. A `Benutzer`
   without an `AnlageVerwaltung` in the `Anlage` may not modify any of them.
6. A `Pflock` order runs far to near per `Ziel`; order is explicit, never inferred from colour. The
   configured distance per `Pflock` is optional and, when present, positive.
7. A `Parcours` is not deleted or reordered in a way that would orphan an existing `Runde`; the
   versioning question is Q-2 and blocks implementation of edits after the first logged `Runde`.
8. Every read of statistics inside an `Anlage` is allowed to its `Administratoren` and `Mitarbeiter`
   and refused to everyone else. The statistics are aggregates; they never expose another
   `Benutzer`'s `Runden`.
9. An `AnlageVerwaltung` never crosses Anlagen: an `Administrator` of one `Anlage` holds nothing in
   another.

## Requirements

**Functional**

- FR-1: Create, edit, and archive-to-inactive an `Anlage` as a `Systemadministrator`.
- FR-2: Grant, change, and revoke an `AnlageVerwaltung` per rules 3 and 4.
- FR-3: List an `Anlage`'s `AnlageVerwaltungen` to its members, with `Benutzer` identity and role.
- FR-4: Create, reorder, edit, and remove `Parcours`, `Ziel`, and `Pflock` per rule 5.
- FR-5: Seed the pilot Anlagen and their master data idempotently, so a redeploy does not duplicate
  them.

**Non-functional**

- NFR-1: Master-data reads are public for active Anlagen; writes always require a logged-in
  `Benutzer` with the matching `AnlageVerwaltung`.
- NFR-2: No endpoint accepts a role or `AnlageVerwaltung` decision from the request body that the caller is
  not entitled to make.

## Open questions

- [ ] Q-1: Admin UI or file import for seeding — and does import stay for creating parcours at
  scale, or is it only for the pilot data?
- [ ] Q-2: May a `Parcours` change after `Runden` were logged against it, and if so, is it versioned
  or edited in place?
- [ ] Q-3: May a `Mitarbeiter` be granted without the `Anlage` having an `Administrator` first?
- [ ] Q-4: What happens to `Runden` when an `Anlage` is deleted, and is an `Anlage` ever hard-deleted
  at all?

## Acceptance criteria

- AC-1: Given a `Systemadministrator` and no `Anlage`, when creating one with name, location, and
  contact, then it exists, is active, and appears in the master-data list.
- AC-2: When a `Benutzer` without an `AnlageVerwaltung` attempts to create or edit any `Anlage`, `Parcours`,
  `Ziel`, or `Pflock`, then the request fails with 403 and nothing is written.
- AC-3: Given an `Anlage` with an `Administrator`, when that `Administrator` grants a `Benutzer` the
  `Mitarbeiter` role, then that `Benutzer` can edit the `Anlage`'s Parcours and read its statistics,
  and still cannot edit another `Anlage`.
- AC-4: Given a `Mitarbeiter`, when attempting to grant an `AnlageVerwaltung` or to promote anyone, then the
  request fails with 403.
- AC-5: Given an `Anlage` with one `Administrator`, when that `Administrator` tries to revoke their
  own `AnlageVerwaltung`, then the request fails with 409 and the `AnlageVerwaltung` stays.
- AC-6: Given a `Ziel` with Pflöcke in a defined order, when the order changes, then the stored order
  matches the request and no order is inferred from colour.
- AC-7: Given a `Pflock` without a configured distance, then creating a `Runde` still works and the
  app offers the slider without a meter estimate.
- AC-8: When running the pilot seed twice against an empty and then a seeded database, then the
  second run changes nothing and duplicates no `Anlage`, `Parcours`, `Ziel`, or `Pflock`.

## Verification

API e2e tests, one case per AC, including the 403 and 409 paths. Domain unit tests for the
`AnlageVerwaltung` rules (rules 2 to 4 and 9) and for `Pflock` ordering and optional distance. AC-8 by
running the seed task twice in a test.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-26 | Authority inside an `Anlage` is an `AnlageVerwaltung` (`Administrator`, `Mitarbeiter`) stored in the `Anlage`, not an account role; the `Anlagenbetreiber` role is dropped. | TimerErTim (PO) |
| 2026-10-26 | `Systemadministrator` creates Anlagen; a `Benutzer` requests one instead ([`anlage-anfrage.md`](anlage-anfrage.md), #68). | TimerErTim (PO) |
| 2026-10-26 | Term is `AnlageVerwaltung`, not `Mitgliedschaft`; roles are `Administrator` and `Mitarbeiter` without an `Anlagen-` prefix. | TimerErTim (PO) |
| 2026-10-26 | Renamed from `Verwaltung` to `AnlageVerwaltung`; the compound names the scope. | TimerErTim (PO) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-26 | Initial draft from issue #10 and the 2026-10-26 role restructure. | Agent (Cursor) |
| 2026-10-26 | `Mitgliedschaft` renamed to `AnlageVerwaltung`; roles lose the `Anlagen-` prefix. | Agent (Cursor) |
| 2026-10-26 | `Verwaltung` renamed to `AnlageVerwaltung`; grammar and rationale sentences updated for the compound. | Agent (Cursor) |
