---
id: SPEC-visual-identity
title: Visual identity and design system
status: Draft
owner: TimerErTim
issues: [7]
superseded_by: null
---

# Visual identity and design system

> **Status.** Mechanics (tokens, primitives, gallery, gates) implemented under issue #7 and reviewed
> by looking at `/dev`. Identity decided: **Bogenschuss Logbuch (Sleek Paperstyle)** (PO,
> 2026-10-25); it supersedes Jagdkarte & Kompass. Stays `Draft` until the PO formally approves.

## Problem

The frontend carries the neutral seed it was built with: zinc surfaces, green primary, hand-written
raw Tailwind palette classes in every component. No identity is decided, and there is no agreed way
to review a proposal, so each change risks contradicting the last.

## Outcome

One described visual identity a person can look at and approve, implemented as a small set of
semantic tokens and shadcn-style components. A reviewer sees every component in every state, light
and dark, on one screen with the backend down. A palette or convention change is one edit that the
app, the agent-facing rule, and this spec all reflect.

## Roles

| Role | Use |
|------|-----|
| Product Owner | Decides the identity by looking at the gallery; approves this spec. |
| Schütze (archer) | Uses the result outdoors, often one-handed and gloved. The accessibility bar is written for this role. |
| Agent implementer | Reads the design rule, builds UI against tokens, never against raw colours. |

## Scope

**In scope**

- Semantic tokens for colour, radius, font in `frontend/src/app/globals.css`, defined for light and
  dark.
- shadcn-style primitives in `frontend/src/components/ui/`, variants via `cva`, public APIs
  compatible with existing call sites.
- Dev-only gallery at `/dev`: light and dark side by side, live contrast checks, touch-target
  overlay.
- Agent-facing design rule (`.cursor/rules/design-system.mdc`) and the sync obligation between rule,
  spec, and token file.
- `check:design` gate in `mise run check`.

**Out of scope**

- Logo, favicon, app icons, README banner — issue #7's output but follow this spec's tokens.
- Brand name and product copy — rebrand issue #6.
- User-facing theme switcher. Identity is one default; the class-based dark variant exists for
  review and system preference, not as a setting.
- A second locale. Copy rules live in the charter's language bar.

## User journeys

1. PO opens `/dev`, scrolls the gallery. Each section shows light next to dark, so a palette is
   judged in both modes at once.
2. PO reads the Tokens section: every colour a swatch, every documented pair showing its contrast
   ratio with an AA verdict. A token below 4.5:1 is visibly a problem, not an opinion.
3. Implementer changes a token or variant, reloads, sees the result. If kept, they update the design
   rule and this decision log in the same commit.
4. An archer logs a `Schuss` on a phone in daylight: targets ≥ 44×44 px, focus visible for keyboard
   users, `Wertungszone` readable as text, not only colour.

## Domain terms and business rules

No new domain terms. UI uses the charter glossary (`Anlage`, `Parcours`, `Ziel`, `Pflock`, `Runde`,
`Schuss`, `Wertung`, `Wertungszone`).

Rules:

1. **Tokens are the only colour source.** Semantic utilities only (`bg-surface`, `text-secondary`);
   no raw palette class or hex outside `globals.css`. One name per colour, no aliases.
2. **Contrast is a gate.** Documented text/surface pairs meet WCAG 2.2 AA (4.5:1). Four prescribed
   values fail as text and are role-restricted instead (see the design rule).
3. **Colour never the only carrier.** Status, `Wertungszone`, validation always include text or icon.
4. **Touch targets ≥ 44×44 px** on logging controls.
5. **Agents do not invent identity.** Changes are proposed by editing tokens and accepted by the PO
   in the gallery.
6. **One primary action per screen.** Destructive actions are never the primary button.

## Requirements

**Functional**

- FR-1: Every page renders light and dark, following system preference by default.
- FR-2: `/dev` renders every component, variant, size, state in light and dark frames, no backend.
- FR-3: Gallery measures computed contrast of every documented pair, shows AA pass/fail.
- FR-4: Gallery can outline interactive elements exposing targets under 44×44 px.
- FR-5: Gallery exists only in the development phase; production export has no `/dev` route or code.

**Non-functional**

- NFR-1: `mise run check` fails on an undocumented token, a raw palette class under `frontend/src`,
  or a retired utility name.
- NFR-2: Existing call sites keep compiling; public variant names of `Button`, `Alert`, `Badge` do
  not change for existing callers.

## Acceptance criteria

- AC-1: Dev server serves `/dev` with every component section in light and dark frames.
- AC-2: Production build contains no `/dev` route.
- AC-3: Every documented token pair shows a contrast ratio and AA verdict in the gallery.
- AC-4: `grep` for raw palette classes under `frontend/src` matches only `globals.css`.
- AC-5: Every token in `globals.css` is named in `.cursor/rules/design-system.mdc`.
- AC-6: `/` and `/schuss` render with the same structure as before, using semantic tokens.
- AC-7: Gallery works with the backend unreachable.

## Open questions

- [x] Q-1: Design direction and palette. Answered: **Bogenschuss Logbuch (Sleek Paperstyle)** —
  hunting logbook on warm paper, one hunter green as the only brand colour.
- [x] Q-2: Typeface. Answered: Lora for titles, Inter for the interface, Geist Mono for machine
  values.

## Decision log

History, newest last. Superseded decisions stay for traceability.

| Date | Decision |
|------|----------|
| 2026-10-24 | Seed palette inherited (zinc/green); no identity invented. |
| 2026-10-24 | Two inherited contrast failures fixed: white on `emerald-600` 3.77:1, `amber-600` text 3.19:1. Same hues, AA-passing shades. |
| 2026-10-24 | **Direction set.** Functional, not presentation UI: clean, low saturation, natural, clear boundaries. Modern behaviour, nothing reinventing UI. Little rounding. |
| 2026-10-24 | **Palette replaced** to follow: warm paper light, warm brown-black dark, grass-green primary. Flat-by-default and functional rounding added to the rules. |
| 2026-10-24 | **Icon button height fixed.** `size="icon"` drew 44px beside 40px `md` buttons, making rows ragged; now draws at `md` height with a `before` overlay for the 44px target. |
| 2026-10-24 | **Geist actually renders.** `next/font` variables were on `<body>` where the root-level `--font-sans` mapping cannot see them, so it silently fell back to `system-ui`. Moved to `<html>`. |
| 2026-10-24 | **Typeface: Nunito Sans** from the gallery comparison (later superseded by Inter). Geist Mono stays for machine values. |
| 2026-10-24 | **Pflock colours are master data, not design tokens.** The checkpoint slider takes band colours as a prop; no Pflock palette in `globals.css`. |
| 2026-10-24 | **Identity proposal "Feldbuch & Wegmarkierung"** prototyped in the gallery (draft). Superseded. |
| 2026-10-24 | **Ten-variant line-up** prototyped so the PO could compare philosophies like for like. Superseded. |
| 2026-10-24 | **Jagdkarte & Kompass deepened** into day and night map sheets. Superseded. |
| 2026-10-25 | **Identity chosen: Jagdkarte & Kompass** — map-and-compass language throughout, map paper light, night-map dark. Superseded 2026-10-25. |
| 2026-10-25 | **Scope marker: visual identity only, no user flows.** The PO revoked work that drifted into journey design. Whether a parcours shows as a map, progress bar, or step list is a product decision, explicitly *not* settled by this identity. |
| 2026-10-25 | **Identity replaced: Bogenschuss Logbuch (Sleek Paperstyle).** PO supplied a written guide. Warm ivory paper, one hunter green (`#1B3B2B`) as the only brand colour, warm ink, hairline pencil rules. Light is paper on a desk; dark is the same book by lamplight — warm charcoal, never pure black, green lightened to sage so it keeps contrast. |
| 2026-10-25 | **Two typefaces, two jobs.** Lora for titles only, so the logbook character lives in headings. Inter for the interface. Geist Mono for machine values. |
| 2026-10-25 | **Guide violations fixed.** Sticky header had `backdrop-blur` (glassmorphism) → opaque `bg-surface`. Dragging row had `shadow-lg` → `ring-1 ring-primary`. |
| 2026-10-25 | **Canonical palette adopted verbatim.** Token names and every hex value match the PO's guide exactly: `--bg-*`, `--border-*`, `--text-*`, `--color-primary*`, `--semantic-*`. `--bg-surface-muted` (`#F0ECE3`) is an addition not in the guide: a second sheet one step off the desk, since the guide's muted ink measures 4.47:1 on the first candidate and would fail AA. |
| 2026-10-25 | **Supersedes "amber darkened": the prescribed amber is restored and role-restricted.** `#D97706`, `#9E9A93`, `#CFCAC0` measure 3.08:1, 2.48:1, 1.58:1 on paper and cannot carry text. Rather than substituting a darker hue, the rulebook restricts each to the role where it is legible: muted ink = placeholders/disabled; amber = rules, washes, icons, borders (a warning that must be *read* is ink on the amber wash); strong contour = 1px field boundary. `--semantic-success` is limited to text on `--bg-surface` (4.85:1). |
| 2026-10-25 | **Inputs sit on a raised sheet.** Inputs, textareas, selects, search fields, dialogs, dropdowns → `--bg-elevated` with `--border-strong`. Cards, tables, header stay on `--bg-surface` with `--border-subtle`. `--color-primary-hover` replaced `bg-primary/90`. |
| 2026-10-25 | **The gallery is domain-free.** All shooting vocabulary removed from `/dev`, replaced by neutral German UI fixtures (`Eintrag`, `Vorgang`, `Zeile`, `Status`, `Kategorie`, `Alpha`/`Beta`). Every gallery string German, identifiers and comments English. Checkpoint band colours moved from raw hex to `var(--…)`. |
| 2026-10-25 | **One name per colour; alias layer removed.** `@theme inline` had kept the shadcn names alive alongside canonical tokens, so the app had two names for every colour and they contradicted each other — `--text-primary` is ink, but the utility `text-primary` was the brand green. Utilities now mirror tokens one to one; the collision resolves in favour of the brand, so ink is **`text-ink`**. 34 files migrated; `check:design` rejects the retired names. |

## Revision history

| Date | Change |
|------|--------|
| 2026-10-24 | Draft from issue #7 and the design-system work. |
| 2026-10-25 | Identity decided, then replaced by Bogenschuss Logbuch; tokens, components, rule, and gallery follow. |
| 2026-10-25 | Canonical palette adopted; contrast handled by role restriction instead of hue substitution; gallery made domain-free; palette unified to one name per colour. |
