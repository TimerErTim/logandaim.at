---
id: SPEC-wertung
title: Classic 3-arrow scoring (first-hit and multi-shot)
status: In delivery
owner: TimerErTim
issues: [8]
superseded_by: null
---

# Wertung — Classic 3-arrow scoring

## Problem

Archers need the score of each Ziel and each Runde computed the same way every time, following the
classic 3D 3-arrow table that Austrian parcours use. Archers also want to log every arrow they
shoot, including extra misses and follow-up hits, for statistics.

## Outcome

The archer logs any number of Schüsse at a Ziel. The app computes two Wertungen from the same
Schüsse: **Erster Treffer** (first-hit, the classic rule) and **Mehrschuss** (multi-shot, every
arrow counts). The UI can show either or both.

## Roles

- Schütze: enters Schüsse per Ziel, sees points in both Wertungen.

## Scope

**In scope**

- Points per arrow position and Wertungszone (table below).
- Two Wertungen per Ziel and per Runde: Erster Treffer and Mehrschuss.
- First-arrow hit flag.

**Out of scope**

- WA 2-arrow, IFAA modes (later spec).
- Distance, Pflock, and shooting position (separate spec).
- Persistence and UI (separate specs).

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Wertungszone` | `Spot` (inner kill), `Kill`, `Body`, `Miss`. |
| `Treffer` | A Schuss in `Spot`, `Kill`, or `Body`. |
| `Pfeil` | Position of a Schuss at one Ziel: 1, 2, 3, 4, … |
| `Erster Treffer` | Wertung that counts only the first Treffer at a Ziel. |
| `Mehrschuss` | Wertung that counts every Schuss at a Ziel. |

Points per Schuss:

| Pfeil | Spot | Kill | Body | Miss |
|:-----:|:----:|:----:|:----:|:----:|
| 1 | 20 | 18 | 16 | 0 |
| 2 | 14 | 12 | 10 | 0 |
| 3 | 8 | 6 | 4 | 0 |
| 4 and later | 0 | 0 | 0 | 0 |

Rules:

1. A Ziel has a sequence of one or more Schüsse in shooting order. There is no upper limit.
   Any Wertungszone may follow any other.
2. Erster Treffer: the Ziel scores the points of the first Treffer by its Pfeil position. All later
   Schüsse score nothing. A Ziel without Treffer scores 0.
3. Mehrschuss: the Ziel scores the sum of the points of every Schuss by its Pfeil position.
4. Schüsse at Pfeil 4 or later score 0 in both Wertungen. They count in hit statistics.
5. A Ziel with no Schuss entered has no result yet (not 0).
6. Each Runde total is the sum of the Ziel results, per Wertung.

## Requirements

**Functional**

- FR-1: Compute Erster Treffer and Mehrschuss points for any non-empty sequence of Schüsse.
- FR-2: Report whether the Ziel was hit with the first arrow.
- FR-3: Sum Ziel results into Runde totals per Wertung.

**Non-functional**

- NFR-1: Rules live in the backend domain. The frontend may preview points but the backend result is authoritative.

## Acceptance criteria

- AC-1: For Pfeil 1–3, every cell of the points table is produced for its Pfeil and Wertungszone.
- AC-2: `[Miss, Miss, Miss]` scores 0 in both Wertungen.
- AC-3: Erster Treffer: `[Kill]` = 18; `[Miss, Spot]` = 14; `[Miss, Miss, Body]` = 4; `[Body, Kill]` = 16.
- AC-4: Mehrschuss: `[Kill, Kill, Spot]` = 38; `[Body, Kill]` = 28; `[Miss, Spot]` = 14.
- AC-5: Schüsse at Pfeil 4 or later score 0: `[Miss, Miss, Miss, Kill]` = 0 in both Wertungen.
- AC-6: An empty sequence has no result. Any non-empty sequence is accepted, regardless of length.
- AC-7: First-arrow hit is true only when Pfeil 1 is a Treffer.
- AC-8: The Runde total per Wertung equals the sum of its Ziel results; Ziele without result are skipped.

## Risks and open questions

- [ ] Q-1: Do Austrian parcours use the same 3-arrow table everywhere? Assumed yes for MVP.

## Verification

Domain unit tests in the backend core, one per AC.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-21 | MVP scoring is classic 3-arrow only. | TimerErTim |
| 2026-10-21 | Log unlimited Schüsse per Ziel; compute Erster Treffer and Mehrschuss side by side. | TimerErTim |
| 2026-10-21 | Mehrschuss scores each Schuss by its Pfeil position (18 + 12 + 8 for Kill, Kill, Spot). | TimerErTim |
| 2026-10-21 | Pfeil 4 and later score 0 in both Wertungen. | TimerErTim |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-21 | Initial spec from PRD section 3.2. | Agent (Claude) |
| 2026-10-21 | Unlimited Schüsse, two Wertungen. Approved. | TimerErTim |
