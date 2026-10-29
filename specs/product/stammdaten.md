---
id: SPEC-stammdaten
title: Master data (Anlage, Parcours, Ziel, Pflock) and Anlage administration
status: Draft
owner: TimerErTim
issues: [10]
superseded_by: null
---

# Stammdaten — Master data and `Anlage` administration

## Problem

Archers need an `Anlage` and its `Parcours` before they can log a `Runde`. Course layouts and
shooting positions change, but old results must remain meaningful and attributable to the same
physical `Ziel`. The product needs master data, scoped authority to maintain it, and a history
contract that survives edits and closure of a site.

## Outcome

A `Systemadministrator` can create an `Anlage`; its `Administratoren` and `Mitarbeiter` maintain
course data within their granted scope. Archers can select active `Parcours` and log `Runden`.
Each `Runde` preserves the complete shooting setup as it was when logging began. A physical
`Ziel` keeps its identity and history across course arrangements at the same `Anlage`; archived
master data never breaks recorded history.

## Roles

- [`Systemadministrator`](charter.md#roles): creates and manages every `Anlage`, can act in any
  `Anlage`, and may grant or revoke any `AnlageVerwaltung`.
- [`Administrator`](charter.md#roles): manages one `Anlage`'s `AnlageVerwaltungen` and master data.
- [`Mitarbeiter`](charter.md#roles): maintains Parcours, Ziele, and Pflöcke at their `Anlage` and
  reads its aggregate statistics; cannot manage `AnlageVerwaltungen` or the Anlage's descriptive
  fields.
- [`Benutzer`](charter.md#roles): reads active master data and logs own Runden; has no master-data
  write authority without an `AnlageVerwaltung`.

`Administrator` and `Mitarbeiter` are roles inside an `Anlage`, represented by its
`AnlageVerwaltung`; they are not account roles. Authorization follows [`konto.md`](konto.md) rules
16–17.

## Scope

**In scope**

- `Anlage` identity, structured address, contact details, lifecycle, and `AnlageVerwaltung`.
- `Parcours`, physical `Ziel`, Parcours-specific Ziel placement, and Parcours-level Pflock setup.
- Admin-UI entry and maintenance; no file import or pilot-data seed task.
- Immutable full shooting-setup snapshots on `Runden` and per-physical-Ziel history continuity.
- Archiving history-referenced master data and removing it from future use.

**Out of scope**

- The post-MVP request workflow for a new `Anlage` (issue #68). A `Benutzer` may request one,
  but only a `Systemadministrator` creates it. On acceptance, the requester receives the first
  `Administrator` `AnlageVerwaltung`; the request flow itself is outside this slice.
- The Runde logging flow and distance slider (separate spec, issue #11).
- Import of master data from files, bulk migration, or a pilot seed task.
- Statistics beyond access to an Anlage's aggregate statistics; individual users' Runden and
  Schüsse are never exposed to Anlage staff.
- Deleting or rewriting logged Runden or Schüsse.

## User journeys

1. **Create an Anlage.** A `Systemadministrator` enters the Anlage's name, structured address,
   and optional contact details in the admin UI. The Anlage may temporarily have no
   `Administrator`; one can be assigned later.
2. **Set up a Parcours.** An authorized staff member creates a Parcours, chooses its Pflock labels
   and far-to-near order, adds Ziele belonging to the same Anlage, sets any per-Ziel distances for
   those Pflock labels, and arranges the Ziele in route order.
3. **Maintain a course.** Staff edit the route, Ziel details, Pflock setup, or distances. The edits
   apply to Runden started afterwards. A Runde already in progress and every completed Runde keep
   the setup captured when that Runde started.
4. **Review history.** An archer opens an old Runde and sees its Parcours name, ordered Ziele,
   target details, Pflock labels/order, and configured distances as captured at start. The physical
   Ziel's history remains continuous across Parcours and edits at the same Anlage.
5. **Close or remove data.** Staff deactivate an Anlage or remove a master-data item. History-
   referenced records are archived and excluded from new use; records without history may be
   deleted. Archived data cannot be restored.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Anlage` | Physical site of a club or commercial operator. Has a name, structured address, optional contact details, Parcours, and `AnlageVerwaltungen`. |
| `AnlageVerwaltung` | One Benutzer's role (`Administrator` or `Mitarbeiter`) inside one Anlage. |
| `Parcours` | Named route at one Anlage, containing an ordered arrangement of that Anlage's physical Ziele and one Pflock setup. |
| `Ziel` | One physical 3D target belonging to one Anlage, with an animal name and target group 1–4. |
| `Zielplatzierung` | One physical Ziel's use in one Parcours. It carries optional distances for that Parcours' Pflock labels. |
| `Pflock` | A shooting-position label in one Parcours' setup, ordered far to near; its color/label is chosen by Anlage staff. |
| `Parcours-Snapshot` | Immutable copy of the Parcours shooting setup captured when a Runde starts. |
| `Archivierung` | Removal of history-referenced master data from current use while retaining its identity and historical references. It is not reversible. |

Rules:

1. Master data is created and maintained through the admin UI. There is no file import or pilot seed
   task. The M1 pilot sites are Breitenstein and Wallhalla Pregarten; their data is entered manually.
   The October 23 PO decision on issue #10 is retained.
2. A `Systemadministrator` creates an `Anlage`. An Anlage may initially have no Administrator.
   Only a `Systemadministrator` or that Anlage's `Administrator` may create, edit, or deactivate
   the Anlage or change its name, address, and contact details. A `Systemadministrator` or
   `Administrator` may grant or revoke `AnlageVerwaltungen`; an Anlage must retain at least one
   Administrator once one has been assigned. The last Administrator cannot revoke their own
   `AnlageVerwaltung`.
3. Anlage names need not be unique. An Anlage's address has required street, postal code,
   locality, and country fields. Contact name, email, phone, and website are each optional,
   structured fields; all may be omitted.
4. An Anlage may have zero or more Parcours. A Parcours belongs to exactly one Anlage and contains
   one or more Zielplatzierungen, each for a physical Ziel at that same Anlage. A physical Ziel may
   be used in multiple Parcours at its Anlage, but may appear no more than once in one Parcours.
5. A physical Ziel has one stable identity, a required non-empty animal name, and target group
   (exactly 1, 2, 3, or 4).
   Editing its details or moving it within or between Parcours at the same Anlage does not change
   identity or break per-Ziel shot-history continuity. Replacing the physical target creates a new
   Ziel identity; it does not inherit the old target's history.
6. Each Parcours defines its own unique Pflock labels/colors and explicit far-to-near order. There
   is no product-wide fixed palette, and order is never inferred from color. Every Zielplatzierung
   may optionally define a distance in meters for each Pflock label in that Parcours. A missing
   distance is allowed and has no meter estimate. Configured distances are positive and strictly
   decrease from farther to nearer Pflocks.
7. Parcours edits after logged Runden are allowed. They affect only Runden started after the edit;
   the edit does not rewrite any existing or in-progress Runde snapshot. A Parcours version is the
   effective setup at a point in time; versions are not separately managed objects.
8. When a Runde starts, its immutable snapshot contains the Parcours identity and name and, in
   route order, each Ziel identity, animal name, target group, and its Parcours-specific Pflock
   labels/colors/order and optional distances. The snapshot is the source for displaying that
   Runde's setup even if current master data later changes or is archived.
9. A Ziel's Schüsse refer to its physical Ziel identity, not its position in a route. Per-Ziel
   history and progress therefore continue across Runden, route edits, and Parcours at the same
   Anlage. A Ziel in multiple Parcours has a separate Zielplatzierung and distance configuration
   in each Parcours.
10. Only a `Systemadministrator`, an `Administrator`, or a `Mitarbeiter` of that Anlage may create,
    edit, reorder, or remove its Parcours, Ziel, Zielplatzierung, and Pflock setup. Only a
    `Systemadministrator` or that Anlage's `Administrator` may create or edit the Anlage's
    descriptive fields, deactivate it, or manage its `AnlageVerwaltungen`. Staff authority never
    crosses Anlagen.
11. Deactivation removes an Anlage and all its Parcours from active listings and prevents new
    Runden there. Existing Runden remain readable with their snapshots. Any Anlage, Parcours, Ziel,
    or Pflock setup referenced by a logged or in-progress Runde snapshot or its Schüsse is archived,
    not destroyed; archived data cannot be restored. Master data without any such history reference
    may be deleted outright. Removing a referenced Ziel removes it from current Parcours
    arrangements while preserving its identity, Schüsse, and past snapshots.
12. Active Parcours and their setup are publicly readable for selecting and logging a Runde.
    Anlage staff may read aggregate statistics for their Anlage, but cannot read another Benutzer's
    Runden or Schüsse. A Systemadministrator may act across Anlagen.

## Requirements

**Functional**

- FR-1: Create an Anlage as a `Systemadministrator`; allow its name, structured address, and
  optional contact details to be edited only by a `Systemadministrator` or its `Administrator`
  (rules 2–3).
- FR-2: Grant, change, and revoke per-Anlage `AnlageVerwaltungen`, preserving at least one
  Administrator after the first is assigned (rule 2).
- FR-3: Create and maintain Parcours, ordered Zielplatzierungen, physical Ziele, and Parcours-level
  Pflock labels/order and per-placement distances (rules 4–6, 10).
- FR-4: Record an immutable full Parcours-Snapshot when each Runde starts (rules 7–8).
- FR-5: Preserve per-physical-Ziel shot history across Parcours and edits at the same Anlage
  (rules 5 and 9).
- FR-6: Deactivate Anlagen and archive history-referenced master data without changing logged
  history; prevent new Runden on inactive or archived data (rule 11).
- FR-7: Allow public read access to active Parcours setup; restrict writes and aggregate statistics
  to the roles defined in rules 2, 10, and 12.
- FR-8: Enter the Breitenstein and Wallhalla Pregarten pilot master data and maintain all master
  data through the admin UI; accept no import file and run no idempotent pilot seed task (rule 1).

**Non-functional**

- NFR-1: Snapshot integrity and master-data invariants are enforced by the backend, not only by the
  UI.
- NFR-2: Authorization is decided by use cases from the loaded `Benutzer` and `Anlage` aggregates,
  following [`konto.md`](konto.md) rule 17. No permission table or role middleware is introduced.
- NFR-3: No master-data operation can rewrite a Runde's captured snapshot, Schüsse, or sporting
  results.

## Acceptance criteria

- AC-1: A `Systemadministrator` creates an Anlage with a name, all required structured address
  fields, and any subset (including none) of optional contact fields; the Anlage is active. Duplicate
  names are allowed. Missing required address fields are rejected. Creation may leave it without an
  Administrator.
- AC-2: A Systemadministrator or Anlage Administrator grants a role to an existing Benutzer.
  A Mitarbeiter cannot grant, change, or revoke an AnlageVerwaltung. Once an Administrator exists,
  revoking its last AnlageVerwaltung fails and leaves the Anlage with an Administrator.
- AC-3: A Mitarbeiter may create and edit Parcours, Ziele, Zielplatzierungen, and Pflock setup at
  their Anlage, but cannot edit Anlage descriptive fields, manage AnlageVerwaltungen, or change a
  different Anlage. A Benutzer without an AnlageVerwaltung cannot write master data (403); an
  unauthenticated write fails with 401.
- AC-4: A Parcours can use physical Ziele from its own Anlage, each at most once. Using a Ziel from
  another Anlage or adding the same Ziel twice is rejected. One physical Ziel may be placed in
  multiple Parcours at the same Anlage.
- AC-5: A Ziel requires a non-empty animal name and target group 1–4; a missing animal name or a
  target group outside that range is rejected. Replacing the physical target creates a new identity;
  editing details or route placement preserves identity and history continuity.
- AC-6: Each Parcours stores unique Pflock labels/colors and an explicit far-to-near order
  independent of color. Each Zielplatzierung may configure distinct distances for those labels.
  Missing distances are accepted; configured distances must be positive and strictly decrease far to
  near.
- AC-7: Given a Runde started under setup L1, when staff reorder its Parcours, edit Ziel details,
  change Pflock labels/order, or change distances, then the existing and in-progress Runde still
  display L1 exactly; a Runde started after the edit captures the new setup L2.
- AC-8: When one physical Ziel appears in Runden from different Parcours or setup versions, its
  Schüsse remain linked to that Ziel and per-Ziel history remains continuous. A Ziel in a second
  Parcours has that Parcours' own distances.
- AC-9: When an archer views a Runde, it displays the captured Parcours identity/name, ordered
  Ziele, animal names, target groups, Pflock labels/colors/order, and optional distances as they were
  when the Runde started, regardless of later master-data edits.
- AC-10: When a `Systemadministrator` or that Anlage's `Administrator` deactivates an Anlage, it
  and its Parcours disappear from active selection and new Runden are refused; old Runden remain
  readable with their snapshots. A Mitarbeiter cannot deactivate an Anlage.
- AC-11: Removing a Ziel, Parcours, Anlage, or Pflock setup referenced by a logged or in-progress
  Runde snapshot or its Schüsse archives the referenced master data and prevents new use. Old
  snapshots, Schüsse, and per-Ziel history remain unchanged. Archived data cannot be restored.
- AC-12: Removing a master-data entity with no logged or in-progress Runde snapshot or Schuss
  reference deletes it outright; it no longer appears in master data or history.
- AC-13: Active Parcours are readable without authentication. Anlage Administrators and
  Mitarbeiter can read only aggregate statistics for their own Anlage; neither can read another
  Benutzer's Runden or Schüsse. A Systemadministrator can act across Anlagen.
- AC-14: No master-data import endpoint or file-import path exists; pilot data is entered in the
  admin UI.

## Verification

Domain unit tests cover Zielgruppe validation, Parcours membership, route uniqueness/order,
Pflock ordering and distance invariants, and lifecycle transitions. Shared use-case conformance
tests cover role checks, Anlage and master-data CRUD, snapshot immutability at Runde start,
per-Ziel identity/history continuity, deactivation, archiving, and refusal of new use on archived
data. API e2e tests cover 401/403 authorization paths and readable active Parcours. The absence of
an import path and pilot entry are checked in the admin UI.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Seed through admin UI, not file import. No idempotent pilot seed task. | TimerErTim (PO) |
| 2026-10-23 | Parcours setup is snapshotted per Runde; the snapshot captures the complete shooting setup, while Schüsse follow physical Ziel identity. | TimerErTim (PO), clarified 2026-10-29 |
| 2026-10-29 | Per-Anlage authority is `AnlageVerwaltung` with `Administrator` and `Mitarbeiter`; a Benutzer never creates an Anlage directly. | TimerErTim (PO), aligned with charter and konto |
| 2026-10-29 | Ziel identity represents a physical target at one Anlage and may be used in several Parcours there. A physical replacement starts a new identity. | TimerErTim (PO) |
| 2026-10-29 | Pflock labels/colors and far-to-near order are defined per Parcours; distances are optional per Zielplatzierung and Pflock label. Staff choose labels/colors; there is no fixed global palette. | TimerErTim (PO) |
| 2026-10-29 | Anlage address is structured and required; contact fields are structured and optional. Anlage descriptions and deactivation are controlled by Systemadministrator or that Anlage's Administrator. | TimerErTim (PO) |
| 2026-10-29 | A created Anlage may initially have no Administrator; after one is assigned, at least one must remain. | TimerErTim (PO) |
| 2026-10-29 | Anlage names need not be unique; animal names are required; contact fields are individually optional; Pflock labels are unique per Parcours. | TimerErTim (PO) |
| 2026-10-29 | History-referenced master data is archived and cannot be restored; data without history may be deleted. Inactive Anlagen are unavailable for new Runden. | TimerErTim (PO) |
| 2026-10-29 | M1 pilot Anlagen are Breitenstein and Wallhalla Pregarten; enter their master data in the admin UI, with no seed task. | TimerErTim (PO) |
| 2026-10-29 | Anlage requests are post-MVP (#68), outside this slice. Only a `Systemadministrator` creates an Anlage; acceptance of a request grants the requester its first `Administrator` `AnlageVerwaltung`. | TimerErTim (PO) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #10 and PO decisions on seeding and Parcours version snapshots. | Agent (MiMo) |
| 2026-10-29 | Synced with the current charter/konto role model and interactively respecified the data model, full Runde snapshot, Ziel identity, Pflock configuration, authorization, Anlage lifecycle, and history retention; clarified deactivation authority and post-MVP Anlage requests. | Agent (Cursor) |
