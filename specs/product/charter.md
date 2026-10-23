---
id: SPEC-charter
title: logandaim.at product charter
status: PO-approved
owner: TimerErTim
issues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
superseded_by: null
---

# logandaim.at — Product Charter

## Vision

logandaim.at is an Austria-wide online logbook for 3D archery, optimized for 3D targets and
parcours. Archers log rounds target by target, including the exact shooting distance between
stakes, and see how their accuracy develops over time. Parcours operators get a standard way to
describe their courses.

Source input: [`../3d_archery_tracking_austria_product_requirements_document.md`](../3d_archery_tracking_austria_product_requirements_document.md).

## Problem

Existing tools are score cards or tournament software. They do not track accuracy per parcours
and per target over time, and they do not record distances between the fixed stake positions.

## Roles

| Role | MVP | Description |
|------|-----|-------------|
| Schütze (archer) | Yes | Registered user. Logs rounds, reviews personal history. Casual and power users share this role; power features come later. |
| System-Administrator | Yes | Maintains master data. Seeds the pilot Anlagen, Parcours, Ziele, and Pflöcke. |
| Anlagenbetreiber (operator) | No | Manages own Anlage and Parcours. Later milestone. |
| Turnierleiter (tournament lead) | No | Events, group scoring, leaderboards. Future. |
| Gast (guest) | No | Logging without account. Not planned before accounts are stable. |

## Domain terms

| Term | Meaning |
|------|---------|
| `Anlage` | Physical site of a club or commercial operator. |
| `Parcours` | Predefined ordered sequence of Ziele at an Anlage (for example "Große Runde, 32 Ziele"). The PRD calls this Route / Parcours-Layout. |
| `Ziel` | One 3D animal target with master data: animal, target group 1–4, Pflock configuration. |
| `Pflock` | Colored stake marking a shooting position (for example Weiß, Blau, Rot, Gelb), ordered far to near per Ziel. |
| `Runde` | One logged pass of an archer through one Parcours on one date. The PRD calls this Session. |
| `Schuss` | One registered arrow at one Ziel: arrow number, Wertungszone, shooting position. |
| `Wertungszone` | `Spot`, `Kill`, `Body`, or `Miss`. |

## MVP scope

Decisions by PO on 2026-10-21:

1. **Master data by admin.** The System-Administrator seeds 2–3 pilot Anlagen with Parcours, Ziele,
   and Pflöcke. Archers do not create or edit master data in the MVP.
2. **Accounts, online-first.** Archers need an account. The app works online. Offline capture and
   background sync (PWA, IndexedDB) are the next milestone after MVP.
3. **Scoring: classic 3-arrow only**, shown as two Wertungen (Erster Treffer and Mehrschuss) from
   the same logged Schüsse. See [`wertung.md`](wertung.md). WA 2-arrow and IFAA modes come later.
4. **Web only.** The inherited Tauri desktop shell is removed. A native or desktop shell may return
   later, so architecture must keep the core and API independent of the delivery shell
   (see [`../decisions/0001-baseline-architecture.md`](../decisions/0001-baseline-architecture.md)).
5. **Distance.** A slider between the Pflöcke of a Ziel. If Pflöcke have a configured distance
   (optional, set by admin now and by the Anlagenbetreiber later), the app also shows a meter estimate.

MVP capabilities:

| Capability | Spec |
|------------|------|
| Classic 3-arrow scoring (Erster Treffer, Mehrschuss) | [`wertung.md`](wertung.md) |
| Account registration and login | [`konto.md`](konto.md) |
| Data protection, consent, and legal obligations | [`datenschutz.md`](datenschutz.md) |
| Operations, security, and launch readiness | [`betrieb.md`](betrieb.md) |
| Master data: Anlage, Parcours, Ziel, Pflock (admin seeding) | to be written |
| Log a Runde target by target, with distance slider between Pflöcke | to be written |
| Personal history: scores and first-arrow hit rate per Parcours over time | to be written |
| Remove YAMS branding and domain | [GitHub milestone "M0 Rebrand"](https://github.com/TimerErTim/logandaim.at/milestones) |

## Non-goals for MVP

- Operator self-service, tournaments, leaderboards.
- Global benchmarks across users (needs privacy and consent design first).
- Equipment tracking, photo log, weather and terrain data, GPS target detection.
- Damage reports, lost-arrow board, traffic heatmaps, announcements.
- WA and IFAA scoring modes.
- Offline-first capture (next milestone).

## Quality bars

**Draft — pending PO approval.** These are cross-cutting acceptance conditions for every
user-visible capability, not a separate feature. They are written here because the charter is the
only spec that covers more than one capability.

- [ ] Q-5: Is the accessibility bar a launch gate for the M1 milestone, or may it ship shortly
  after launch?

### Accessibility

Issues: #48.

The app is used outdoors, one-handed, often with gloves. The core journeys (registration, login,
logging a `Runde`, viewing history) meet WCAG 2.2 AA:

1. Interactive targets are at least 44 by 44 pixels on the logging screen.
2. Core journeys are completable by keyboard alone with a visible focus indicator.
3. Form fields have programmatic labels; validation errors are announced to assistive technology.
4. Text and essential controls meet AA contrast and stay legible on a phone in daylight.
5. `Wertungszone` state is never encoded by colour alone.
6. An automated accessibility check runs in CI and fails on new serious violations; a manual pass
   covers the four core journeys.

### Language and copy

Issues: #49.

1. No user-facing string is hardcoded in a component; all text comes from one locale source.
2. Dates, numbers, and distances render in `de-AT` format.
3. Domain terms match the glossary of this charter; a lint or test rejects YAMS names, veterinary
   terms, and stray English UI text.
4. Validation and error messages come from one source and use the same wording for the same
   condition everywhere.
5. The locale setup can add a second locale later without moving strings again.

## Import

**Draft — pending PO approval.** See issue #52.

An archer may bring existing paper or spreadsheet history in, so the first session is not an empty
profile.

1. A documented CSV schema defines the columns for `Runde`, date, `Parcours`, `Ziel`, `Pfeil`
   position, and `Wertungszone`.
2. Validation reports every failing row with a row number and a reason; nothing is written on
   failure.
3. A dry-run preview shows what would be imported, including computed `Wertungen`, before
   committing.
4. Imported `Runden` are marked as imported and are distinguishable in history.
5. The import references existing master data only; an unknown `Parcours` or `Ziel` fails with a
   clear message.
6. Re-importing the same file does not duplicate `Runden`.

Open questions:

- [ ] Q-6: Does import belong to MVP onboarding or a later milestone?

## Milestones

| Milestone | Goal |
|-----------|------|
| M0 Rebrand | No YAMS name, logo, domain, or Tauri shell remains. Build, check, and test pass. |
| M1 MVP | Archer registers, picks a pilot Parcours, logs a Runde, sees personal history. |
| M2 Offline | Full Runde capture without network, background sync. |
| Later | Power-user features, operators, benchmarks, tournaments. |

## Open questions (not blocking M0)

- [ ] Q-1: Which 2–3 pilot Anlagen (for example Oberösterreich, Steiermark)? Needed before M1 seeding.
- [x] Q-2: Distance input. Answered 2026-10-21, see MVP scope item 5.
- [ ] Q-3: Privacy: which data is visible to other users in MVP? Default assumption: none.
- [ ] Q-4: Bow class (Blankbogen, Recurve, Langbogen, Compound) per Runde in MVP?

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-21 | MVP master data seeded by admin; archers only log rounds. | TimerErTim |
| 2026-10-21 | Accounts required; online-first; offline sync is next milestone. | TimerErTim |
| 2026-10-21 | MVP scoring: classic 3-arrow only. | TimerErTim |
| 2026-10-21 | Remove Tauri for now; keep shell-agnostic architecture so it can return. | TimerErTim |
| 2026-10-21 | Distance: slider between Pflöcke; meter estimate when Pflöcke have an optional configured distance. | TimerErTim |
| 2026-10-21 | Two Wertungen (Erster Treffer, Mehrschuss) from unlimited logged Schüsse. | TimerErTim |
| 2026-10-21 | Code prefix `logandaim`: `logandaim-*` crates, `LOGANDAIM_*` env vars, `Logandaim*` type names. | TimerErTim |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-21 | Initial charter from PRD and PO decisions. | Agent (Claude) |
| 2026-10-21 | Approved. | TimerErTim |
| 2026-10-23 | Linked the konto.md draft for the accounts capability. | Agent (MiMo) |
| 2026-10-23 | Added Draft "Quality bars" (accessibility, language) and "Import" sections, and linked the `datenschutz.md` and `betrieb.md` drafts. Approved MVP scope text unchanged; the new sections are pending PO approval. | Agent (Cursor) |
