---
id: SPEC-runde
title: Log a Runde target by target with distance slider
status: Draft
owner: TimerErTim
issues: [11, 23]
superseded_by: null
---

# Runde — Logging a round target by target

## Problem

An archer walks a `Parcours` and shoots several arrows at each `Ziel`, standing between the
`Pflöcke` at whatever distance the situation allows. Today this ends up on a paper score card
that can neither compute the `Wertung` reliably nor remember where each arrow was shot from —
the distance from stake to target that the product's promise (accuracy per `Parcours` and per
`Ziel` over time) rests on. Logging happens outdoors, on a phone, one hand on the device and the
other carrying the bow.

## Outcome

A `Schütze` starts a `Runde` for one `Parcours` and one date, walks the `Ziele` in order (skipping
and revisiting freely), logs every `Schuss` at each `Ziel` with its `Wertungszone` and where it
was shot from on a slider between the `Pflöcke`, and finishes the `Runde`. The app shows the
`Wertungen` per `Ziel` and per `Runde`, and a meter estimate of the shooting position when the
`Pflöcke` have distances. The whole flow works with one thumb on a phone.

## Roles

- [`Schütze`](charter.md#roles): starts, logs, edits, finishes, and deletes own `Runden`. Nobody
  else sees them ([`konto.md`](konto.md) rule 10).
- [`System-Administrator`](charter.md#roles): no special rights here — like every `Schütze`, only
  own `Runden` are visible and manageable. Master data is maintained in
  [`stammdaten.md`](stammdaten.md).

## Scope

**In scope**

- Starting a `Runde` on a `Parcours` and date; walking its `Ziele`; finishing, resuming, editing,
  and deleting the `Runde`.
- Per `Ziel`: unlimited `Schüsse`, each with `Wertungszone` and `Schussposition`.
- The `Schussposition` slider between the `Ziel`'s `Pflöcke` with detents at each `Pflock`, and the
  meter estimate when `Distanzen` exist.
- One-thumb mobile requirements for the logging flow.

**Out of scope**

- The scoring rules themselves ([`wertung.md`](wertung.md), PO-approved): this spec consumes them
  and does not redefine points, `Pfeil` positions, or `Wertungen`.
- Master data and its version snapshots ([`stammdaten.md`](stammdaten.md)): this spec records and
  displays the `Parcours-Snapshot`, it does not define it.
- History, statistics, and benchmarks over `Runden` (issue #12 and later specs).
- Bow class per `Runde`: deferred by PO decision, tracked in issue #23
  ([deferred](#bow-class-per-runde)); no behavior is defined for it here.
- Import of previous scorecards (#52), offline capture (#16), WA 2-arrow and IFAA modes.

## User journeys

1. **Start a `Runde`.** The `Schütze` picks a `Parcours` and a date (default: today) and the
   `Runde` opens on its first `Ziel`. The `Parcours-Snapshot` is recorded now; later master-data
   changes never alter this `Runde`.
2. **Log a `Ziel`.** For the current `Ziel` the `Schütze` adds `Schüsse` in shooting order, each
   with its `Wertungszone`, and sets where the arrow was shot from on the slider between the
   `Pflöcke` — at a `Pflock` or anywhere between two of them. When the `Pflöcke` have distances,
   a meter estimate appears next to the slider. The `Wertung` of the `Ziel` updates as `Schüsse`
   are added.
3. **Skip and revisit.** The `Schütze` moves to any other `Ziel` at any time — skipping ahead or
   going back — and adds `Schüsse` there. `Ziele` without `Schuss` are visibly unfinished.
4. **Finish.** The `Schütze` marks the `Runde` finished and sees the two `Wertung` totals.
   `Ziele` without `Schuss` have no result. An unfinished `Runde` stays open and can be resumed
   later.
5. **Edit.** The `Schütze` corrects a mis-entry in an own `Runde`: add, change, or remove
   `Schüsse` and `Schusspositionen`, or change the date. The `Wertungen` recompute; the `Parcours`
   and its snapshot never change.
6. **Delete.** The `Schütze` deletes an own `Runde` after explicit confirmation; the `Runde` and
   its `Schüsse` are gone from history.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Runde` | One logged pass of one `Schütze` through one `Parcours` on one `Datum` ([charter](charter.md#domain-terms)). Owns its `Schüsse` and its `Parcours-Snapshot` ([`stammdaten.md`](stammdaten.md)). |
| `Datum` | The day the `Runde` was shot. Part of the `Runde`, changeable by its owner. |
| `Schuss` | One registered arrow at one `Ziel`: `Pfeil` position in shooting order, `Wertungszone`, `Schussposition`. |
| `Schussposition` | Where a `Schuss` was shot from: an `Anteil` between two adjacent `Pflöcke` of its `Ziel`. |
| `Anteil` | Fraction from 0 to 1 along a pair of adjacent `Pflöcke`: 0 at the farther `Pflock`, 1 at the nearer `Pflock`. |
| `Distanz-Schätzung` | Meter estimate of a `Schussposition`, shown when the two adjacent `Pflöcke` both carry a `Distanz`. |

Rules:

1. A `Runde` belongs to exactly one `Konto` and records one `Parcours`, one `Datum`, and the
   `Parcours-Snapshot` taken when the `Runde` is created ([`stammdaten.md`](stammdaten.md) rules
   7–8). The `Parcours` never changes afterwards; a wrong `Parcours` is fixed by deleting the
   `Runde` and logging it again.
2. The `Ziele` of a `Runde` are the `Ziele` of its snapshot, in visiting order. The `Schütze` may
   log them in any order: skip `Ziele`, revisit them, and add `Schüsse` at any time until the
   `Runde` is deleted. Within one `Ziel`, the `Schüsse` keep their shooting order (`Pfeil` 1, 2,
   3, … per [`wertung.md`](wertung.md)).
3. Every `Schuss` records exactly one `Wertungszone` (`Spot`, `Kill`, `Body`, `Miss`) and exactly
   one `Schussposition`. There is no upper limit on `Schüsse` per `Ziel`
   ([`wertung.md`](wertung.md) rule 1).
4. `Schussposition`. The slider spans the `Ziel`'s `Pflöcke` from the farthest to the nearest
   (`Pflock` order far to near, [`stammdaten.md`](stammdaten.md) rule 6) and has a detent at each
   `Pflock`. The position is stored as the pair (segment, `Anteil`): the segment is one pair of
   adjacent `Pflöcke` (farther, nearer) and the `Anteil` is a fraction from 0 (at the farther
   `Pflock`) to 1 (at the nearer `Pflock`). A detent lands on `Anteil` 0 or 1; between detents the
   position is continuous. So every position has exactly one representation, a detent on an
   interior `Pflock` is stored as `Anteil` 1 of the segment whose nearer end is that `Pflock`;
   `Anteil` 0 of the first segment is the farthest `Pflock` and `Anteil` 1 of the last segment the
   nearest one.
   A `Ziel` with exactly one `Pflock` has no slider: every `Schussposition` is that `Pflock`.
5. `Distanz-Schätzung`. A `Pflock` with a `Distanz` shows its meter value at its detent
   ([`stammdaten.md`](stammdaten.md) AC-5). For a position between two `Pflöcke` that both carry a
   `Distanz`, the app shows the linear interpolation `d = d_f + Anteil × (d_n − d_f)` (farther
   `d_f`, nearer `d_n`), rounded to whole meters and labelled as an estimate (for example
   "ca. 35 m"). When either `Pflock` of the stored segment lacks a `Distanz`, no estimate is shown
   for positions in that segment.
6. `Wertung` follows [`wertung.md`](wertung.md) unchanged: per `Ziel`, `Erster Treffer` and
   `Mehrschuss` computed from its `Schüsse`; a `Ziel` without `Schuss` has no result (not 0); each
   `Runde` total is the sum of its `Ziel` results per `Wertung`.
7. Finishing marks the `Runde` complete and is display-only state: it changes no data. A finished
   `Runde` can still be edited and deleted; an unfinished `Runde` can be resumed.
8. Editing never changes the `Parcours` or the `Parcours-Snapshot`. Master-data changes made after
   the `Runde` was started never rewrite it ([`stammdaten.md`](stammdaten.md) rules 7–8); the
   `Runde` always displays the layout as it was shot.
9. Deleting removes the `Runde` with all its `Schüsse` — including from per-`Ziel` shot history —
   irreversibly, after explicit confirmation.
10. Only the owning `Konto` may view, edit, finish, or delete a `Runde`
    ([`konto.md`](konto.md) rule 10); requests from any other `Konto` fail like unauthenticated
    ones.

## Requirements

**Functional**

- FR-1: Start a `Runde` for one `Parcours` and one `Datum` (default today), recording the
  `Parcours-Snapshot` at creation (rule 1).
- FR-2: Present the `Ziele` in visiting order with an unfinished marker, and let the `Schütze`
  move between them freely — skip and revisit (rule 2).
- FR-3: Log unlimited `Schüsse` per `Ziel` in shooting order, each with `Wertungszone` and
  `Schussposition` (rules 3–4).
- FR-4: Set the `Schussposition` on a slider between the `Ziel`'s `Pflöcke` with detents at each
  `Pflock`, and show the `Distanz-Schätzung` when it exists (rules 4–5).
- FR-5: Show the `Wertung` per `Ziel` and the two `Runde` totals, computed per
  [`wertung.md`](wertung.md) (rule 6).
- FR-6: Finish a `Runde` and resume an unfinished one (rule 7).
- FR-7: Edit `Datum`, `Schüsse`, and `Schusspositionen` of an own `Runde`, with `Wertungen`
  recomputed (rules 7–8).
- FR-8: Delete an own `Runde` after explicit confirmation (rule 9).
- FR-9: Enforce ownership on every read and write of a `Runde` (rule 10).

**Non-functional**

- NFR-1: One-thumb logging. On the logging screen every interactive control has a touch target of
  at least 44×44 pixels ([charter](charter.md#accessibility)); the primary controls (`Wertungszone`,
  slider, `Ziel` navigation, finish) sit in the lower thumb zone and need no re-gripping; logging a
  `Schuss` needs no typing, no hover, no precise pointing or drag accuracy, and stays usable with
  gloves; `Wertungszone` state is never encoded by colour alone.
- NFR-2: The `Wertung` computation is authoritative in the backend domain
  ([`wertung.md`](wertung.md) NFR-1); the frontend may preview points.
- NFR-3: Ownership is enforced at the API level, not merely in the UI
  ([`konto.md`](konto.md) FR-7).

## Acceptance criteria

- AC-1: Given a `Parcours` with `Ziele` in visiting order, when starting a `Runde` on a date, then
  the `Runde` holds exactly that `Parcours` and `Datum` and its `Parcours-Snapshot` equals the
  `Parcours-Version` at that moment; master-data changes afterwards leave the `Runde` showing the
  layout as it was shot.
- AC-2: When the `Schütze` logs `Ziele` out of order — skipping ahead, revisiting a logged `Ziel` —
  then every `Schuss` stays attached to its `Ziel` and within each `Ziel` the `Schüsse` keep the
  shooting order `Pfeil` 1, 2, 3, …
- AC-3: When 10 `Schüsse` are logged at one `Ziel`, then all 10 are stored; those from `Pfeil` 4 on
  score 0 in both `Wertungen` but remain in the `Runde`.
- AC-4: Given `Schüsse` at a `Ziel`, then its `Erster Treffer` and `Mehrschuss` equal the results
  defined by [`wertung.md`](wertung.md), and the `Runde` totals equal the sum of its `Ziel`
  results; a `Ziel` without `Schuss` contributes nothing and shows no result.
- AC-5: Given a `Ziel` with `Pflöcke` far to near, when the slider rests on a detent, then the
  stored `Schussposition` is `Anteil` 0 or 1 of the matching segment and redisplay shows exactly
  that `Pflock`; between two detents it is stored with `0 < Anteil < 1` and redisplay shows the
  same place between the same two `Pflöcke`.
- AC-6: Given `Pflöcke` with `Distanz` 40 m (farther) and 30 m (nearer) around one segment, then
  the detents show 40 m and 30 m and `Anteil` 0.5 shows the estimate "ca. 35 m"; when only one of
  the two `Pflöcke` has a `Distanz`, positions in that segment show no estimate while the `Pflock`
  with a `Distanz` still shows its own value at its detent.
- AC-7: Given a `Ziel` with exactly one `Pflock`, then no slider is offered and every logged
  `Schussposition` is that `Pflock`.
- AC-8: When a `Runde` is finished with unlogged `Ziele`, then those `Ziele` show no result and the
  totals cover only the logged `Ziele`; `Schüsse` added when the `Runde` is resumed later are
  included in the recomputed totals.
- AC-9: When the owner adds, changes, or removes a `Schuss` or `Schussposition` in an own `Runde`,
  or changes its `Datum`, then the `Wertungen` recompute and the `Parcours-Snapshot` is unchanged.
- AC-10: When the owner deletes a `Runde` after confirming, then the `Runde` and its `Schüsse` are
  gone, including from per-`Ziel` shot history; without confirmation nothing is deleted.
- AC-11: When another `Konto` requests a `Runde` that is not its own, then the request fails with
  403 (401 when unauthenticated), and the same request with the owning `Konto` succeeds.
- AC-12: On the logging screen every interactive control has a touch target of at least 44×44
  pixels, and the whole journey (start, log a `Ziel` with `Schüsse` and position, skip, finish) is
  completable with one thumb on a phone.

## Deferred requirements

The capabilities in this part are not part of the MVP contract above. They are written here so the
deferred requirements do not get forgotten or relitigated from scratch.

### Bow class per Runde

Issue: #23. Deferred by PO decision 2026-10-23 (charter Q-4).

Not in MVP. This spec deliberately defines **no behavior** for it: no field, no rule, and no
acceptance criterion above refers to a bow class. When issue #23 is picked up, the requirement is
specified there (or by extending this spec through a revision) together with its own acceptance
criteria.

## Risks and open questions

- [ ] Q-1: Slider resolution. The PO decided the storage shape (fraction 0..1 between the two
  adjacent `Pflöcke`, 2026-10-23) but not the resolution of a stored `Anteil`. Draft assumption:
  continuous in the UI, stored rounded to 3 decimal places.
- [ ] Q-2: Is a `Runde` editable forever by its owner (draft assumption), or does it lock after
  some time or once it feeds history statistics?
- [ ] Q-3: Is deletion irreversible behind a confirmation (draft assumption), or is there an undo
  or trash period?

Resolved (PO 2026-10-23): slider storage as `Anteil` fraction between the two adjacent `Pflöcke`
with a meter estimate when `Distanzen` exist; bow class per `Runde` deferred to issue #23. See the
decision log.

## Verification

Domain unit tests in the backend core cover the `Schussposition` model (segment/`Anteil`
canonicalization and detents of rule 4, single-`Pflock` case, interpolation of rule 5) and the
`Wertung` hand-off to [`wertung.md`](wertung.md). Use-case conformance tests cover start, walk,
skip/revisit, finish, edit, and delete (AC-1 to AC-10) against fake and real persistence. API e2e
tests cover ownership enforcement (AC-11). The one-thumb journey and touch-target sizes (AC-12)
are a manual check on a phone.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Slider storage: `Anteil` fraction 0..1 between the two adjacent `Pflöcke`; meter estimate shown when `Pflock` `Distanzen` exist. | TimerErTim (PO) |
| 2026-10-23 | Bow class per `Runde` is not in MVP; deferred and tracked in issue #23. | TimerErTim (PO) |
| 2026-10-23 | A detent on an interior `Pflock` is stored as `Anteil` 1 of the segment whose nearer end is that `Pflock`, so every position has exactly one representation. | Agent (MiMo) |
| 2026-10-23 | `Datum` is editable; `Parcours` is fixed at start, a wrong choice is fixed by delete and re-log (snapshot is taken at creation). | Agent (MiMo) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #11 and PO decisions. | Agent (MiMo) |

## Appendix A: implementation sketch (non-normative)

- A `Runde` is an aggregate owning its `Schüsse`; it references `Ziel` identities (stable per
  [`stammdaten.md`](stammdaten.md) rule 9) and stores its `Parcours-Snapshot` as an immutable copy.
- A `Schuss` stores `Pfeil`, `Wertungszone`, and the `Schussposition` as (segment between adjacent
  `Pflöcke`, `Anteil`); the canonical form of rule 4 keeps storage unambiguous and the slider
  redisplay exact.
- `Wertungen` are computed by the domain per [`wertung.md`](wertung.md) from the `Schüsse`; the
  `Runde` totals are the sums of its `Ziel` results.
- Deleting a `Runde` removes the aggregate; per-`Ziel` history queries (issue #12) read `Schüsse`
  through their `Ziel` reference.
