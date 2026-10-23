---
id: SPEC-stammdaten
title: Master data (Anlage, Parcours, Ziel, Pflock) and admin seeding
status: Draft
owner: TimerErTim
issues: [10]
superseded_by: null
---

# Stammdaten — Master data and admin seeding

## Problem

Before an archer can log a `Runde`, the course must exist as data: the `Anlage` with its
`Parcours`, the `Ziele` in visiting order, and the `Pflöcke` with colors and distances. Today that
knowledge lives with the parcours operators and in the archers' heads. Logged `Runden` then have
nothing stable to reference — and real courses change: targets move, distances are re-measured,
and layouts are rearranged between seasons. Without a master-data model that survives those
changes, history becomes untrustworthy exactly where the product's value lies (accuracy per
parcours and per target over time).

## Outcome

A `System-Administrator` seeds and maintains the master data of the pilot `Anlagen` in an admin
UI: `Anlage`, `Parcours`, `Ziele`, `Pflöcke`. A `Schütze` picks a `Parcours` and sees its `Ziele`
with `Pflöcke` and distances. Every logged `Runde` keeps the layout exactly as it was shot, and a
`Ziel`'s shot history stays complete and continuous across `Runden` and across layout changes.

## Roles

- [`System-Administrator`](charter.md#roles): seeds and maintains master data in the admin UI.
  For M1 the PO seeds the pilot data personally.
- [`Schütze`](charter.md#roles): consumes master data read-only — selects a `Parcours`, sees
  `Ziele`, `Pflöcke`, and distances when logging `Runden` and reviewing history.

## Scope

**In scope**

- Master data `Anlage`, `Parcours`, `Ziel`, `Pflock` with the fields below.
- Seeding and maintenance exclusively through the admin UI (create, edit, reorder, delete,
  restore).
- `Pflock` order per `Ziel` (far to near) and optional `Distanz` per `Pflock`
  (charter MVP scope item 5).
- The `Parcours-Snapshot` recorded with every `Runde`, `Ziel` identity continuity, and deletion
  semantics for master data referenced by logged history.

**Out of scope**

- File import of master data (PO decision 2026-10-23: admin UI only).
- Maintenance by `Anlagenbetreiber` (later milestone) and by `Schützen` (never in MVP).
- The logging flow itself and its distance slider (separate spec); this spec defines only the
  data that flow consumes and the snapshot it must record.
- Editing or deleting logged `Runden` and `Schüsse` — history is never rewritten.
- A `Ziel` shared across `Anlagen`; GPS coordinates, photos, terrain data.
- Tournament-specific setup (later specs).

## User journeys

1. **Seed an `Anlage`.** The `System-Administrator` logs in, opens the master-data admin area,
   and creates the `Anlage` with `Name`, `Standort`, and `Kontakt`. (For M1 the PO seeds the pilot
   `Anlagen` personally; known candidates are Breitenstein and Wallhalla Pregarten — informational
   only, the final list is charter Q-1.)
2. **Seed a `Parcours`.** At the `Anlage` the admin creates a `Parcours` (for example
   "Große Runde"), creates its `Ziele` (`Tier`, `Zielgruppe` 1–4) with their `Pflöcke` (`Farbe`,
   far-to-near order, optional `Distanz` in meters), and arranges the `Ziele` in visiting order.
   Everything is entered in the admin UI; no file is imported.
3. **Maintain between seasons.** The admin reorders `Ziele`, changes a `Ziel`'s `Tier` or
   `Zielgruppe`, re-measures `Distanz` values. `Runden` already logged keep their layout; the
   changes apply to `Runden` logged from now on.
4. **Review history after a change.** A `Schütze` opens an old `Runde` and sees it exactly as
   shot — the `Ziele` in the order of that day with the `Pflöcke` and distances valid then. The
   `Ziel` history aggregates `Schüsse` across all `Runden`, regardless of how the layout changed
   in between.
5. **Remove a target.** The admin deletes a `Ziel`. It disappears from the current arrangements
   and from logging, while its logged `Schüsse` and the past `Runden` containing it stay complete
   and unchanged.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Anlage` | Physical site of a club or commercial operator: `Name`, `Standort`, `Kontakt`. |
| `Parcours` | Named ordered sequence of `Ziele` at an `Anlage` (for example "Große Runde, 32 Ziele"). |
| `Ziel` | One physical 3D animal target at an `Anlage`: `Tier`, `Zielgruppe` 1–4, `Pflöcke`. |
| `Pflock` | Colored stake marking a shooting position of a `Ziel` (for example Weiß, Blau, Rot, Gelb): `Farbe`, far-to-near order, optional `Distanz`. |
| `Zielgruppe` | Target-group classification of a `Ziel`: exactly 1, 2, 3, or 4. |
| `Distanz` | Optional shooting distance in meters from a `Pflock` to its `Ziel` — the `Abschussdistanz` the stake marks. |
| `Parcours-Version` | The layout of a `Parcours` (visiting order and `Ziele` arrangement with configuration) as it applies to the `Runden` logged at a point in time. Master-data edits start a new version for future `Runden`; versions are not managed as separate objects. |
| `Parcours-Snapshot` | The frozen `Parcours-Version` a `Runde` records at log time. Never rewritten. |
| `Archivierung` | What happens to deleted master data referenced by logged history: it leaves master data and new logging but stays in history. Reversible. |

Rules:

1. Master data is maintained only by a `System-Administrator`, through the admin UI. There is no
   file import (PO decision 2026-10-23). `Schützen` read master data and never change it.
2. An `Anlage` records `Name`, `Standort`, and `Kontakt`. `Name` is unique across all `Anlagen`.
3. A `Parcours` belongs to exactly one `Anlage`, has a `Name`, and arranges one or more `Ziele` of
   that `Anlage` in visiting order (positions 1..n). A `Ziel` appears at most once per `Parcours`.
4. A `Ziel` belongs to exactly one `Anlage` as its physical target and records `Tier`, a
   `Zielgruppe` of exactly 1, 2, 3, or 4, and one or more `Pflöcke`. A `Ziel` may appear in
   several `Parcours` of its `Anlage` (Q-1).
5. A `Pflock` belongs to exactly one `Ziel` and records `Farbe`, a unique order position within
   its `Ziel`, and an optional `Distanz` in meters.
6. `Pflock` order per `Ziel` is strictly far to near: position 1 is the farthest shooting
   position and every step comes nearer. Among the `Pflöcke` that carry a `Distanz`, the values
   strictly decrease along that order. `Distanz` is optional per `Pflock`; a `Pflock` without it
   shows no meter value. Distances are master data set by the admin now and by the
   `Anlagenbetreiber` later (charter MVP scope item 5).
7. History contract. Logged `Runden` and `Schüsse` are immutable history: master-data changes
   never rewrite them. Snapshot contract: kept in a `Parcours-Snapshot` are the `Parcours`
   identity and name at log time and, in visiting order, for every position the `Ziel` identity
   and its configuration at log time (`Tier`, `Zielgruppe`, `Pflöcke` with `Farbe`, order,
   optional `Distanz`); a snapshot is never updated afterwards, so a `Runde` always displays the
   layout as it was shot.
8. The snapshot is taken when the `Runde` is created (logging starts). Master-data changes after
   that affect only `Runden` logged later.
9. `Ziel` identity. A `Ziel` keeps one identity from creation until deletion. Editing `Tier`,
   `Zielgruppe`, `Pflöcke`, the position in a `Parcours`, or which `Parcours` use it never
   changes the identity. Its `Schüsse` stay attached, so its shot history and progress remain
   continuous across `Runden` and across `Parcours-Versionen` — old snapshots show the old
   configuration, new `Runden` the new one, all under the one `Ziel`.
10. Replacing the physical target is expressed either as an edit of the existing `Ziel` (history
    continues; old snapshots show the old `Tier`) or as adding a new `Ziel` and archiving the old
    one (the new `Ziel`'s history starts fresh).
11. Deletion never deletes or rewrites logged history. A `Ziel` with logged `Schüsse` is archived
    instead of destroyed: it leaves the current `Parcours` arrangements and logging, while its
    identity, its `Schüsse`, its history view, and the snapshots containing it stay complete. A
    `Ziel` without `Schüsse` is deleted outright. The same applies one level up: a `Parcours`
    with logged `Runden` and an `Anlage` with logged history are archived, and entities without
    history are deleted outright. Archived master data can be restored by the admin; identity and
    history continue.

## Requirements

**Functional**

- FR-1: Create, edit, and delete `Anlagen` (`Name`, `Standort`, `Kontakt`) in the admin UI
  (rules 1–2).
- FR-2: Create and maintain `Parcours` per `Anlage`: `Name` and an ordered `Ziele` arrangement
  (rule 3).
- FR-3: Create and maintain `Ziele` per `Anlage`: `Tier`, `Zielgruppe` 1–4, `Pflöcke`
  (rules 4–5).
- FR-4: Create and maintain `Pflöcke` per `Ziel`: `Farbe`, far-to-near order, optional `Distanz`
  in meters, with the invariants of rule 6.
- FR-5: Seed and maintain all master data exclusively through the admin UI; accept no import file
  (rule 1).
- FR-6: Record a `Parcours-Snapshot` with every `Runde` at log time and never rewrite it
  (rules 7–8).
- FR-7: Keep `Schüsse` attached to `Ziel` identity and expose per-`Ziel` history and progress
  across `Runden` and `Parcours-Versionen` (rule 9).
- FR-8: On deletion, archive master data referenced by logged history and delete the rest,
  without ever touching logged history; support restoring archived master data (rule 11).
- FR-9: Show master data read-only to `Schützen`: `Parcours` selection, visiting order, `Pflöcke`
  with colors and distances.

**Non-functional**

- NFR-1: The rules and snapshot integrity live in the backend domain. The admin UI is a
  maintenance surface only and cannot rewrite history.
- NFR-2: Master-data writes require the `System-Administrator` role at the API level, not merely
  hidden in the UI.

## Implementation sketches

Non-normative implementation sketches (data model, snapshot storage, admin API surface) are
technical design and live in
[Appendix A](#appendix-a-implementation-sketch-non-normative) at the end of this spec. They guide
implementation and are not part of the contract; the contract lives in the rules and acceptance
criteria above.

## Acceptance criteria

- AC-1: Given a `System-Administrator` in the admin UI, when creating an `Anlage` with `Name`,
  `Standort`, and `Kontakt`, then the `Anlage` exists with exactly these fields and is visible
  read-only to `Schützen`; a duplicate `Name` is rejected.
- AC-2: Given an `Anlage`, when the admin creates a `Parcours` with `Ziele` arranged in visiting
  order, then the `Ziele` appear in exactly that order, and a `Ziel` appearing twice in one
  `Parcours` is rejected.
- AC-3: When the admin creates a `Ziel`, then a `Zielgruppe` outside 1–4 or a `Ziel` without at
  least one `Pflock` is rejected; a valid `Ziel` stores `Tier`, `Zielgruppe`, and its `Pflöcke`.
- AC-4: Given a `Ziel` with `Pflöcke` Weiß (position 1), Blau (2), Rot (3) far to near, when it is
  displayed anywhere, then they appear in exactly this order; duplicate or missing order
  positions within a `Ziel` are rejected.
- AC-5: Given `Pflöcke` with configured `Distanz`, when displayed, then each shows its meter value
  and the values strictly decrease far to near; a `Pflock` without `Distanz` shows no meter
  value; a `Distanz` sequence that does not strictly decrease along the order is rejected.
- AC-6: When seeding or changing master data, then every entity is created and edited through the
  admin UI (no import file is accepted anywhere), a `Schütze`'s write request fails with 403 (401
  when unauthenticated), and the same request with a `System-Administrator` succeeds.
- AC-7: Given a `Runde` logged under layout L1, when the admin afterwards reorders the
  `Parcours`'s `Ziele` or edits a `Ziel`'s `Tier`, `Zielgruppe`, `Pflöcke`, or `Distanz`, then the
  logged `Runde` still shows L1 exactly (order and configuration as shot) and `Runden` logged
  afterwards show the changed layout.
- AC-8: Given `Schüsse` at one `Ziel` across `Runden` logged under different `Parcours-Versionen`,
  when viewing the `Ziel`'s history, then all its `Schüsse` appear under that one `Ziel`, and its
  progress stays computable across the `Runden` and versions.
- AC-9: Given a `Runde`, when viewing it, then it shows the `Parcours` and, in visiting order,
  each `Ziel` with `Tier`, `Zielgruppe`, and its `Pflöcke` (`Farbe`, order, optional `Distanz`) as
  they were at log time.
- AC-10: Given a `Ziel` with logged `Schüsse`, when the admin deletes it, then it disappears from
  the current `Parcours` arrangements and from new logging, while its `Schüsse`, its history view,
  and the snapshots of past `Runden` remain complete and unchanged.
- AC-11: Given a `Ziel` without `Schüsse`, when the admin deletes it, then it leaves no trace in
  master data or history.
- AC-12: Given a `Parcours` with logged `Runden` (likewise an `Anlage` with logged history), when
  the admin deletes it, then it cannot be chosen for new `Runden`, while logged `Runden` remain
  visible with their snapshots; the same entities without history disappear outright.
- AC-13: Given an archived `Ziel` or `Parcours`, when the admin restores it, then it returns to
  master data and to new logging, and its history continues under the same identity.

## Risks and open questions

- [ ] Q-1: `Ziel` ↔ `Parcours` cardinality. This draft models a `Ziel` as a physical target of one
  `Anlage`, usable in several `Parcours` of that `Anlage` (sub-courses share targets). Confirm, or
  restrict each `Ziel` to exactly one `Parcours`.
- [ ] Q-2: `Tier`: free text per `Ziel` (draft assumption) or a fixed catalog of 3D animals?
- [ ] Q-3: `Kontakt` of an `Anlage`: one free-text line (draft assumption) or structured fields
  (phone, email, web)?

Charter Q-1 (pilot `Anlagen`) stays open there; known candidates are Breitenstein and Wallhalla
Pregarten. The PO seeds the data personally; nothing in this spec is real master data.

## Verification

Domain unit tests in the backend core cover the invariants of rules 2–6 (`Name` uniqueness,
visiting order, `Zielgruppe` range, unique `Pflock` positions, far-to-near order, decreasing
`Distanz`). Use-case conformance tests cover the seeding CRUD, the snapshot contract and its
immutability (AC-7, AC-9), per-`Ziel` continuity (AC-8), and archiving and restore (AC-10 to
AC-13). API e2e tests cover role enforcement and the admin surface (AC-1 to AC-6). The absence of
an import path and the seeding journey are a manual check on the deployed admin UI.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Seeding: admin UI, not file import. | TimerErTim (PO) |
| 2026-10-23 | Parcours changes after logged Runden: version snapshots. A `Runde` snapshots the layout (order, `Ziele` arrangement) at log time; shot history follows `Ziel` identity — per-`Ziel` continuity, per-`Runde` layout snapshot. | TimerErTim (PO) |
| 2026-10-23 | `Distanz` is the shooting distance per `Pflock` to its `Ziel` (PRD: `Pflöcke` mark `Abschussdistanzen`); the slider estimate belongs to the logging spec. | Agent (MiMo) |
| 2026-10-23 | A `Ziel` is a physical target of one `Anlage`, usable in several `Parcours` of that `Anlage` (Q-1 pending PO confirmation). | Agent (MiMo) |
| 2026-10-23 | Deleting history-referenced master data archives it (rule 11) instead of destroying it, so identity and history survive. | Agent (MiMo) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #10 and PO decisions (admin-UI seeding, version snapshots). | Agent (MiMo) |

## Appendix A: implementation sketch (non-normative)

- `Anlage`, `Parcours`, `Ziel`, `Pflock` carry stable IDs. A `Parcours` stores an ordered list of
  `Ziel` references; `Pflöcke` live under their `Ziel`.
- A `Runde` stores an immutable copy of the arrangement and the per-`Ziel` configuration
  (rule 7) alongside references to `Ziel` IDs for its `Schüsse`; history views read the snapshot,
  never current master data.
- Archiving is a state on master data, not a row delete; history queries include archived
  entities, selection for new `Runden` excludes them.
- The admin API surface is CRUD per entity plus reorder operations; there is no import endpoint.
  Snapshot integrity and the invariants of rules 2–6 are enforced in the backend domain.
