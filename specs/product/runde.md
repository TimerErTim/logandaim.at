---
id: SPEC-runde
title: Capture a Runde target by target, on phone and desktop
status: Draft
owner: TimerErTim
issues: [11, 23]
superseded_by: null
---

# Runde — Capturing a round target by target

## Problem

An archer walks a `Parkour` and shoots several arrows at each `Ziel`, standing between the
`Pflöcke` at whatever distance the situation allows. Today this ends up on a paper score card that
can neither compute the `Wertung` reliably nor remember where each arrow was shot from — the
distance from stake to target that the product's promise (accuracy per `Parkour` and per `Ziel`
over time) rests on. Logging happens outdoors, on a phone, one hand on the device and the other
carrying the bow; reviewing or correcting the same `Runde` happens later on a wide screen.

## Outcome

A `Benutzer` starts a `Runde` for one `Parkour` and one date, walks the `Ziele` in order (skipping
and revisiting freely), logs every `Schuss` at each `Ziel` with its `Wertungszone` and where it was
shot from, and finishes the `Runde`. The app shows the `Wertungen` per `Ziel` and per `Runde`, and
a meter value or estimate of the shooting position. At most one `Runde` is in progress per
`Benutzer`; while it runs, any other screen keeps it one tap away with its elapsed time and
progress. The capture screen focuses on one `Ziel` on a phone and lays the whole route out as cards
on a wide screen, with one-thumb logging throughout.

## Roles

- [`Benutzer`](charter.md#roles): starts, logs, edits, finishes, and deletes own `Runden`. Nobody
  else sees them ([`konto.md`](konto.md) rule 10).
- [`Systemadministrator`](charter.md#roles): no special rights here — like every `Benutzer`, only
  own `Runden` are visible and manageable. Master data is maintained in
  [`stammdaten.md`](stammdaten.md).

## Scope

**In scope**

- Starting a `Runde` on a `Parkour` and date; walking its `Ziele`; finishing, resuming, editing,
  and deleting the `Runde`.
- At most one `Runde` in progress per `Benutzer`, and its `Rundenübersicht` band on the other
  screens.
- Per `Ziel`: unlimited `Schüsse`, each with `Wertungszone` and `Schussposition`.
- The `Schussposition` slider between the `Ziel`'s `Pflöcke` with detents at each `Pflock`, the
  meter estimate when `Distanzen` exist, and an exact `Schussdistanz` typed for a single `Schuss`.
- The capture screen in two presentations: one `Ziel` at a time on a narrow screen, all `Ziel`
  cards on a wide screen.
- One-thumb mobile requirements for the capture flow.

**Out of scope**

- The scoring rules themselves ([`wertung.md`](wertung.md), PO-approved): this spec consumes them
  and does not redefine points, `Pfeil` positions, or `Wertungen`.
- Master data and its version snapshots ([`stammdaten.md`](stammdaten.md)): this spec records and
  displays the `Parkour-Snapshot`, it does not define it.
- The visual treatment of the capture screen — tokens, primitives, and paper furniture. That is
  [`visual-identity.md`](visual-identity.md) and the design-system rule; this spec fixes what each
  block contains and how the flow progresses, not how it is drawn.
- History, statistics, and benchmarks over `Runden` (issue #12 and later specs).
- Bow class per `Runde`: deferred by PO decision, tracked in issue #23
  ([deferred](#bow-class-per-runde)); no behavior is defined for it here.
- Import of previous scorecards (#52), offline capture (#16), WA 2-arrow and IFAA modes.

## User journeys

1. **Start a `Runde`.** The `Benutzer` picks a `Parkour` and a date (default: today) and the
   `Runde` opens in the capture screen, on its first uncaptured `Ziel`. The `Parkour-Snapshot` is
   recorded now; later master-data changes never alter this `Runde`. If a `Runde` is already in
   progress, the start is refused and the app offers to resume that one (rule 7).
2. **Resume.** A `Benutzer` with a `Runde` in progress sees the `Rundenübersicht` band on any other
   signed-in screen and taps it to return to capture (rule 15). Capture opens on the first
   uncaptured `Ziel`, or on the first `Ziel` when every `Ziel` is captured.
3. **Capture a `Ziel` on a phone.** One `Ziel` fills the screen: its ledger of `Schüsse` above, a
   `Wertungszone` button group that registers the next `Schuss` in one tap, and the default-position
   control in the thumb zone below (rule 13). The `Benutzer` taps `Weiter` to the next `Ziel` and
   `Zurück` to an earlier one.
4. **Capture the route on a wide screen.** Every `Ziel` is a card in route order, each with its own
   ledger and its own `Wertungszone` group next to its default-position control (rule 14). The
   `Benutzer` moves between cards freely; a card that received the latest entry is active and the
   captured cards read as done.
5. **Skip and revisit.** The `Benutzer` moves to any other `Ziel` at any time — skipping ahead or
   going back — and adds `Schüsse` there. `Ziele` without `Schuss` are visibly uncaptured.
6. **Finish.** After the last `Ziel` the phone flow opens the `Rundenabschluss` with the two
   `Wertung` totals; on a wide screen the same screen is reached from the finish action below the
   cards. Finishing marks the `Runde` complete. `Ziele` without `Schuss` have no result. An
   unfinished `Runde` stays open and can be resumed.
7. **Edit.** The `Benutzer` corrects a mis-entry in an own `Runde`: add, change, or remove
   `Schüsse` and `Schusspositionen`, or change the date. The `Wertungen` recompute; the `Parkour`
   and its snapshot never change.
8. **Delete.** The `Benutzer` deletes an own `Runde` after explicit confirmation; the `Runde` and
   its `Schüsse` are gone from history.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Runde` | One logged pass of one `Benutzer` through one `Parkour` on one `Datum` ([charter](charter.md#domain-terms)). Owns its `Schüsse` and its `Parkour-Snapshot` ([`stammdaten.md`](stammdaten.md)). |
| `Datum` | The day the `Runde` was shot. Part of the `Runde`, changeable by its owner. |
| `Schuss` | One registered arrow at one `Ziel`: `Pfeil` position in shooting order, `Wertungszone`, `Schussposition`. |
| `Schussposition` | Where a `Schuss` was shot from: either an `Anteil` between two adjacent `Pflöcke` of its `Ziel`, or a `Schussdistanz`. |
| `Anteil` | Fraction from 0 to 1 along a pair of adjacent `Pflöcke`: 0 at the farther `Pflock`, 1 at the nearer `Pflock`. |
| `Schussdistanz` | Distance of a `Schuss` in whole meters, typed by the `Benutzer` for that one `Schuss`. |
| `Distanz-Schätzung` | Meter estimate of an `Anteil`, shown when the two adjacent `Pflöcke` both carry a `Distanz`. |
| `Zielfortschritt` | Captured `Ziele` of a `Runde` over all `Ziele` of its snapshot. |
| `Rundenübersicht` | The bottom band that keeps a `Runde` in progress reachable from the other screens. |

Rules:

1. A `Runde` belongs to exactly one `Benutzer` and records one `Parkour`, one `Datum`, and the
   `Parkour-Snapshot` taken when the `Runde` is created ([`stammdaten.md`](stammdaten.md) rules
   7–8). The `Parkour` never changes afterwards; a wrong `Parkour` is fixed by deleting the `Runde`
   and logging it again.
2. The `Ziele` of a `Runde` are the `Ziele` of its snapshot, in visiting order. The `Benutzer` may
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
   position is continuous. So every position has exactly one representation: a detent on an
   interior `Pflock` is stored as `Anteil` 1 of the segment whose nearer end is that `Pflock`;
   `Anteil` 0 of the first segment is the farthest `Pflock` and `Anteil` 1 of the last segment the
   nearest one. A `Ziel` with exactly one `Pflock` has no slider: every `Schussposition` of the
   `Anteil` form is that `Pflock`.

   A `Schuss` whose distance was typed in meters stores a `Schussdistanz` instead of a segment and
   `Anteil`. The two forms are exclusive: typing meters replaces an `Anteil` position and choosing
   a slider position replaces a `Schussdistanz`, so a `Schuss` never carries both and always has
   exactly one position. A `Schussdistanz` is a positive whole number of meters and is offered for
   every `Ziel`, including one with a single `Pflock`, because it records a distance the `Benutzer`
   measured rather than a stake the `Parkour` defines.
5. Distance display. A `Pflock` with a `Distanz` shows its meter value at its detent
   ([`stammdaten.md`](stammdaten.md) AC-6). For a position between two `Pflöcke` that both carry a
   `Distanz`, the app shows the linear interpolation `d = d_f + Anteil × (d_n − d_f)` (farther
   `d_f`, nearer `d_n`), rounded to whole meters and labelled as an estimate (for example
   "ca. 35 m"). When either `Pflock` of the stored segment lacks a `Distanz`, no estimate is shown
   for positions in that segment. A `Schussdistanz` is shown as its exact value ("35 m"), never as
   an estimate, and takes the position's place in the `Ziel`'s shot ledger.
6. `Wertung` follows [`wertung.md`](wertung.md) unchanged: per `Ziel`, `Erster Treffer` and
   `Mehrschuss` computed from its `Schüsse`; a `Ziel` without `Schuss` has no result (not 0); each
   `Runde` total is the sum of its `Ziel` results per `Wertung`.
7. One running `Runde` per `Benutzer`. A `Benutzer` has at most one `Runde` in progress at a time:
   starting another is refused while one is in progress, and the app offers to resume the running
   one instead. Finishing or deleting the running `Runde` frees the slot. Two start requests issued
   at the same time leave exactly one `Runde` in progress.
8. `Rundenabschluss`. The end of the capture flow shows the two `Runde` totals and, when `Ziele`
   are still uncaptured, how many. Finishing marks the `Runde` complete and is display-only state:
   it changes no data. A finished `Runde` can still be edited and deleted; an unfinished `Runde`
   can be resumed.
9. Editing never changes the `Parkour` or the `Parkour-Snapshot`. Removing a `Schuss` renumbers the
   later `Schüsse` of that `Ziel`, so `Pfeil` stays contiguous from 1 and the points of the moved
   `Schüsse` follow their new `Pfeil`. Master-data changes made after the `Runde` was started never
   rewrite it ([`stammdaten.md`](stammdaten.md) rules 7–8); the `Runde` always displays the layout
   as it was shot.
10. Deleting removes the `Runde` with all its `Schüsse` — including from per-`Ziel` shot history —
    irreversibly, after explicit confirmation.
11. Only the owning `Benutzer` may view, edit, finish, or delete a `Runde`
    ([`konto.md`](konto.md) rule 10); requests from any other `Benutzer` fail like unauthenticated
    ones.
12. `Zielfortschritt`. A `Ziel` is captured once it holds at least one `Schuss`. The
    `Zielfortschritt` of a `Runde` is its captured `Ziele` over all `Ziele` of its snapshot. A
    `Ziel` whose last `Schuss` is removed is uncaptured again. Capture is not completion: only
    finishing (rule 8) completes the `Runde`.
13. Mobile capture. On a narrow screen the capture screen shows one `Ziel` at a time and the
    `Benutzer` advances through the route:

    - A pinned top strip shows both `Runde` totals and the `Zielfortschritt` (rule 12).
    - Beneath it, a collapsed `Zielübersicht` expands into a selectable list of every `Ziel` in
      route order with its capture state; selecting one opens it.
    - The open `Ziel` shows its animal name, its route position, and its own `Erster Treffer` and
      `Mehrschuss` values.
    - Below that, its shot ledger lists every `Schuss` as one row: `Wertungszone` select, the
      resulting points, the `Schussposition` or `Schussdistanz`, a delete action, and an expand
      action that opens that row's position control.
    - Directly beneath the ledger, a `Wertungszone` button group registers a new `Schuss` in one
      tap. The `Schuss` is appended as the next `Pfeil` of this `Ziel`.
    - At the bottom, in the thumb zone, the default-position control sets where the next `Schuss`
      is shot from (rule 16): the `Pflock` slider and a field for an exact `Schussdistanz`
      (rule 4). It stays out of the ledger; a ledger row's expand action edits that row instead.
    - `Zurück` and `Weiter` move between `Ziele`. Advancing past the last `Ziel` opens the
      `Rundenabschluss` (rule 8) instead of wrapping around.
14. Desktop capture. On a wide screen the capture screen shows the route as a column of `Ziel`
    cards in route order, with no step progression:

    - Each card carries its `Ziel`, its shot ledger, and its own default-position control grouped
      with its `Wertungszone` button group, so the position a new `Schuss` will use reads as part
      of entering that `Schuss`.
    - The card that received the latest entry is the active card. Every other card settles into
      its capture state (rule 12): one holding at least one `Schuss` reads as captured, one holding
      none keeps the uncaptured mark. So the first entry in a new card settles the card just left,
      and a card that was merely opened never reads as done.
    - One finish action sits below the cards and opens the `Rundenabschluss` (rule 8).
15. `Rundenübersicht` band. While the `Benutzer` has a `Runde` in progress and is not in the capture
    flow, every signed-in screen carries a bottom-anchored horizontal band naming the `Parkour` and
    showing how long the `Runde` has been running (from its start) and its `Zielfortschritt`
    (rule 12). One tap resumes capture. The band is absent on the capture screen and on the
    `Rundenabschluss`, and reserves its own space, so no content is hidden behind it.
16. Default position. Every new `Schuss` of a `Ziel` starts at the position of that `Ziel`'s most
    recently logged `Schuss` in this `Runde`, whether an `Anteil` or a `Schussdistanz`. When the
    `Ziel` holds no `Schuss`, the default is the nearest `Pflock`. The default is derived from the
    logged `Schüsse` and never stored separately, so it survives a resume without extra state.

## Requirements

**Functional**

- FR-1: Start a `Runde` for one `Parkour` and one `Datum` (default today), recording the
  `Parkour-Snapshot` at creation (rule 1).
- FR-2: Present the `Ziele` in visiting order with an uncaptured marker, and let the `Benutzer`
  move between them freely — skip and revisit (rule 2).
- FR-3: Log unlimited `Schüsse` per `Ziel` in shooting order, each with `Wertungszone` and
  `Schussposition` (rules 3–4).
- FR-4: Set the `Schussposition` on a slider between the `Ziel`'s `Pflöcke` with detents at each
  `Pflock`, show the `Distanz-Schätzung` when it exists, and accept an exact `Schussdistanz` for a
  single `Schuss` (rules 4–5).
- FR-5: Show the `Wertung` per `Ziel` and the two `Runde` totals, computed per
  [`wertung.md`](wertung.md) (rule 6).
- FR-6: Hold at most one `Runde` in progress per `Benutzer`, refuse a second start, and offer to
  resume the running one (rule 7).
- FR-7: Finish a `Runde` through the `Rundenabschluss` and resume an unfinished one (rule 8).
- FR-8: Edit `Datum`, `Schüsse`, and `Schusspositionen` of an own `Runde`, with `Wertungen`
  recomputed and deleted `Schüsse` renumbered (rule 9).
- FR-9: Delete an own `Runde` after explicit confirmation (rule 10).
- FR-10: Enforce ownership on every read and write of a `Runde` (rule 11).
- FR-11: Show the `Zielfortschritt` of a `Runde` (rule 12).
- FR-12: Present the capture screen on a narrow screen as one `Ziel` at a time with the strip,
  `Zielübersicht`, ledger, `Wertungszone` group, bottom default-position control, and
  `Zurück`/`Weiter` (rule 13).
- FR-13: Present the capture screen on a wide screen as `Ziel` cards with per-card ledger,
  per-card default-position control next to the `Wertungszone` group, the active-card marking, and
  one finish action (rule 14).
- FR-14: Show the `Rundenübersicht` band while a `Runde` is in progress and the `Benutzer` is
  outside the capture flow, and resume capture from it (rule 15).
- FR-15: Start each new `Schuss` from its `Ziel`'s default position (rule 16).

**Non-functional**

- NFR-1: One-thumb logging. On the capture screen every interactive control has a touch target of
  at least 44×44 pixels ([charter](charter.md#accessibility)); the primary controls
  (`Wertungszone`, default-position control, `Ziel` navigation, finish) sit in the lower thumb
  zone and need no re-gripping; logging a `Schuss` needs no typing, no hover, and no precise
  pointing or drag accuracy, and stays usable with gloves; `Wertungszone` state is never encoded
  by colour alone.
- NFR-2: The `Wertung` computation is authoritative in the backend domain
  ([`wertung.md`](wertung.md) NFR-1); the frontend may preview points.
- NFR-3: Ownership is enforced at the API level, not merely in the UI
  ([`konto.md`](konto.md) rules 10 and 17). The one-running-`Runde` rule (rule 7) holds against
  concurrent requests, not only against the UI.
- NFR-4: Layout parity. Every capture capability — logging a `Schuss`, editing a ledger row, both
  position forms, `Ziel` navigation, and finishing — exists on a narrow and on a wide screen
  (rules 13–14). The layout difference is presentation only; no capability exists in one
  presentation alone.
- NFR-5: Economy. The capture screen states its `Parkour`, `Datum`, and running state once, in the
  screen header or the `Ziel` header, never as a repeated banner or badge, and renders no
  instruction that the controls already make obvious. Labels name the thing, not the gesture.

## Acceptance criteria

- AC-1: Given a `Parkour` with `Ziele` in visiting order, when starting a `Runde` on a date, then
  the `Runde` holds exactly that `Parkour` and `Datum` and its `Parkour-Snapshot` equals the
  `Parkour` version at that moment; master-data changes afterwards leave the `Runde` showing the
  layout as it was shot.
- AC-2: When the `Benutzer` logs `Ziele` out of order — skipping ahead, revisiting a logged `Ziel` —
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
- AC-7: Given a `Ziel` with exactly one `Pflock`, then no slider is offered, a `Schuss` registered
  by the `Wertungszone` group stores that `Pflock` as its `Schussposition`, and an exact
  `Schussdistanz` can still be typed for a `Schuss` (AC-15).
- AC-8: When a `Runde` is finished with unlogged `Ziele`, then those `Ziele` show no result and the
  totals cover only the logged `Ziele`; `Schüsse` added when the `Runde` is resumed later are
  included in the recomputed totals.
- AC-9: When the owner adds, changes, or removes a `Schuss` or `Schussposition` in an own `Runde`,
  or changes its `Datum`, then the `Wertungen` recompute and the `Parkour-Snapshot` is unchanged.
- AC-10: When the owner deletes a `Runde` after confirming, then the `Runde` and its `Schüsse` are
  gone, including from per-`Ziel` shot history; without confirmation nothing is deleted.
- AC-11: When another `Benutzer` requests a `Runde` that is not its own, then the request fails with
  403 (401 when unauthenticated), and the same request with the owning `Benutzer` succeeds.
- AC-12: On the capture screen every interactive control has a touch target of at least 44×44
  pixels, and the whole journey on a phone (start, log a `Ziel` with `Schüsse` and position, skip,
  finish) is completable with one thumb.
- AC-13: Given a `Benutzer` with a `Runde` in progress, when another `Runde` is started, then the
  start is refused, the running `Runde` is unchanged, and the app offers to resume it; after
  finishing or deleting the running `Runde` a new start succeeds; two start requests issued at the
  same time leave exactly one `Runde` in progress.
- AC-14: Given a `Runde` whose snapshot has 5 `Ziele`, when 2 of them hold at least one `Schuss`,
  then the `Zielfortschritt` reads 2 of 5 in the `Rundenübersicht` band and on the capture screen;
  removing the last `Schuss` of one of them returns it to 1 of 5.
- AC-15: Given a `Ziel` with no `Schuss`, when the first `Schuss` is registered, then its position
  is the nearest `Pflock`; given a `Ziel` whose most recent `Schuss` holds `Anteil` 0.5 or a
  `Schussdistanz`, then the next one-tap `Schuss` starts from that same position. When a distance
  is typed instead, the `Schuss` stores exactly that positive whole-meter `Schussdistanz`, shows it
  without the estimate label, and no longer carries the `Anteil` form; a non-positive or fractional
  value is rejected.
- AC-16: Given a `Ziel` with `Schüsse` at `Pfeil` 1–4, when the `Schuss` at `Pfeil` 2 is deleted,
  then the remaining three are renumbered 1–3, their points follow their new `Pfeil`, and the
  `Ziel` and `Runde` `Wertungen` recompute.
- AC-17: On a narrow screen the capture screen shows exactly one `Ziel` at a time, with both
  `Runde` totals and the `Zielfortschritt` in a pinned strip, a `Zielübersicht` that expands into
  a selectable list of every `Ziel`, a shot ledger whose every row offers a `Wertungszone` select,
  its points, its position, delete, and expand, a `Wertungszone` group that registers a `Schuss`
  in one tap, and the default-position control at the bottom; `Zurück` and `Weiter` move between
  `Ziele` and advancing past the last `Ziel` opens the `Rundenabschluss`, which a mid-route
  `Weiter` never opens.
- AC-18: On a wide screen the capture screen shows every `Ziel` as a card in route order with its
  own ledger and its own default-position control grouped with its `Wertungszone` group; the card
  that received the latest entry is active, a card holding at least one `Schuss` reads as
  captured, a card holding none keeps the uncaptured mark, and finishing is reached from the single
  action below the cards.
- AC-19: While a `Runde` is in progress and the `Benutzer` is on any signed-in screen outside the
  capture flow, a bottom band shows the `Parkour`, the elapsed time since the `Runde` started, and
  its `Zielfortschritt`, and one tap opens capture at the first uncaptured `Ziel`; on the capture
  screen and the `Rundenabschluss` the band is absent and nothing is drawn under it.

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
- [ ] Q-4: Is the nearest `Pflock` the right default for the first `Schuss` of a `Ziel`
  (rule 16), or should the first `Schuss` require an explicit position before it is registered?
- [ ] Q-5: How does a `Schussdistanz` enter the per-`Ziel` and per-`Parkour` accuracy metrics of
  issue #12 — as its exact meters, or only through the `Distanz-Schätzung` it would have had?
  Draft assumption: exact meters, since that is what the `Benutzer` measured.

Resolved (PO 2026-10-23): slider storage as `Anteil` fraction between the two adjacent `Pflöcke`
with a meter estimate when `Distanzen` exist; bow class per `Runde` deferred to issue #23. See the
decision log.

Resolved (PO 2026-11-06): one `Runde` in progress per `Benutzer`; a direct number field stores an
exact `Schussdistanz` per `Schuss`; capture is one `Ziel` at a time on a phone and a card per `Ziel`
on a wide screen; the capture strip shows the `Runde` totals and progress; deleting a `Schuss`
renumbers the later `Schüsse`.

## Verification

Domain unit tests in the backend core cover the `Schussposition` model (segment/`Anteil`
canonicalization and detents of rule 4, the exclusive `Schussdistanz` form, single-`Pflock` case,
interpolation of rule 5), the `Pfeil` renumbering of rule 9, and the `Wertung` hand-off to
[`wertung.md`](wertung.md). Use-case conformance tests cover start, walk, skip/revisit, finish,
edit, and delete (AC-1 to AC-10), the one-running-`Runde` rule including a concurrent double start
(AC-13), `Zielfortschritt` (AC-14), and the default position (AC-15). API e2e tests cover ownership
enforcement (AC-11) and the refused second start (AC-13). The two capture presentations (AC-12,
AC-17 to AC-19) and the `Rundenübersicht` band are manual checks: a phone-sized viewport for
one-thumb reach and touch targets, a wide viewport for the card layout, and both for layout parity
(NFR-4).

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-23 | Slider storage: `Anteil` fraction 0..1 between the two adjacent `Pflöcke`; meter estimate shown when `Pflock` `Distanzen` exist. | TimerErTim (PO) |
| 2026-10-23 | Bow class per `Runde` is not in MVP; deferred and tracked in issue #23. | TimerErTim (PO) |
| 2026-10-23 | A detent on an interior `Pflock` is stored as `Anteil` 1 of the segment whose nearer end is that `Pflock`, so every position has exactly one representation. | Agent (MiMo) |
| 2026-10-23 | `Datum` is editable; `Parkour` is fixed at start, a wrong choice is fixed by delete and re-log (snapshot is taken at creation). | Agent (MiMo) |
| 2026-11-06 | One `Runde` in progress per `Benutzer`; a second start is refused and offers to resume the running one. | TimerErTim (PO) |
| 2026-11-06 | The direct number field stores an exact positive whole-meter `Schussdistanz` on the `Schuss`; a `Schuss` carries either the `Anteil` form or the `Schussdistanz` form, never both. | TimerErTim (PO) |
| 2026-11-06 | The `Rundenübersicht` band sits at the bottom of every signed-in screen except the capture screen and shows elapsed time and `Zielfortschritt`. | TimerErTim (PO) |
| 2026-11-06 | Capture is one `Ziel` at a time with a pinned strip, `Zielübersicht`, and bottom position control on a phone, and `Ziel` cards with per-card position control on a wide screen; the difference is presentation only. | TimerErTim (PO) |
| 2026-11-06 | The capture strip shows both `Runde` totals and the `Zielfortschritt`; the open `Ziel` shows its own two values. | TimerErTim (PO) |
| 2026-11-06 | Deleting a `Schuss` renumbers the later `Schüsse` of that `Ziel`, so `Pfeil` stays contiguous. | TimerErTim (PO) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #11 and PO decisions. | Agent (MiMo) |
| 2026-11-06 | Redesigned capture: one running `Runde` per `Benutzer` with a bottom `Rundenübersicht` band, the mobile step flow and the desktop card layout, exact `Schussdistanz` entry, `Zielfortschritt`, and `Pfeil` renumbering on delete. Renamed `Schütze` to `Benutzer` and `Parcours` to `Parkour` to match the charter and `stammdaten.md`. | Agent (Cursor) |

## Appendix A: implementation sketch (non-normative)

- A `Runde` is an aggregate owning its `Schüsse`; it references `Ziel` identities (stable per
  [`stammdaten.md`](stammdaten.md) rule 9) and stores its `Parkour-Snapshot` as an immutable copy.
- A `Schuss` stores `Pfeil`, `Wertungszone`, and the `Schussposition` as either (segment between
  adjacent `Pflöcke`, `Anteil`) or a whole-meter `Schussdistanz`; the canonical form of rule 4 plus
  the exclusive variants keep storage unambiguous and the slider redisplay exact.
- The one-running-`Runde` rule is a constraint on the owning `Benutzer`, enforced in the same
  transaction as the start (a partial unique index on in-progress `Runden` per owner, or an
  equivalent check), never only in the UI.
- `Pfeil` is the position of a `Schuss` within its `Ziel` and is rewritten on removal (rule 9), so
  the score of a moved `Schuss` changes with it; the domain owns that rewrite.
- `Wertungen` are computed by the domain per [`wertung.md`](wertung.md) from the `Schüsse`; the
  `Runde` totals are the sums of its `Ziel` results.
- `Zielfortschritt` and the default position (rules 12 and 16) are read from the logged `Schüsse`:
  no capture cursor, "last visited `Ziel`", or remembered default is stored.
- Deleting a `Runde` removes the aggregate; per-`Ziel` history queries (issue #12) read `Schüsse`
  through their `Ziel` reference.
