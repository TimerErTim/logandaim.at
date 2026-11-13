---
id: SPEC-runde
title: Capture a Runde target by target, on phone and desktop
status: PO-approved
owner: TimerErTim
issues: [11, 23, 72, 93]
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

A `Benutzer` starts a `Runde` for one `Parkour` and one date — or a `Freie Runde` with no `Parkour`
at all — walks the `Ziele` in order (skipping and revisiting freely), logs every `Schuss` at each
`Ziel` with its `Wertungszone` and where it was shot from — a distance in meters, or the `Pflock`
(its colour on a `Freie Runde`) — and finishes the `Runde`. The app shows the
`Wertungen` per `Ziel` and per `Runde` and the shot positions as they were recorded. At most one
`Runde` is in progress per `Benutzer`; while it runs, any other screen keeps it one tap away with
its elapsed time and progress. The capture screen focuses on one `Ziel` on a phone and lays the
whole route out as cards on a wide screen, with one-thumb logging throughout.

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
- Starting a `Freie Runde` on a date with no `Anlage` and no `Parkour`, adding and removing its
  `Freies Ziel` just in time, and capturing it the same way.
- At most one `Runde` in progress per `Benutzer`, and its `Rundenübersicht` band on the other
  screens.
- Per `Ziel`: unlimited `Schüsse`, each with `Wertungszone` and `Schussposition`.
- The `Schussposition`: a `Schussdistanz` in meters or the `Pflock`, entered through a slider that
  clings to stakes without configured `Distanzen` and interpolates on a meter scale when they have
  them, plus an exact distance typed for a single `Schuss`.
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
   `Wertungszone` button group that registers the next `Schuss` in one tap, and the position
   control in the thumb zone below (rule 13). The `Benutzer` taps `Weiter` to the next `Ziel` and
   `Zurück` to an earlier one.
4. **Capture the route on a wide screen.** Every `Ziel` is a card in route order, each with its own
   ledger and its own `Wertungszone` group next to its position control (rule 14). The
   `Benutzer` moves between cards freely; a card that received the latest entry is active and the
   captured cards read as done.
5. **Skip and revisit.** The `Benutzer` moves to any other `Ziel` at any time — skipping ahead or
   going back — and adds `Schüsse` there. `Ziele` without `Schuss` are visibly uncaptured.
6. **Finish.** After the last `Ziel` the phone flow opens the `Rundenabschluss`, a summary with the
   two `Wertung` totals and the uncaptured `Ziele`; on a wide screen the same summary is reached
   from the finish action below the cards. The summary finishes nothing by itself: the `Runde` is
   completed by its finish action (rule 8). `Ziele` without `Schuss` have no result. An unfinished
   `Runde` stays open and can be resumed.
7. **Edit.** While an own `Runde` is in progress, the `Benutzer` corrects a mis-entry: add, change,
   or remove `Schüsse` and `Schusspositionen`, or change the date. The `Wertungen` recompute; the
   `Parkour` and its captured setup never change. Completing the `Runde` freezes it: afterward,
   its `Schüsse`, `Schusspositionen`, route, target names, and date are read-only.
8. **Delete.** The `Benutzer` deletes an own `Runde` after explicit confirmation; the `Runde` and
   its `Schüsse` are gone from history.
9. **Shoot a free `Runde`.** The `Benutzer` starts a `Freie Runde` under a name, picks no
   `Parkour`, adds a `Ziel` as they come to it ("Ziel 1", "Ziel 2", …), and logs `Schüsse` there
   exactly as on a `Parkour` round — a picked colour or an exact distance per shot. Removing a
   `Ziel` takes its `Schüsse` with it after a confirmation that names them.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Runde` | One logged pass of one `Benutzer` through one `Parkour` — or through a `Freie Runde`'s own `Ziele` — on one `Datum` ([charter](charter.md#domain-terms)). Owns its `Schüsse` and its setup ([`stammdaten.md`](stammdaten.md)). |
| `Freie Runde` | A `Runde` with no `Anlage` and no `Parkour`, whose `Ziele` the `Benutzer` adds and removes while shooting. Its own name identifies it. |
| `Freies Ziel` | A `Ziel` of a `Freie Runde`, created on the spot from a name and auto-named "Ziel 1", "Ziel 2", … when none is given. It exists only inside that `Runde`. |
| `Datum` | The day the `Runde` was shot. Part of the `Runde`, changeable by its owner. |
| `Schuss` | One registered arrow at one `Ziel`: `Pfeil` position in shooting order, `Wertungszone`, `Schussposition`. |
| `Schussposition` | Where a `Schuss` was shot from: a `Schussdistanz`, or the `Pflock` it was shot from — whose colour stands for the chosen colour on a `Freie Runde`. Exactly one of the two. |
| `Schussdistanz` | Distance a `Schuss` was shot from, as a positive whole number of meters. |
| `Zielfortschritt` | Captured `Ziele` of a `Runde` over all `Ziele` of its setup: the snapshot's `Ziele`, or the `Freie Runde`'s current `Ziele`. |
| `Rundenübersicht` | The bottom band that keeps a `Runde` in progress reachable from the other screens. |

Rules:

1. A `Runde` belongs to exactly one `Benutzer` and records one `Datum`, plus its setup (rule 17):
   one `Parkour` with its `Anlage` and the `Parkour-Snapshot` taken when the `Runde` is created
   ([`stammdaten.md`](stammdaten.md) rules 7–8), or a `Freie Runde`'s own name. The setup never
   changes afterwards; a wrong `Parkour` is fixed by deleting the `Runde` and logging it again.
2. The `Ziele` of a `Parkour` round are the `Ziele` of its captured setup, in visiting order (a
   `Freie Runde` holds its own, rule 18). While the `Runde` is in progress, the `Benutzer` may log
   them in any order: skip `Ziele`, revisit them, and add `Schüsse`. Completing the `Runde` freezes
   its route, `Schüsse`, and `Datum`; a completed `Runde` is read-only and may only be deleted.
   Within one `Ziel`, the `Schüsse` keep their shooting order (`Pfeil` 1, 2, 3, … per
   [`wertung.md`](wertung.md)).
3. Every `Schuss` records exactly one `Wertungszone` (`Spot`, `Kill`, `Body`, `Miss`) and at most
   one `Schussposition`. There is no upper limit on `Schüsse` per `Ziel`
   ([`wertung.md`](wertung.md) rule 1). Recording stays a single optional field so "position not
   recorded" is representable without a second, parallel state; the capture flow always fills it
   from what the slider represents when the `Schuss` is registered (rule 4).
4. `Schussposition`. When a `Schuss` has a `Schussposition`, it is one of two things and never
   both: a `Schussdistanz` — a positive whole number of meters — or the `Pflock` it was shot from.
   There is no fraction between stakes and no second derived field: the stored value is what the
   position control represented when the `Schuss` was registered. Whether that is a distance or a
   stake depends on the `Ziel`'s snapshot:

   - Every position resolves within the `Ziel`'s **effective** `Pflöcke`
     ([`stammdaten.md`](stammdaten.md) rule 6): the `Ziel`'s own set when its snapshot carries one,
     otherwise the `Parkour`-wide set. A `Ziel` with an own set is controlled by that set alone —
     the slider, the offered stakes, and the validator never see the `Parkour`-wide set — while
     every other `Ziel` keeps the `Parkour`-wide set. A stake or a configured `Distanz` outside the
     effective set is not a valid position for that `Ziel`.
   - When the `Ziel`'s effective `Pflöcke` carry configured `Distanzen`, the slider runs on a meter
     scale spanning them from the farthest to the nearest ([`stammdaten.md`](stammdaten.md) rule 6).
     Its position is the interpolated distance between the two adjacent `Pflöcke`, stored rounded to
     whole meters as a `Schussdistanz`. Moving it there stores a distance, including when it sits
     exactly on a `Pflock`: the stake's configured `Distanz` is stored as that distance.
   - When they carry no configured `Distanzen`, there is no meter scale. The slider offers the
     `Pflöcke` themselves, clings to the nearest one, and the `Schuss` stores that `Pflock`. No
     meter value is invented for a stake whose distance nobody configured.
   - A `Ziel` with exactly one `Pflock` has no slider. A one-tap `Schuss` stores that `Pflock`,
     and an exact distance can still be typed.

   An exact distance typed for a `Schuss` always stores a `Schussdistanz`, whatever the effective
   `Pflöcke` configure, so a `Benutzer` who stepped somewhere off-stake records where they
   actually stood. Registering or re-entering a position replaces the previous one, so a `Schuss`
   never carries both forms and never carries two of the same form.
5. Distance display. The shot ledger shows the stored `Schussposition` of every `Schuss`: a
   `Schussdistanz` as meters, a `Pflock` as that `Pflock`. The slider shows a meter value only on
   a `Ziel` whose effective `Pflöcke` configure `Distanzen`, and no meter value at all on a
   `Pflock` that has none ([`stammdaten.md`](stammdaten.md) AC-6). A `Schuss` that stored a
   `Schussdistanz` gets no `Pflock` indicator, because a distance is not a stake; a `Schuss` that
   stored a `Pflock` shows that stake and no meters. Interpolated and typed distances are both
   displayed as a bare meter value ("35 m"); the stored position does not distinguish an
   estimate from a directly entered value.

   Every displayed `Pflock` and every meter value belongs to the `Ziel`'s **effective** set
   (rule 4, [`stammdaten.md`](stammdaten.md) rule 6). A `Ziel` with its own `Pflöcke` therefore
   displays that own set and its own `Distanzen`, and never the `Parkour`-wide set; a `Ziel`
   without one displays the `Parkour`-wide set. The `Ziel` view names only the set that applies to
   it, so a reader never has to decide which of two lists a shown stake came from.
6. `Wertung` follows [`wertung.md`](wertung.md) unchanged: per `Ziel`, `Erster Treffer` and
   `Mehrschuss` computed from its `Schüsse`; a `Ziel` without `Schuss` has no result (not 0); each
   `Runde` total is the sum of its `Ziel` results per `Wertung`.
7. One running `Runde` per `Benutzer`. A `Benutzer` has at most one `Runde` in progress at a time:
   starting another is refused while one is in progress, and the app offers to resume the running
   one instead. Finishing or deleting the running `Runde` frees the slot. Two start requests issued
   at the same time leave exactly one `Runde` in progress.
8. `Rundenabschluss`. Reaching the end of the route opens a summary of the whole `Runde`: both
   totals, the `Zielfortschritt`, and how many `Ziele` are still uncaptured. The summary is a read
   of what was logged, not a write — it is shown *before* the `Runde` is finished, and the finish
   action lives in it. Only pressing that action marks the `Runde` complete, and completing is
   display-only state that changes no logged data. A finished `Runde` is read-only and may be
   deleted; an unfinished `Runde` can be resumed.
9. While a `Runde` is in progress, editing never changes the `Parkour` or the captured setup.
   Removing a `Schuss` renumbers the later `Schüsse` of that `Ziel`, so `Pfeil` stays contiguous
   from 1 and the points of the moved `Schüsse` follow their new `Pfeil`. Master-data changes made
   after the `Runde` was started never rewrite it ([`stammdaten.md`](stammdaten.md) rules 7–8); the
   `Runde` always displays the layout as it was shot. Completing freezes its route, `Schüsse`, and
   `Datum`; only deletion remains available afterwards.
10. Deleting removes the `Runde` with all its `Schüsse` — including from per-`Ziel` shot history —
    irreversibly, after explicit confirmation.
11. Only the owning `Benutzer` may view, edit, finish, or delete a `Runde`
    ([`konto.md`](konto.md) rule 10); requests from any other `Benutzer` fail like unauthenticated
    ones.
12. `Zielfortschritt`. A `Ziel` is captured once it holds at least one `Schuss`. The
    `Zielfortschritt` of a `Runde` is its captured `Ziele` over all `Ziele` of its setup. A
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
    - At the bottom, in the thumb zone, the position control holds where the next `Schuss` is shot
      from (rule 4), starting from the derived initial state (rule 16): the slider and a field for
      an exact `Schussdistanz`. It stays out of the ledger; a ledger row's expand action edits that
      row instead.
    - `Zurück` and `Weiter` move between `Ziele`. Advancing past the last `Ziel` opens the
      `Rundenabschluss` (rule 8) instead of wrapping around.
14. Desktop capture. On a wide screen the capture screen shows the route as a column of `Ziel`
    cards in route order, with no step progression:

    - Each card carries its `Ziel`, its shot ledger, and its own position control grouped
      with its `Wertungszone` button group, so the position a new `Schuss` will use reads as part
      of entering that `Schuss`.
    - The ledger rows carry the same contents as on a phone (rule 13), so a logged `Schuss` is
      corrected in place here too (NFR-4).
    - The card that received the latest entry is the active card. Every other card settles into
      its capture state (rule 12): one holding at least one `Schuss` reads as captured, one holding
      none keeps the uncaptured mark. So the first entry in a new card settles the card just left,
      and a card that was merely opened never reads as done.
    - One finish action sits below the cards and opens the `Rundenabschluss` (rule 8).
15. `Rundenübersicht` band. While the `Benutzer` has a `Runde` in progress and is not in the capture
    flow, every signed-in screen carries a bottom-anchored horizontal band naming the `Runde` — a
    `Parkour` round by its `Parkour`, a `Freie Runde` by its own name — and showing how long the
    `Runde` has been running (from its start) and its `Zielfortschritt` (rule 12). One tap resumes
    capture. The band is absent on the capture screen and on the `Rundenabschluss`, and reserves
    its own space, so no content is hidden behind it.
16. Initial slider state. What a `Schuss` stores is decided when it is registered: the value the
    slider or the distance field holds at that moment (rule 4). This rule only fixes where the
    slider *starts*, so a resumed or freshly opened `Ziel` does not begin from an arbitrary place:

    - The slider opens at the position of that `Ziel`'s most recently logged `Schuss` in this
      `Runde`, whether that `Schuss` holds a `Schussdistanz` or a `Pflock`.
    - When the `Ziel` holds no `Schuss`, it opens at the nearest `Pflock`, which also covers a
      `Ziel` with exactly one `Pflock`; a `Freie Runde` has no `Pflock`, so it opens with no
      position chosen (rule 19).
    - An opened ledger row (rule 13) starts at that row's own stored position instead, since it is
      editing that `Schuss`.

    The initial state is derived from the logged `Schüsse` and never stored separately, so it
    survives a resume of an in-progress round without extra state and cannot drift from what was recorded.
17. Setup. A `Runde` is either a `Parkour` round or a `Freie Runde`, never both and never neither.
    A `Parkour` round always names both its `Anlage` and its `Parkour`, as one origin the `Runde`
    records at the start (rule 1); a `Freie Runde` names neither and carries its own name instead.
    The origin is fixed at the start in both variants: a wrong `Parkour` is fixed by deleting the
    `Runde` and logging it again, and a `Freie Runde`'s name cannot be turned into a `Parkour`'
    `Anlage`.
18. A `Freie Runde` has no `Parkour`, no Anlage identity, and no `Pflöcke`. Its route is a
round-owned snapshot of targets with neutral round-local identities, in the order they were added.
While the `Runde` is in progress, targets are added and removed just in time while shooting.
Completing the `Runde` freezes this route as historical data; completed targets and names cannot
be added, removed, or renamed afterwards:

    - Adding a `Freies Ziel` appends it with the next ordinal and gives it the next unused name
      "Ziel N"; an explicit name replaces that default.
    - Removing a `Freies Ziel` deletes its `Schüsse` with it, in the same write, after explicit
      confirmation that names how many `Schüsse` go with it (rule 10's irreversibility applies).
    - The `Ziele` of a `Parkour` round are its snapshot's `Ziele` and never change (rule 2); only
      a `Freie Runde` may have a `Ziel` added or removed while it is in progress.
    - Completing a `Freie Runde` freezes its route. Its targets, names, and order remain as a
      historical snapshot and cannot be added, removed, or renamed afterwards.
19. `Schussposition` on a `Freie Runde`. Rule 4 still holds exactly: one exclusive value, a
    `Schussdistanz` or a `Pflock`. A `Freie Runde` configures no `Pflöcke`, so the `Pflock` form
    means the colour the `Benutzer` picked for that shot, and its only validity rule is that the
    colour is a well-formed colour (rule 4's effective-set rule has no set to resolve in). A
    `Schussdistanz` behaves exactly as on a `Parkour` round. There is no slider on a `Freie Runde`.
    The capture position control presents both choices together: a selectable colour palette and a
    numeric exact-meter field. Selecting a colour clears the meter; entering a valid positive whole
    meter clears the colour. The field visually shows the selected colour swatch when colour is the
    active form, otherwise it shows the exact meter value. An expanded shot row uses the same
    control, initially showing that shot's stored colour or meter value; it offers the colour
    palette rather than a slider, and edits only that shot.
20. During capture of a running `Freie Runde`, the `Freies Ziel` name in its card header is directly
    editable. Focusing the displayed name smoothly transitions it into a text input; leaving the
    field saves a changed, trimmed, non-empty name. Escape cancels the edit and restores the stored
    name. Saving preserves the target ordinal and its shots. A failed save keeps the name editable
    and exposes the error; a completed round keeps the name as plain text.
21. Every rule above that speaks of a `Runde`'s setup — the one-running-`Runde` rule (rule 7), the
    `Rundenabschluss` (rule 8), editing (rule 9), deletion (rule 10), ownership (rule 11),
    `Zielfortschritt` (rule 12), the capture presentations (rules 13–14), the `Rundenübersicht`
    band (rule 15), and the initial position state (rule 16) — holds for both variants. Where a
    rule names the `Parkour`, a `Freie Runde` names itself instead.

## Requirements

**Functional**

- FR-1: Start a `Runde` for one `Parkour` and one `Datum` (default today), recording the
  `Parkour-Snapshot` at creation (rule 1).
- FR-2: Present the `Ziele` in visiting order with an uncaptured marker, and let the `Benutzer`
  move between them freely — skip and revisit (rule 2).
- FR-3: Log unlimited `Schüsse` per `Ziel` in shooting order, each with `Wertungszone` and
  `Schussposition` (rules 3–4).
- FR-4: Set the `Schussposition` on a slider that stores a meter distance when the `Ziel`'s
  effective `Pflöcke` configure `Distanzen` and clings to a `Pflock` when they do not, and accept an
  exact `Schussdistanz` typed for a single `Schuss` (rules 4–5).
- FR-5: Show the `Wertung` per `Ziel` and the two `Runde` totals, computed per
  [`wertung.md`](wertung.md) (rule 6).
- FR-6: Hold at most one `Runde` in progress per `Benutzer`, refuse a second start, and offer to
  resume the running one (rule 7).
- FR-7: Summarise the `Runde` in the `Rundenabschluss` before finishing, complete it only on that
  summary's finish action, allow resuming only while unfinished, and keep completed rounds read-only
  (rule 8).
- FR-8: Edit `Datum`, `Schüsse`, and `Schusspositionen` of an own in-progress `Runde`, with
  `Wertungen` recomputed and deleted `Schüsse` renumbered; refuse these edits after completion
  (rule 9).
- FR-9: Delete an own `Runde` after explicit confirmation (rule 10).
- FR-10: Enforce ownership on every read and write of a `Runde` (rule 11).
- FR-11: Show the `Zielfortschritt` of a `Runde` (rule 12).
- FR-12: Present the capture screen on a narrow screen as one `Ziel` at a time with the strip,
  `Zielübersicht`, ledger, `Wertungszone` group, bottom position control, and
  `Zurück`/`Weiter` (rule 13).
- FR-13: Present the capture screen on a wide screen as `Ziel` cards with per-card ledger,
  per-card position control next to the `Wertungszone` group, the active-card marking, and
  one finish action (rule 14).
- FR-14: Show the `Rundenübersicht` band while a `Runde` is in progress and the `Benutzer` is
  outside the capture flow, and resume capture from it (rule 15).
- FR-15: Open a `Ziel`'s slider at its derived initial state — the `Ziel`'s last logged position, or
  the nearest `Pflock` when it has none — and an opened ledger row at that row's own position
  (rule 16).
- FR-16: Start a `Freie Runde` with no `Anlage` and no `Parkour`, under a name (rule 17).
- FR-17: Add a round-local `Ziel` just in time with the next ordinal and its default or given name,
  and remove one after a confirmation that names the `Schüsse` it takes with it while the `Runde`
  is in progress; completion freezes the route (rule 18).
- FR-18: Set a `Freie Runde`'s `Schussposition` as one exclusive value — a picked colour or an
  exact positive whole-meter `Schussdistanz` — with no slider (rule 19). Present the colour palette
  and meter field together, in both new-shot entry and per-shot editing; selecting one form clears
  the other and the field displays the active colour or meter.
- FR-19: Rename a running `Freies Ziel` directly in its header by focusing the name and saving a
  changed non-empty name on defocus; Escape cancels (rule 20).

**Non-functional**

- NFR-1: One-thumb logging. On the capture screen every interactive control has a touch target of
  at least 44×44 pixels ([charter](charter.md#accessibility)); the primary controls
  (`Wertungszone`, position control, `Ziel` navigation, finish) sit in the lower thumb zone and
  need no re-gripping. Registering a `Schuss` is one tap on a `Wertungszone`: no typing, no hover,
  no scrolling to a control, and it stays usable with gloves. Setting the position tolerates an
  imprecise finger — the slider accepts a coarse touch in both its meter-scaled and its
  stake-clinging mode, and typing a `Schussdistanz` is only ever optional. `Wertungszone` state is
  never encoded by colour alone.
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
- AC-5: Given a `Ziel` whose effective `Pflöcke` carry no configured `Distanzen`, when the slider
  is moved, then it clings to a `Pflock` of that set, the `Schuss` stores that `Pflock`, and
  redisplay shows the same `Pflock`; no meter value is stored or shown. Given a `Ziel` whose
  effective `Pflöcke` do carry `Distanzen`, moving the slider stores a positive whole-meter
  `Schussdistanz` and redisplay puts the slider at the same place, and resting it on a `Pflock`
  stores that stake's configured `Distanz` rather than the stake. A `Ziel` with its own `Pflöcke`
  is judged by that own set alone: its slider and its stored `Pflock` never come from the
  `Parkour`-wide set, and a `Pflock` or `Distanz` of the `Parkour`-wide set that the own set does
  not carry is refused for it.
- AC-6: Given adjacent effective `Pflöcke` with `Distanz` 40 m (farther) and 30 m (nearer), then
  a slider position halfway between them stores 35 m and displays it as "35 m", while the stakes'
  own 40 m and 30 m show at their positions. A `Pflock` without a configured `Distanz` gets no
  meter value, and a `Schuss` that stored a `Schussdistanz` shows no `Pflock` indicator. When the
  `Ziel` carries its own `Pflöcke`, the same holds for the own set's order and `Distanzen` even
  where the `Parkour`-wide set configures different values for the same colour.
- AC-7: Given a `Ziel` whose effective set holds exactly one `Pflock`, then no slider is offered, a
  `Schuss` registered by the `Wertungszone` group stores that `Pflock` as its `Schussposition`, and
  an exact `Schussdistanz` can still be typed for a `Schuss` (AC-15). A `Ziel` whose own set holds
  one `Pflock` behaves the same, judged by its own set and not by the `Parkour`-wide one. A
  `Ziel`'s own `Pflöcke` never appear on another `Ziel` of the same `Runde`.
- AC-8: When an in-progress `Runde` is finished with unlogged `Ziele`, then those `Ziele` show no
  result and the totals cover only the logged `Ziele`. A completed `Runde` cannot be resumed and
  accepts no further `Schüsse`.
- AC-9: While an owned `Runde` is in progress, when the owner adds, changes, or removes a `Schuss` or
  `Schussposition`, or changes its `Datum`, then the `Wertungen` recompute and the captured setup is
  unchanged. After completion, attempts to add, change, or remove a `Schuss` or `Schussposition`,
  or change the `Datum`, are refused; the completed `Runde` remains readable and may be deleted.
- AC-10: When the owner deletes a `Runde` after confirming, then the `Runde` and its `Schüsse` are
  gone, including from per-`Ziel` shot history; without confirmation nothing is deleted.
- AC-11: When another `Benutzer` requests a `Runde` that is not its own, then the request fails with
  403 (401 when unauthenticated), and the same request with the owning `Benutzer` succeeds.
- AC-12: On the capture screen every interactive control has a touch target of at least 44×44
  pixels; a `Schuss` is registered by one tap on a `Wertungszone` with no typing; the position of
  a `Schuss` is settable without precise pointing in both slider modes and an exact distance is
  only ever optional typing; and the whole journey on a phone (start, log a `Ziel` with `Schüsse`
  and position, skip, finish) is completable with one thumb while wearing gloves.
- AC-13: Given a `Benutzer` with a `Runde` in progress, when another `Runde` is started, then the
  start is refused, the running `Runde` is unchanged, and the app offers to resume it; after
  finishing or deleting the running `Runde` a new start succeeds; two start requests issued at the
  same time leave exactly one `Runde` in progress.
- AC-14: Given a `Runde` whose snapshot has 5 `Ziele`, when 2 of them hold at least one `Schuss`,
  then the `Zielfortschritt` reads 2 of 5 in the `Rundenübersicht` band and on the capture screen;
  removing the last `Schuss` of one of them returns it to 1 of 5.
- AC-15: Given a `Ziel` with no `Schuss`, when the first `Schuss` is registered, then its position
  is the nearest `Pflock`; given a `Ziel` whose most recent `Schuss` holds a `Schussdistanz` of
  35 m or the `Pflock` GELB, then the next one-tap `Schuss` starts from that same value. When a
  distance is typed instead, the `Schuss` stores exactly that positive whole-meter
  `Schussdistanz`, displays it without the estimate label, and holds no `Pflock`; a non-positive or
  fractional value is rejected.
- AC-16: Given an in-progress `Runde` with a `Ziel` holding `Schüsse` at `Pfeil` 1–4, when the
  owner deletes the `Schuss` at `Pfeil` 2, then the remaining three are renumbered 1–3, their
  points follow their new `Pfeil`, and the `Ziel` and `Runde` `Wertungen` recompute. The same
  deletion on a completed `Runde` is refused.
- AC-17: On a narrow screen the capture screen shows exactly one `Ziel` at a time, with both
  `Runde` totals and the `Zielfortschritt` in a pinned strip, a `Zielübersicht` that expands into
  a selectable list of every `Ziel`, a shot ledger whose every row offers a `Wertungszone` select,
  its points, its position, delete, and expand, a `Wertungszone` group that registers a `Schuss`
  in one tap, and the position control at the bottom; `Zurück` and `Weiter` move between
  `Ziele` and advancing past the last `Ziel` opens the `Rundenabschluss`, which a mid-route
  `Weiter` never opens.
- AC-18: On a wide screen the capture screen shows every `Ziel` as a card in route order with its
  own ledger and its own position control grouped with its `Wertungszone` group; the card
  that received the latest entry is active, a card holding at least one `Schuss` reads as
  captured, a card holding none keeps the uncaptured mark, and finishing is reached from the single
  action below the cards.
- AC-19: While a `Runde` is in progress and the `Benutzer` is on any signed-in screen outside the
  capture flow, a bottom band shows the `Parkour`, the elapsed time since the `Runde` started, and
  its `Zielfortschritt`, and one tap opens capture at the first uncaptured `Ziel`; on the capture
  screen and the `Rundenabschluss` the band is absent and nothing is drawn under it.
- AC-20: When the `Benutzer` reaches the end of the route, then the `Rundenabschluss` shows both
  totals, the `Zielfortschritt`, and the number of uncaptured `Ziele` while the `Runde` is still in
  progress; the `Runde` becomes complete only when the finish action in that summary is pressed,
  so opening the summary and leaving without pressing it leaves the `Runde` resumable, and pressing
  it changes no `Schuss`, `Wertung`, or `Datum`.
- AC-21: Given a `Benutzer` with no `Runde` in progress, when a `Freie Runde` is started with a
  name and no `Parkour`, then it opens with that name, no `Anlage` and no `Parkour` at all, no
  `Pflöcke`, and no `Ziel`; the one-running-`Runde` rule holds against a `Parkour` round in
  progress the same way round.
- AC-22: Given a running `Freie Runde`, when a round-local `Ziel` is added without a name, then it
  is appended with the next ordinal and named "Ziel N" from that ordinal, an explicit name replaces
  that default, and when one is removed after confirmation its `Schüsse` are gone from the `Runde`
  and from the ledger in the same write; removing a `Ziel` from a `Parkour` or completed round is
  refused. Completing a `Freie Runde` freezes its target list and names as the round's historical
  snapshot.
- AC-23: Given a `Freies Ziel`, when a `Schuss` is registered with a colour, then its stored
  `Schussposition` is that `Pflock` colour and holds no distance, and when one is registered with
  an exact distance, then it stores that positive whole-meter `Schussdistanz` and holds no colour;
  a malformed colour or a non-positive distance is refused, and a later position replaces the
  previous one so no `Schuss` ever carries both.
- AC-24: Given a `Freie Runde` whose `Freies Ziel` hold `Schüsse`, then each `Ziel`'s `Erster
  Treffer` and `Mehrschuss` equal the results of [`wertung.md`](wertung.md), the `Runde` totals are
  the sums of its `Ziel` results, a `Ziel` without `Schuss` shows no result, and adding or removing
  a `Ziel` recomputes both.
- AC-25: Given a running `Freie Runde`, then the `Rundenübersicht` band and the `Runden` list name
  it by its own name, and the band and capture screen show the same `Zielfortschritt` as for a
  `Parkour` round, counted over the `Freie Runde`'s current `Ziele`.
- AC-26: Given an owned running `Freie Runde` and one of its round-local targets, when the owner
  gives that target a non-empty name, then the name is persisted and shown in capture; renaming is
  refused for a completed `Runde`, a `Parkour` round, or a target not in that `Runde`, and the
  existing ownership and authentication rules apply.
- AC-27: Given a running `Freie Runde`, when the user focuses a `Freies Ziel` name in its card
  header, then the plain name smoothly becomes an input; when focus leaves after a changed,
  non-empty name, the trimmed name is saved, while an unchanged value is not submitted, Escape
  cancels, and a failed save leaves the input open with an error. A completed round shows plain text.
- AC-28: Given a `Freie Runde` capture position or an expanded shot row, when the user selects a
  colour, then that colour is the sole stored form and the meter field shows its swatch; when the
  user enters a valid positive whole-meter value, then that exact distance is the sole stored form
  and the field shows its number. Choosing either form clears the other; invalid/incomplete meter
  edits do not replace the last valid stored form, and the shot-row control opens with that shot's
  current form and offers the same colour palette, not a slider.

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

No open questions remain. The bow class question is deferred, not open
([deferred](#bow-class-per-runde)), and every other question raised in review was answered:

Resolved (PO 2026-11-06, spec review): `Schussposition` is a single optional value — a distance or
a `Pflock` — not an `Anteil` between stakes. The UI converts a slider position to and from that
value and clings to the nearest `Pflock` when the snapshot configures no `Distanzen`, storing the
`Pflock`; a stored distance gets no `Pflock` indicator. A `Runde` is editable forever and deletion
is irreversible for MVP with no restore. The nearest `Pflock` is the initial slider state, not a
stored default. Also resolved then: one `Runde` in progress per `Benutzer`; capture is one `Ziel`
at a time on a phone and a card per `Ziel` on a wide screen; the capture strip shows the `Runde`
totals and progress; the end of the route shows a summary before the `Runde` is finished; deleting
a `Schuss` renumbers the later `Schüsse`.

Resolved (PO 2026-11-06, follow-up): only a `Schussdistanz` feeds meter-based metrics. A
`Pflock`-form `Schussposition` does not contribute, even where the snapshot configures a `Distanz`
for that stake — the distance was never recorded for the shot, so it is not imputed.

Resolved (PO 2026-11-11): `Freie Runde` is the ongoing implementation scope, including JIT
`Ziel` management. A `Freies Ziel` can be renamed only while its own `Freie Runde` is in progress;
blank or whitespace-only names are rejected. The update preserves the `Ziel` ordinal and its
logged `Schüsse`. The separate issue split and approval gate proposed during implementation were
rejected; delivery remains within the existing Runde work.

## Verification

Domain unit tests in the backend core cover the `Schussposition` value (the two exclusive forms of
rule 4, the single-`Pflock` case, and rejection of a non-positive or fractional `Schussdistanz`),
the effective per-`Ziel` `Pflöcke` set that a position resolves within, the `Pfeil` renumbering of
rule 9, and the `Wertung` hand-off to [`wertung.md`](wertung.md).
Use-case conformance tests cover start, walk, skip/revisit, finish, edit, and delete (AC-1 to
AC-10), the one-running-`Runde` rule including a concurrent double start (AC-13), `Zielfortschritt`
(AC-14), and the initial slider state (AC-15). API e2e tests cover ownership enforcement (AC-11)
and the refused second start (AC-13). The slider's two behaviours and its redisplay (AC-5, AC-6),
the two capture presentations (AC-12, AC-17, AC-18), and the `Rundenübersicht` band (AC-19) are
manual checks: a phone-sized viewport for one-thumb reach and touch targets, a wide viewport for the
card layout, and both for layout parity (NFR-4). The `Rundenabschluss` completion rule (AC-20) is a
use-case conformance test. The `Freie Runde` setup (setup typestate rules 17–21, AC-21 to AC-28) is
proven at three layers: domain unit tests for setup variants and `Freies Ziel` ordinals, colours,
and name changes; use-case conformance tests for start, JIT `Ziel` add/remove/rename (including
ownership and round-state restrictions) with persistence and cascade; and API e2e tests for
`POST /runden/frei`, JIT `Ziel` routes, ownership, invalid names, rename persistence, and a refused
colour. The free capture screen, including its running-only rename action, is a manual check like the
`Parkour` capture screen. AC-27 and AC-28 are verified with frontend type/lint checks and a manual
check of focus/blur and shot-row editing on a phone-sized viewport.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-11-12 | A `Freie Runde` route uses neutral round-local target identities and is frozen on completion. Completed targets and names are historical and cannot be edited. | TimerErTim (PO) |
|------|----------|----|
| 2026-10-23 | ~~Slider storage: `Anteil` fraction 0..1 between the two adjacent `Pflöcke`; meter estimate shown when `Pflock` `Distanzen` exist.~~ Superseded 2026-11-06 by the single optional distance. | TimerErTim (PO) |
| 2026-10-23 | Bow class per `Runde` is not in MVP; deferred and tracked in issue #23. | TimerErTim (PO) |
| 2026-10-23 | ~~A detent on an interior `Pflock` is stored as `Anteil` 1 of the segment whose nearer end is that `Pflock`, so every position has exactly one representation.~~ Superseded 2026-11-06: no `Anteil` is stored. | Agent (MiMo) |
| 2026-10-23 | `Datum` is editable; `Parkour` is fixed at start, a wrong choice is fixed by delete and re-log (snapshot is taken at creation). | Agent (MiMo) |
| 2026-11-06 | One `Runde` in progress per `Benutzer`; a second start is refused and offers to resume the running one. | TimerErTim (PO) |
| 2026-11-06 | The direct number field stores an exact positive whole-meter `Schussdistanz` on the `Schuss`; a `Schussposition` is one form or the other, never both. | TimerErTim (PO) |
| 2026-11-06 | The `Rundenübersicht` band sits at the bottom of every signed-in screen except the capture screen and shows elapsed time and `Zielfortschritt`. | TimerErTim (PO) |
| 2026-11-06 | Capture is one `Ziel` at a time with a pinned strip, `Zielübersicht`, and bottom position control on a phone, and `Ziel` cards with per-card position control on a wide screen; the difference is presentation only. | TimerErTim (PO) |
| 2026-11-06 | The capture strip shows both `Runde` totals and the `Zielfortschritt`; the open `Ziel` shows its own two values. | TimerErTim (PO) |
| 2026-11-06 | Deleting a `Schuss` renumbers the later `Schüsse` of that `Ziel`, so `Pfeil` stays contiguous. | TimerErTim (PO) |
| 2026-11-06 | Reaching the end of the route opens a read-only `Rundenabschluss`; only its finish action completes the `Runde`. The summary never finishes implicitly. | Agent (Cursor) |
| 2026-11-06 | `Schussposition` is one optional value, not an `Anteil`: a `Schussdistanz` in whole meters or a `Pflock`. The UI converts a slider position to and from that value; with no configured `Distanzen` the slider clings to the nearest `Pflock` and stores the `Pflock`; a stored distance shows no `Pflock` indicator. | TimerErTim (PO) |
| 2026-11-06 | A `Runde` is editable forever; deletion is irreversible for MVP with no restore. | TimerErTim (PO) |
| 2026-11-06 | The nearest `Pflock` is the initial slider state for a `Ziel` with no logged `Schuss`. | TimerErTim (PO) |
| 2026-11-06 | Only a `Schussdistanz` contributes to meter-based metrics; a `Pflock`-form `Schussposition` does not, even when the snapshot configures a `Distanz` for that stake. | TimerErTim (PO) |
| 2026-11-06 | Rule 16 is named "initial slider state": the derived value seeds the control, while what a `Schuss` stores is decided at registration. | Agent (Cursor) |
| 2026-11-07 | Interpolated and typed distances use the same bare meter display (for example, "35 m"); no estimate label is shown because the single stored position cannot preserve that distinction. | TimerErTim (PO) |
| 2026-11-13 | A completed `Runde` is immutable: its `Schüsse`, positions, and `Datum` cannot be changed, and no `Schuss` can be registered; it remains readable and may be deleted. | TimerErTim (PO) |
| 2026-11-07 | ~~A closed `Runde` remains correctable: its `Schüsse` and `Datum` can be edited, but no new `Schuss` can be registered after completion.~~ Superseded 2026-11-13 by the PO's decision that completion freezes the entire round. | TimerErTim (PO) |
| 2026-11-13 | Each captured `Ziel` owns its effective `Pflöcke` as colour plus optional distance; inheritance from a Parkour is resolved when the Runde starts. There is no Runde-wide Pflock set. | TimerErTim (PO) |
| 2026-11-13 | Provenance is only the `AnlageId`/`ParkourId` pair; the `Runde` stores its name separately. A completed `Runde` has optional provenance and is immutable. | TimerErTim (PO) |
| 2026-11-10 | A `Freie Runde`'s `Ziele` are added and removed just in time, auto-named "Ziel 1", "Ziel 2", …; removing one deletes its `Schüsse` after a confirmation that names them. Its `Schussposition` stays the one exclusive value: a picked colour (the `Pflock` form) or an exact positive whole-meter `Schussdistanz`; there is no slider. | TimerErTim (PO) |
| 2026-11-11 | Freie Runde remains in the ongoing implementation scope. An owned running Freie Runde may rename one of its Freie Ziele; whitespace-only names are invalid. The rename preserves ordinal and Schüsse. | TimerErTim (PO) |
| 2026-11-11 | Freies Ziel names edit inline on focus and save on defocus; colour and exact meter are presented as mutually exclusive choices in the capture control and expanded shot editor. | TimerErTim (PO) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-23 | Initial draft from issue #11 and PO decisions. | Agent (MiMo) |
| 2026-11-06 | Redesigned capture: one running `Runde` per `Benutzer` with a bottom `Rundenübersicht` band, the mobile step flow and the desktop card layout, exact `Schussdistanz` entry, a `Rundenabschluss` summary before finishing, `Zielfortschritt`, and `Pfeil` renumbering on delete. Renamed `Schütze` to `Benutzer` and `Parcours` to `Parkour` to match the charter and `stammdaten.md`. | Agent (Cursor) |
| 2026-11-06 | Spec review: replaced the `Anteil` position with one optional value (distance or `Pflock`), rewrote rules 3–5, AC-5/AC-6/AC-15, and renamed rule 16 to "initial slider state". Resolved Q-2 to Q-5 (editable forever, irreversible deletion, nearest-`Pflock` initial state, exact meters) and folded the remaining metric question into Q-1. Updated the charter's distance decision to match. | Agent (Cursor) |
| 2026-11-06 | Follow-up review: Q-1 resolved — a `Pflock`-form position is excluded from meter-based metrics rather than imputed from the stake's configured `Distanz`. No open questions remain. | Agent (Cursor) |
| 2026-11-07 | Capture implementation decisions: use the bare meter display for interpolated distances, and preserve correction of `Schüsse` and `Datum` after a `Runde` is closed while refusing new `Schüsse`. | TimerErTim (PO) |
| 2026-11-06 | Approved. Status set to `PO-approved` on the PO's explicit instruction; implementation is tracked in #86. | TimerErTim (PO) |
| 2026-11-10 | Added the per-Ziel `Pflöcke` set (rules 4–5, AC-5 to AC-7): a `Schussposition` on a `Ziel` with its own set resolves within that set, the slider and the ledger use the effective set, and each `Ziel` displays only the set that applies to it. | Agent (Cursor) |
| 2026-11-10 | Added the `Freie Runde` (rules 17–20, FR-16 to FR-18, AC-21 to AC-25): a `Runde` with no `Anlage` and no `Parkour`, JIT `Freies Ziel`, a colour-or-distance `Schussposition` with no slider, and the `Wertung`/band/history rule extended to both setups. Set to `Draft` while the change is open; it returns to `PO-approved` only on the PO's explicit approval. | Agent (Cursor) |

## Appendix A: implementation sketch (non-normative)

- A `Runde` is an aggregate owning its `Schüsse`; it references `Ziel` identities (stable per
  [`stammdaten.md`](stammdaten.md) rule 9) and stores its `Parkour-Snapshot` as an immutable copy.
- A `Schuss` stores `Pfeil`, `Wertungszone`, and the `Schussposition` as one exclusive variant: a
  whole-meter `Schussdistanz` or a `Pflock` reference. The two forms keep storage unambiguous, so
  "the distance" and "the stake" can never disagree.
- The slider lives entirely in the UI: it reads the snapshot's configured `Distanzen` to decide
  between a meter scale and stake-clinging, and it writes one of the two stored forms. No slider
  fraction, interpolation result, or meter estimate is persisted.
- The one-running-`Runde` rule is a constraint on the owning `Benutzer`, enforced in the same
  transaction as the start (a partial unique index on in-progress `Runden` per owner, or an
  equivalent check), never only in the UI.
- `Pfeil` is the position of a `Schuss` within its `Ziel` and is rewritten on removal (rule 9), so
  the score of a moved `Schuss` changes with it; the domain owns that rewrite.
- `Wertungen` are computed by the domain per [`wertung.md`](wertung.md) from the `Schüsse`; the
  `Runde` totals are the sums of its `Ziel` results.
- `Zielfortschritt` and the initial slider state (rules 12 and 16) are read from the logged
  `Schüsse`: no capture cursor, "last visited `Ziel`", or remembered slider position is stored.
- Deleting a `Runde` removes the aggregate; per-`Ziel` history queries (issue #12) read `Schüsse`
  through their `Ziel` reference.
