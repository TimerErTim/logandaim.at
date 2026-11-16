---
id: SPEC-statistik
title: Personal statistics and insights
status: Draft
owner: TimerErTim
issues: [12]
superseded_by: null
---

# Statistik — personal statistics and insights

## Problem

A `Benutzer` records `Runden`, `Ziele`, `Schüsse`, and `Schusspositionen`. History alone does not
show what changes, where points go, or which patterns repeat. The user needs grounded statistics,
not guesses dressed as coaching.

## Outcome

A `Benutzer` can review personal progress across `Runden`, inspect one `Runde`, and understand every
figure's formula, sample, and limits. Personal data stays private. Cross-user comparisons stay off
until the privacy gate in [#51](https://github.com/TimerErTim/logandaim.at/issues/51) is approved.

## Roles

- `Benutzer`: reads statistics derived only from own `Runden` and `Schüsse`.
- `Systemadministrator`: no extra access; reads only own statistics.
- `Administrator` and `Mitarbeiter`: receive no statistics about a person's records.

## Scope

**In scope**

- **Tier 1 — proposed M1:** private overview at `/statistik`; per-round `Auswertung` tab at
  `/runden/detail`; the three history charts approved in [issue #12](https://github.com/TimerErTim/logandaim.at/issues/12); plus the existing-data figures
  below. Tier 1 delivery must include the three approved charts: score trend per `Parkour`,
  first-arrow hit rate over time, and weak `Ziele` by target group and distance. `Zielgruppe` is not
  stored in code; its slice is blocked. PO must approve added figures and changed M1 scope.
- **Tier 2 — later:** distance sweet spot/drop-off, recovery, closing, front/back, bounce-back, and
  pace-versus-score analyses below.
- **Tier 3 — blocked by data gaps:** metrics needing new stored facts or corrected master-data
  implementation.
- **Tier 4 — privacy-gated:** pooled first-arrow hit-rate comparison and percentile below.

Tier names are not delivery promises. [Issue #12](https://github.com/TimerErTim/logandaim.at/issues/12) approved three history charts for MVP. This
Draft expands that proposal; only the PO can approve added Tier 1 figures and changed MVP scope.

**Out of scope**

- Operator/facility statistics. No `Anlage` staff member may inspect a person's records
  ([`stammdaten.md`](stammdaten.md)). Anonymous aggregates need a separate product and privacy spec.
- Public profiles, friends, shared results, leaderboards, and identifiable comparisons.
- Medical, psychological, or causal claims. A score pattern does not prove fatigue, poor focus,
  course difficulty, or a training effect.
- Mid-round projected score. Target difficulty and remaining arrow outcomes vary; no validated
  prediction model exists. Show current totals, not a forecast.
- Weather, elevation, walking distance, equipment, and target posture. The data is not captured.

## User journeys

1. **Review progress.** A signed-in `Benutzer` opens `Statistik`, selects a date range or `Alle`, and
   sees personal totals, trends, hit patterns, distance/`Pflock` performance, and repeated `Ziel`
   results. Each figure states its sample and limits.
2. **Review one `Runde`.** From round detail, the user opens `Auswertung` and sees that round's
   figures plus up to three evidence-backed takeaways. A running round may show current figures,
   labelled `Laufend`; it shows no projected result in Tier 1.
3. **Inspect a pattern.** The user filters by date, `Parkour`, `Wertung`, distance band, `Pflock`, or
   `Ziel`, where applicable. Filters do not change stored records.
4. **Start with little history.** A new user sees known counts and what more completed rounds will
   unlock. No chart invents zeroes for missing evidence.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Auswertung` | Statistics for one `Runde`, separate from its authoritative score. |
| `Treffer` | `Schuss` in `Spot`, `Kill`, or `Body`; `Miss` is not a hit. |
| `Erfasste Ziel` | Route target with at least one stored `Schuss`. An unshot `Ziel` has no result, not zero. |
| `Auswertbare Runde` | Completed `Runde`. A running round supplies live values only where stated. |
| `Aufbau-Signatur` | Ordered stable physical target IDs plus effective `Pflock` colours and configured distances captured per target. Reorder, replacement, or effective stake change starts a new comparable series; renames alone do not. |
| `Pflock-Gruppe` | Shots in one `Parkour` with the same stored `Pflock` colour. Colour is arbitrary data, not a global stake name. |
| `Distanzband` | `[0,15)`, `[15,25)`, `[25,35)`, `[35,45)`, or `[45,∞)` metres, using stored whole-metre `Schussdistanz`. |
| `Rolling Mean` | Mean of the latest fixed number of eligible completed rounds, ordered by `(begonnen_am, RundeId)`. |
| `Konfidenzintervall` | Sampling-uncertainty range. It is not a cause, forecast, or guarantee. |

Rules:

1. Statistics are read-only folds over the user's `Runden`, captured `Rundenaufbau`, and `Schüsse`.
   Existing `Wertung` remains authoritative; statistics never alter scores.
2. Historical comparisons use completed rounds only. A running round may appear in its own
   `Auswertung`, labelled `Laufend`; it never enters PBs, rolling means, trend fits, seasonality, or
   cross-round baselines. Activity counts mean completed rounds unless stated otherwise.
3. Only the owning `Benutzer` may read personal figures. Another user's request returns 403; no
   authenticated user returns 401, matching protected round reads.
4. Missing records are not zero. Unshot targets are excluded from score and hit-rate denominators.
   A shot without position counts in scores and arrow totals, but not distance or stake breakdowns.
5. Only stored `Schussdistanz` feeds distance metrics. A stored `Pflock` never becomes a distance,
   even if the captured stake has configured distance ([`runde.md`](runde.md)).
6. Use the round snapshot, not live master data. Group physical targets by stable `AnlageZielId`;
   group course history by `ParkourId` and `Aufbau-Signatur`. A renamed `Parkour` does not split its
   series.
7. `Freie Runde` has no `Parkour` or physical `AnlageZielId`. Include it in personal volume and
   overall score summaries; exclude it from course, physical-target, and benchmark figures.
8. Show percentages with numerator and denominator. Do not round before aggregation. Counts and
   points remain exact integers.
9. Descriptive rates may start at `n = 1`; always show numerator and denominator. Show Wilson 95%
   intervals for binomial rates. Use `n ≥ 8` plus a 95% interval excluding zero for inferential
   trend/correlation claims only. A single-round `Auswertung` may show a raw rate, labelled as one
   round; make no trend or confidence claim.
10. Never turn missing evidence into `0%`, a blank plot, or a semantic warning colour.
11. `Erster Treffer` and `Mehrschuss` are alternate views of the same shots. Never add their totals.
12. Point figures use [`wertung.md`](wertung.md): for arrows 1/2/3, Spot = 20/14/8, Kill =
    18/12/6, Body = 16/10/4, Miss = 0. Arrow 4+ scores 0 in both modes. `Erster Treffer` uses the
    first non-Miss arrow's points; `Mehrschuss` sums every arrow. Statistics call the existing
    authoritative result; they do not substitute another scoring table.
13. **Intervals:** use two-sided 95% intervals where specified.
    - **Proportions:** Wilson score interval. For `p = x/n`, `z = 1.96`, centre is
      `(p + z²/(2n))/(1 + z²/n)`; half-width is
      `z × sqrt(p(1-p)/n + z²/(4n²))/(1 + z²/n)`. Clamp endpoints to `[0,1]`.
    - **Means:** Student-t interval `mean ± t(0.975,n−1) × s/√n`; `s` is sample standard deviation.
      Require `n ≥ 2`.
    - **Regression slope:** `slope ± t(0.975,n−2) × SE(slope)`. Require `n ≥ 8`; show the fitted
      slope only if interval excludes zero.
    - **Correlation:** Fisher transform `z = atanh(r)`, standard error `1/√(n−3)`, interval
      `z ± 1.96/√(n−3)`, then inverse-transform endpoints with `tanh`. Require `n ≥ 8` and interval
      excluding zero.
    - Keep full precision for calculation; round only for display. These intervals do not correct
      for dependence among shots or targets.

### Empty states

- No completed rounds: show `Noch keine abgeschlossenen Runden.` Keep only actual totals; do not
  draw empty axes as zero results.
- No shots: show `Noch keine Schüsse erfasst.` Rates are unavailable (`—`), never `0%`.
- Filter matches nothing: say `Keine Ergebnisse für diese Auswahl.` Offer a clear-filter action.
- Sample gate unmet: show observed and required counts; hide any rate or ranking that the gate forbids.
- No positioned shots: explain that distance/`Pflock` breakdown needs recorded positions. Never copy
  total accuracy into those groups.
- Setup changed: start a separate series and state that older rounds are not comparable.
- Benchmark gate unmet: show `Vergleich noch nicht verfügbar.` Never disclose pool size or how close
  it is to threshold.

## Statistics catalogue

Each item names inputs, calculation, meaning, and limits. `n` is the observation count. Rates show
numerator and denominator.

## Tier 1 — proposed M1, private and computable from current data

#### Activity and score summary

1. **Completed rounds**
   - **Input:** owned rounds in `Abgeschlossen` state with `abgeschlossen_am`.
   - **Calculation:** count each once by range, `Parkour`, and overall. Free rounds count overall.
   - **Meaning:** logged activity, not time spent shooting.
   - **Limit:** date filters use `begonnen_am`; the model has no editable shooting date. Deleted
     rounds no longer contribute.

2. **Logged shots and arrows per target**
   - **Input:** stored `Schüsse` in selected owned rounds.
   - **Calculation:** count shots; divide by count of `Erfasste Ziele` for mean shots per recorded
     target. Include `Pfeil` 4+ and missing-position shots.
   - **Meaning:** recorded arrow volume.
   - **Limit:** incomplete entry can understate arrows physically shot.

3. **Facilities visited**
   - **Input:** distinct `AnlageId`s among completed Parkour rounds in range.
   - **Calculation:** count unique IDs; free rounds add none.
   - **Meaning:** recorded facility reach, not travel distance or home course.

4. **Personal best (PB)**
   - **Input:** completed Parkour rounds with every snapshot target captured; group by `ParkourId`,
     `Aufbau-Signatur`, and `Wertung`.
   - **Calculation:** maximum round points in group. For cross-course context, show PPT beside raw
     score; never call it a course PB.
   - **Meaning:** highest recorded result for a complete comparable setup.
   - **Limit:** partial rounds cannot compete for PB. Raw totals do not compare routes or route
     versions. Free rounds have no stable route identity; exclude them from course PB and rolling
     series.

5. **Mean and median score**
   - **Input:** same complete rounds and grouping as PB.
   - **Calculation:** mean = sum of round totals / round count. Median = middle sorted total; with
     even `n`, average the two middle totals. Calculate separately per `Wertung`. Example: 100, 110,
     300 gives mean 170 and median 110.
   - **Meaning:** mean shows level but reacts to outliers; median shows the middle result and resists
     them.
   - **Limit:** show `n`; exclude partial rounds. For cross-course summaries use PPT, not raw totals.

6. **Rolling 5- and 10-round mean**
   - **Input:** latest 5 or 10 complete Parkour rounds in one `Parkour` + `Aufbau-Signatur` group.
   - **Calculation:** mean round total, newest included; compute each `Wertung` separately. First
     round starts at one; each completion shifts window by one. Show only at `n ≥ 5` or `n ≥ 10`.
   - **Meaning:** recent score level, less dominated by one round than PB.
   - **Limit:** setup change starts a new window. Before threshold, state how many more rounds are
     needed; show no partial rolling mean.

7. **Points per target (PPT)**
   - **Input:** points and targets with a `Wertung` in eligible round(s).
   - **Calculation:** total points / targets with at least one shot. Across rounds, sum points and
     targets first; do not average round means. Example: 20 points / 2 targets and 30 / 5 gives
     `50/7`, not mean of 10 and 6.
   - **Meaning:** points per entered target; helps compare route lengths.
   - **Limit:** does not correct for target difficulty or unshot targets. Show captured / route
     target count.

8. **Points per arrow (PPA)**
   - **Input:** all stored shots and points under the selected authoritative `Wertung`.
   - **Calculation:** sum awarded points / all stored arrows in those rounds. Include misses,
     `Pfeil` 4+, and arrows on targets with no hit as zero-point arrows. Aggregate points and arrows
     before division. Example: 20 points / 4 arrows = 5 PPA. Compute each `Wertung` separately.
   - **Meaning:** average awarded points per entered arrow.
   - **Limit:** incomplete logs understate the denominator. Show rate only with at least 20 arrows
     across 3 rounds.

#### Arrow and target results

9. **Hit-zone distribution**
   - **Input:** every stored shot in selected rounds: `Spot`, `Kill`, `Body`, `Miss`.
   - **Calculation:** count each zone / all stored shots; show `count / total` and percentage.
     Include `Pfeil` 4+, as it records a zone despite scoring zero.
   - **Meaning:** where entered arrows land.
   - **Limit:** multiple arrows at one target are separate observations. This is not target-kill rate.

10. **First-arrow hit rate**
    - **Input:** each captured target's `Pfeil` 1.
    - **Calculation:** first shots in `Spot`, `Kill`, or `Body` / targets with a recorded first shot.
      `Miss` is a miss. Example: 7/10 = 70%. Show Wilson 95% interval with every rate; one-round
      analysis labels it as one round.
    - **Meaning:** how often first recorded arrow hits any scoring zone.
    - **Limit:** unshot targets are excluded, not misses.

11. **First-arrow kill rate**
    - **Input:** same first shots as item 10.
    - **Calculation:** first shots in `Kill` / all recorded first shots. `Spot` is reported
      separately in the hit-zone split; `Body` counts as a hit for item 10, not a kill. Show numerator and denominator plus Wilson 95% interval.
    - **Meaning:** stricter first-arrow success than hit rate.
    - **Limit:** excludes second and later arrows.

12. **Miss rate**
    - **Input:** all stored shots, including extra arrows.
    - **Calculation:** `Miss` shots / all stored shots. Show `misses / shots` and Wilson 95% interval.
    - **Meaning:** share of entered arrows that missed.
    - **Limit:** not share of targets scoring zero. `0/0` is unavailable, not `0%`.

13. **Zero-target rate**
    - **Input:** captured targets with computed `Erster Treffer` result.
    - **Calculation:** targets with `Erster Treffer == 0` / all targets with a result. Show rate and
      Wilson 95% interval with numerator and denominator.
    - **Meaning:** share of recorded targets yielding no `Erster Treffer` points.
    - **Limit:** later arrows can hit although `Erster Treffer` is zero. Do not call this miss rate.
      `0/0` is unavailable.

14. **Points by arrow position**
    - **Input:** stored shots, `Pfeil`, `Wertungszone`, and scoring rules.
    - **Calculation:** show both scoring modes separately. For `Mehrschuss`, group each arrow's
      awarded points by `Pfeil`; divide points by arrows at that position. For `Erster Treffer`, show
      first-hit position distribution: targets whose first `Treffer` occurred at `Pfeil` 1, 2, 3,
      or 4+, divided by targets with any hit. Also show mean first-hit points at each hit position.
      Targets with no hit are reported separately. Show counts and denominators.
    - **Meaning:** `Mehrschuss` shows awarded points by arrow position; `Erster Treffer` shows which
      arrow first produced a scoring hit.
    - **Limit:** denominators differ. Archers may stop after a hit. `Pfeil` 4+ can hit but scores
      zero. Neither view isolates aim skill.

#### Distance and `Pflock`

15. **Accuracy by distance band**
    - **Input:** shots with stored `Schussdistanz` in whole metres.
    - **Calculation:** assign each shot to `[0,15)`, `[15,25)`, `[25,35)`, `[35,45)`, or `[45,∞)`.
      Show arrow hit rate (`Spot`/`Kill`/`Body` / all shots). Separately, first-arrow hit rate uses
      targets whose positioned `Pfeil` 1 is in that band: first-arrow hits / such targets. Count a
      target once. Show counts, denominators, and Wilson 95% intervals.
    - **Meaning:** recorded accuracy at entered distances.
    - **Limit:** exclude `Pflock` and missing positions. No stake-distance imputation, and no causal
      effect of distance implied.

16. **Accuracy by `Pflock`**
    - **Input:** Parkour shots whose stored position is `Pflock`; key by `ParkourId` and exact colour.
    - **Calculation:** per colour show shot count and arrow hit rate. First-arrow hit rate uses
      targets whose `Pfeil` 1 stored that colour / targets with a positioned first shot at that
      colour. Target counts once. Show arrow and first-arrow rates with Wilson 95% intervals and
      numerator/denominator.
    - **Meaning:** result for arrows recorded at that Parkour's stake colour.
    - **Limit:** same hex in another course is separate. Setup or colour changes affect comparability.
      Do not infer distance from configured stake data.

17. **Distance sweet spot and drop-off** *(Tier 2)*
    - **Input:** item 15's bands with at least 20 positioned first shots across 3 rounds per band.
    - **Calculation:** sweet spot = eligible band with highest first-arrow hit rate. Drop-off = first
      farther eligible band at least 15 percentage points below nearest eligible band, with
      non-overlapping Wilson 95% intervals. Compare each farther eligible band with nearest eligible
      band.
    - **Meaning:** descriptive distance pattern in entered shots.
    - **Limit:** no conclusion without eligible bands. Do not interpolate or call a band the user's
      physical limit.

#### Course and target history

18. **Score and first-arrow trends per `Parkour`**
    - **Input:** completed rounds with same `ParkourId` and `Aufbau-Signatur`. Score points use
      complete rounds and split by `Wertung`; first-arrow rate uses rounds with recorded first shots.
    - **Calculation:** order by `begonnen_am`, ties by `RundeId`. Plot one total per complete round.
      Separately plot first-arrow hits / first shots per round; require 5 first shots per round and
      show denominator. Show points from first eligible round. Fit score as outcome against
      sequential round index by ordinary least squares at `n ≥ 8`. Report slope in points per round
      and two-sided 95% t interval (`n−2` degrees of freedom). Do not fit an inferential line to
      first-arrow rates in Tier 1. For each proportion, also show its Wilson 95% interval.
    - **Meaning:** direction and rate of score change in comparable rounds; first-arrow series shows
      recorded hit-rate changes.
    - **Limit:** show fitted score slope only if interval excludes zero. Otherwise show points and
      `Trend nicht eindeutig`. Improvement does not prove training caused it.

19. **Weak and strong physical `Ziele`**
    - **Input:** snapshot `AnlageZielId`; completed Parkour rounds.
    - **Calculation:** per target, first-arrow hits / targets with a recorded first shot. Require 3
      first shots across 2 rounds. Compare with user's overall first-arrow rate in same date range;
      rank by percentage-point difference. Show numerator, denominator, rounds, difference. Example:
      target 1/3 = 33%, versus personal 60%, or −27 percentage points. Also show missed first shots
      by distance band from item 15; one target may appear in multiple bands.
    - **Meaning:** targets where this user's first-arrow result sits below or above own baseline.
    - **Limit:** never call a target objectively hard/easy. Do not group by animal name. Free-round
      targets have no persistent physical ID and are excluded. Do not rank below target gate.

20. **Pooled first-arrow hit rate and course-challenge proxy** *(Tier 4; privacy-gated)*
    - **Input:** first-arrow shots in completed Parkour rounds. Key by `ParkourId`,
      `Aufbau-Signatur`, and exact per-shot `Pflock` colour. Only first arrows with stored `Pflock`
      position enter. Same colour in another Parkour is separate; no distance-based stake inference.
    - **Calculation:** pooled hit rate = pooled first-arrow `Treffer` / pooled positioned first
      arrows. Challenge proxy = `1 − pooled hit rate`, shown as pooled miss percentage. Also compare
      current user's first-arrow rate with the pooled rate, in percentage points. Require at least
      20 completed rounds from at least 5 distinct users for the exact key. Never let many rounds
      from one user masquerade as many contributors; never expose per-user contributions.
    - **Meaning:** pooled observed hit/miss rate at this Parkour setup and stake colour. Higher miss
      rate suggests greater challenge for this cohort, not objectively harder course.
    - **Limit:** participant skill, selection, and sample composition affect rate. It is not an
      objective difficulty rating. Consent, retention, aggregation, anti-inference rules, and
      authorization are blocked on [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51). No implementation before privacy and PO approval.

21. **Personal percentile among pooled users** *(Tier 4; privacy-gated)*
    - **Input:** each eligible user's first-arrow hit rate for item 20's key and same cohort gate.
    - **Calculation:** `(eligible user rates below current + 0.5 × tied rates) / eligible user rates`,
      including current user once. Show whole percentile only after cohort gate. Do not rank arrows
      as if they were independent users.
    - **Meaning:** position among user-level rates for first arrows at this course setup and stake.
    - **Limit:** never show another user's rate, identity, contributor count, or pooled shot count.
      Disabled until [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51) and PO approve. Cohort floor alone is not privacy approval.

#### Pace, activity, and habits

22. **Round duration and minutes per target**
    - **Input:** completed round with `begonnen_am`, `abgeschlossen_am`, and at least one recorded
      target.
    - **Calculation:** duration = end − start. Minutes per target = duration minutes / recorded
      target count. Show duration, target count, quotient. Compare round durations only within the
      same `Parkour` + `Aufbau-Signatur`; compare free rounds only with the same free-round name.
      Cross-round pace figures require at least 5 rounds in that group.
    - **Meaning:** elapsed round pace averaged over recorded targets.
    - **Limit:** includes breaks; not active shooting time or measured per-target time. Free rounds use
      their frozen target count. Running rounds are excluded from historical average.

23. **Completion rate** *(data-blocked)*
    - **Input needed:** explicit terminal result for each started round: completed or abandoned, and
      terminal time.
    - **Calculation when captured:** completed / (completed + explicitly abandoned) in selected
      cohort. Show counts. Exclude still-open rounds; report Parkour/free rounds separately.
    - **Meaning:** share of known terminal outcomes that completed.
    - **Limit:** current model only has `InBearbeitung` and `Abgeschlossen`. An open round can be
      active, paused, or never resumed. No defensible completion rate exists; do not count open
      rounds as failures.

24. **Pace versus score** *(Tier 2)*
    - **Input:** completed rounds in one `Parkour` + `Aufbau-Signatur` + `Wertung`, positive duration,
      at least 3 recorded targets, and at least 8 rounds.
    - **Calculation:** Pearson `r` between minutes per recorded target and points per scored target.
      Free rounds form a separate exact-name group. Compute Fisher-z 95% interval; report `r`,
      interval, `n`, and observed pairs only when interval excludes zero.
    - **Meaning:** direction and strength of linear association in these rounds.
    - **Limit:** correlation does not show pace caused score change. Breaks and missing shots affect
      measures. Multiple targets within a round are correlated; interval treats each round as one
      observation.

25. **Weekly/monthly activity and streaks**
    - **Input:** completed rounds by local `de-AT` date of `begonnen_am`.
    - **Calculation:** counts per ISO week, month, weekday. Active-day streak = consecutive local
      calendar days with a completed round; active-week streak = consecutive ISO weeks with one.
      Show current and longest; a gap ends streak.
    - **Meaning:** regularity of logged activity, not training quality.
    - **Limit:** display timezone rule. Do not infer motivation or missed sessions.

26. **Seasonality** *(data-limited)*
    - **Input:** completed rounds by local month and season of `begonnen_am`.
    - **Calculation:** counts of rounds and shots by season. Compare mean points per scored target only
      for the same `Parkour` + `Aufbau-Signatur` + `Wertung`, with at least 5 rounds in each compared
      season. Seasons: winter Dec–Feb, spring Mar–May, summer Jun–Aug, autumn Sep–Nov.
    - **Meaning:** when the user records rounds and descriptive score differences by season.
    - **Limit:** start time stands in for shooting date. Do not combine different setup difficulty or
      claim weather/season caused a score change.

#### Three post-round takeaways

27. **Biggest relative gain**
    - **Input:** current round's distance-band or `Pflock` first-arrow rate; baseline from user's 10
      most recent earlier completed comparable rounds.
    - **Calculation:** require 3 first shots in current cell, 10 baseline first shots across at
      least 3 rounds. Difference = current rate − baseline rate. Show cell with largest positive
      percentage-point difference.
    - **Meaning:** strongest current result above recent personal baseline.
    - **Limit:** say `+N Prozentpunkte`, not `+N Punkte`. Descriptive, not a tested improvement.
      Omit when no cell qualifies.

28. **First-arrow misses**
    - **Input:** current round's recorded first arrows.
    - **Calculation:** count first arrows in `Miss`; show count / first shots. No counterfactual
      points.
    - **Meaning:** number of first-arrow misses to review.
    - **Limit:** call these misses, never lost points or preventable loss. Omit without first shot.

29. **Best first-arrow streak**
    - **Input:** route-order targets with recorded first shot.
    - **Calculation:** longest consecutive route positions whose first arrow is `Treffer`. An unshot
      target or first-arrow `Miss` breaks streak. Show length and first/last ordinal.
    - **Meaning:** longest uninterrupted run of first-arrow hits.
    - **Limit:** not probability or forecast. Omit if no first-arrow hit.

## Tier 2 — later, no new stored fields required

30. **Recovery after first-arrow miss**
    - **Input:** targets with `Pfeil` 1 = `Miss` and a recorded `Pfeil` 2.
    - **Calculation:** second arrows in `Spot`, `Kill`, or `Body` / eligible second arrows after first
      miss. Require 20 eligible second arrows across 3 completed rounds to show rate. A target with
      no second arrow is neither success nor failure; show that count separately.
    - **Meaning:** how often a recorded second arrow hits after a first-arrow miss.
    - **Limit:** does not prove sight correction, adjustment, or technique.

31. **Closing performance**
    - **Input:** last five route positions of a completed Parkour round with at least five targets.
    - **Calculation:** mean `Erster Treffer` points per scored target in last five positions minus
      round's mean per scored target. Show scored / 5.
    - **Meaning:** whether recorded closing targets scored above/below that round's mean.
    - **Limit:** route order is not elapsed time or fatigue. Missing result is not zero.

32. **Front half versus back half**
    - **Input:** captured route positions split at `ceil(total route targets / 2)`.
    - **Calculation:** each half's sum of `Erster Treffer` points / targets with result. Show both
      means, counts, and difference. Example: 90/10 = 9 early; 72/10 = 7.2 late; difference −1.8.
    - **Meaning:** score difference between early and later route sections.
    - **Limit:** not fatigue or concentration. Halves may have different target difficulty.

33. **Bounce-back after a zero target**
    - **Input:** target with `Erster Treffer == 0` and next route position with result.
    - **Calculation:** average first-hit points on following targets; compare with other scored
      targets in same rounds. Require 5 zero-to-next pairs across 3 rounds. Show means and pair count.
    - **Meaning:** recorded score after a zero target versus other targets in those rounds.
    - **Limit:** not tilt, mindset, or emotional recovery; adjacent targets may differ.

## Tier 3 — blocked by data gaps

34. **Target-group performance**
    - **Input:** first-arrow results joined to captured `Zielgruppe` 1–4.
    - **Calculation:** per group, first-arrow hits / recorded first shots; require 20 shots across 5
      rounds. Show group counts and rates.
    - **Meaning:** user's results by official target group.
    - **Limit:** `stammdaten.md` requires groups 1–4, but Rust, SQL, and API do not store/expose it.
      Add and snapshot that field first. Never infer group from target name or dimensions.

35. **Per-target pace**
    - **Input:** timestamp entering/leaving each target, or equivalent trusted duration record.
    - **Calculation:** elapsed target time; summarize by physical target/group with at least 10
      observations across 3 rounds.
    - **Meaning:** where time is spent on course.
    - **Limit:** round start/end cannot recover this. Do not divide round duration evenly and call it
      measured target pace.

36. **Distance-estimation accuracy**
    - **Input:** independently captured estimated and reference distance for same shot.
    - **Calculation:** signed error = estimate − reference; absolute error = absolute signed error.
      Show mean signed error (bias direction) and median absolute error (typical size).
    - **Meaning:** whether estimates trend high/low and by how much.
    - **Limit:** current `Schussdistanz` stores one value and cannot distinguish typed, interpolated,
      or estimated. Stake distance is not an independent reference.

37. **Facility map by Bundesland**
    - **Input:** structured `Anlage` address with Austrian Bundesland field.
    - **Calculation:** count distinct visited `Anlagen` by Bundesland.
    - **Meaning:** geographic coverage of recorded visits.
    - **Limit:** current address model lacks Bundesland. Do not guess from locality or postal code.

## Tier 4 — only after privacy approval

38. **Pooled benchmark cohort**
    - **Key:** `ParkourId`, `Aufbau-Signatur`, `Wertung`, and exact `Pflock` colour stored per first
      `Schuss`. Keep a colour separate per Parkour. Exclude free, incomplete, mixed-colour, or
      missing-position rounds from the relevant key.
    - **Gate:** at least 20 eligible completed rounds from at least 5 distinct `Benutzer` for that
      exact key. Current user's shots may enter pooled aggregate; include current user's rate once in
      percentile rank.
    - **Protection:** return only approved comparison and percentile. Never return other identities,
      individual rates, contributor count, or pooled shot count. Apply floor after every filter; add
      rate limits and privacy review under [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51).
    - **Meaning:** comparison across a sufficiently broad cohort, not universal ranking.
    - **Blocker:** charter excludes global benchmarks. [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51) and PO approval must change that before
      implementation.

## Requirements

**Functional**

- FR-1: Provide private `/statistik` overview and per-round `Auswertung` at `/runden/detail`.
- FR-2: Offer date, `Parkour`, `Wertung`, distance-band, `Pflock`, and physical-`Ziel` filters only
  where applicable; show active filters and allow clearing them.
- FR-3: Compute Tier 1 figures by defined formulas; show numerator, denominator, unit, and sample
  where applicable.
- FR-4: Keep `Erster Treffer` and `Mehrschuss` separate.
- FR-5: Historical calculations use completed rounds; label live round figures `Laufend`.
- FR-6: Show up to three post-round takeaways only when their own evidence gates pass. Omit the rest;
  never fill slots with generic advice.
- FR-7: Use `begonnen_am` for date filters, activity, and seasons until separate shooting date exists.
  State this limit in help.
- FR-8: Keep Tier 2, Tier 3, and Tier 4 out of M1 UI.
- FR-9: Enforce ownership on every API read; hiding a UI control is not authorization.

**Non-functional**

- NFR-1: Every figure has text value, unit, sample, and denominator where relevant. Charts add a view;
  they are not the only explanation.
- NFR-2: Use German UI copy and `de-AT` dates, numbers, and units. Round only for display.
- NFR-3: Statistics are read-only and must not delay capture. Load after round detail if needed.
- NFR-4: Meet WCAG 2.2 AA: keyboard operation, visible focus, labelled controls, text alternatives,
  and no meaning by colour alone.
- NFR-5: No M1 response contains another user's data. Tier 4 stays disabled until [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51) and PO approval.
- NFR-6: Regression and correlation never claim cause, diagnose technique, or predict future score.

## Acceptance criteria

- **AC-1 — Ownership:** Own request returns only own rounds/shots. Another user's data returns 403;
  unauthenticated request returns 401.
- **AC-2 — Both scoring modes:** Show separate `Erster Treffer` and `Mehrschuss` totals and rates;
  never combine them.
- **AC-3 — Missing targets:** Exclude unshot targets from score/rate denominators; show captured /
  route targets. Never count unshot target as zero.
- **AC-4 — Hit zones:** Count every stored zone once, including `Pfeil` 4+, and divide by all stored
  arrows.
- **AC-5 — First-arrow rate:** First `Miss`, second `Kill` counts as first-arrow miss and recovery hit;
  not first-arrow hit. Exclude unshot targets.
- **AC-6 — Zero-target rate:** Target with shots and `Erster Treffer == 0` counts once; target with no
  shot is excluded.
- **AC-7 — Distance provenance:** Meter distance enters one band. `Pflock` and missing positions do
  not. Never impute stake distance.
- **AC-8 — Band boundaries:** 14, 15, 24, 25, 34, 35, 44, 45 m map to `[0,15)`, `[15,25)`,
  `[15,25)`, `[25,35)`, `[25,35)`, `[35,45)`, `[35,45)`, `[45,∞)`.
- **AC-9 — `Pflock` scope:** Same colour in different Parkours stays separate. Missing-position shots
  enter neither group.
- **AC-9a — No projection:** A running round shows current recorded totals only; it shows no projected
  mid-round score.
- **AC-10 — PPT:** 20 points / 2 targets and 30 / 5 aggregate to `50/7`, not mean of 10 and 6.
- **AC-10a — PPA:** 20 points / 4 arrows gives 5 in that `Wertung`; include zero-point arrows and
  compute each scoring mode separately.
- **AC-11 — PB/windows:** Partial rounds do not compete for PB or rolling mean. Withhold rolling mean
  until 5/10 eligible rounds. Setup change starts new series.
- **AC-11a — Values:** For arrows 1/2/3, verify Spot 20/14/8, Kill 18/12/6, Body 16/10/4, Miss 0;
  Pfeil 4+ scores 0 in both modes. `Erster Treffer` selects first non-Miss; `Mehrschuss` sums all.
- **AC-12 — Trend gate:** Under 8 comparable complete rounds, show points only. At `n ≥ 8`, show
  score slope only if two-sided 95% interval excludes zero; else say trend is unclear.
- **AC-13 — Target gate:** Below 3 first shots or 2 rounds for physical target, do not rank. Group by
  snapshot identity, not animal name.
- **AC-14 — Course/free:** Include free rounds in personal overall activity/score. Exclude from
  course and physical-target trends. Split series when setup changes.
- **AC-15 — Duration:** Use start/end timestamps only. Label minutes per target as elapsed duration /
  recorded targets, not active or measured per-target time.
- **AC-16 — Inference:** No pace correlation below 8 comparable rounds or when Fisher-z 95% interval
  includes zero. Never phrase it as cause.
- **AC-17 — Takeaways:** Gain needs current and baseline gates; misses show no hypothetical lost
  points; streak breaks at unshot target or first-arrow miss.
- **AC-18 — Activity/seasons:** Group by local `de-AT` date of `begonnen_am`; show season definitions.
  Do not claim activity means cause.
- **AC-19 — Privacy:** No M1 response contains another user's data. Tier 4 returns no identities or
  cohort count and stays unavailable until [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51) and PO approve.
- **AC-20 — Accessible charts:** Each chart has labelled text and table/list equivalent. Keyboard
  and screen-reader users can read all figures without chart.
- **AC-21 — Blocked dimensions:** Do not render target-group, per-target pace, distance-error,
  Bundesland, completion-rate, or operator figures until prerequisites and approval exist.

## Risks and open questions

- [ ] Q-1: Does PO approve cross-user pooled statistics, consent, and 20-round/5-user floor via [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51)?
      Until resolved, Tier 4 stays disabled.
- [ ] Q-2: Should user-entered shooting date replace `begonnen_am`? Current model has no separate date;
      M1 uses start timestamp.
- [ ] Q-3: Resolve `Zielgruppe` mismatch: `stammdaten.md` requires groups 1–4, but code does not
      persist/expose it. Target-group statistics stay blocked.
- [ ] Q-4: Should free-round series be user-selectable? M1 excludes them from comparable course PBs.
- [ ] Q-5: Which chart form works on phone and desktop? Formulas/denominators stay fixed; follow
      design system and verify `/dev`.

## Verification

- Domain tests cover formula/denominator for AC-2–AC-18: empty inputs, partial rounds, setup changes,
  missing positions, band boundaries, and sample gates. Test PPA and arrow-position calculations for
  both scoring modes.
- API tests cover ownership (AC-1, AC-19) and prove aggregate responses contain no foreign rows.
- Frontend tests/manual checks cover filters, empty states, German formatting, text alternatives
  (AC-20), phone and wide layouts.
- Tier 4 tests use synthetic fixtures only, with cohorts below, at, and above gate.
- AC-21 tests assert blocked metrics stay absent until prerequisites and approval exist.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-11-16 | Draft scope: private statistics proposed for M1; Tier 2/3 deferred; pooled comparison gated on [issue #51](https://github.com/TimerErTim/logandaim.at/issues/51). | TimerErTim (discussion) |
| 2026-11-16 | Overview at `/statistik`; per-round `Auswertung` tab; Tier 1 derives from existing stored fields. | TimerErTim (discussion) |
| 2026-11-16 | Trend/correlation inference requires `n ≥ 8` and 95% interval excluding zero. | TimerErTim (discussion) |
| 2026-11-16 | Benchmark floor: 20 completed rounds from at least 5 distinct users. | TimerErTim (discussion) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-11-16 | Initial detailed Draft from issue #12 and statistics design discussion. | Agent (Cursor) |
