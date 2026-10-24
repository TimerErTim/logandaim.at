---
id: SPEC-visual-identity
title: Visual identity and design system
status: Draft
owner: TimerErTim
issues: [7]
superseded_by: null
---

# Visual identity and design system

> **Status note.** The mechanics in this spec (tokens, primitives, gallery, gates) are implemented
> under issue #7 and reviewed by looking at `/dev`. They stay in `Draft` because the identity they
> support is not decided yet: the seed palette is inherited, and every visual decision below the
> mechanics is still a PO call.

## Problem

logandaim.at has no identity of its own yet. The frontend carries the neutral seed it was built
with: zinc surfaces and a green primary, hand-written as raw Tailwind palette classes in every
component. Nobody has decided what the product should look like, and there is no agreed way to
review a proposal, so any change risks contradicting the previous one.

## Outcome

The product has one described visual identity that a person can look at and approve, implemented as
a small set of semantic tokens and shadcn-style components. A reviewer can see every component in
every state, in light and dark, on one screen while the backend is down. A change to the palette or
to a component convention is a single edit that the app, the agent-facing rule, and this spec all
reflect.

## Roles

| Role | Use |
|------|-----|
| Product Owner | Decides the identity by looking at the gallery; approves this spec. |
| Schütze (archer) | Uses the result outdoors, often one-handed and with gloves. The accessibility bar is written for this role. |
| Agent implementer | Reads the design rule and builds UI against the tokens, never against raw colours. |

## Scope

**In scope**

- Semantic design tokens for colour, radius, and font in `frontend/src/app/globals.css`, defined
  for light and dark.
- shadcn-style UI primitives in `frontend/src/components/ui/`, with variants defined through
  `cva` and public component APIs kept compatible with the existing call sites.
- A dev-only component gallery at `/dev` with light and dark side by side, live contrast checks,
  and a touch-target overlay.
- An agent-facing design rule (`.cursor/rules/design-system.mdc`) and the sync obligation between
  the rule, this spec, and the token file.
- A `check:design` gate in `mise run check` that fails on an undocumented token or a raw palette
  class.

**Out of scope**

- The logo, favicon, app icons, and README banner. Those are part of issue #7's output but follow
  this spec's tokens; they are tracked in the same issue.
- The brand name and product copy. Covered by the rebrand issue #6.
- A user-facing theme switcher. Identity is one default; the class-based dark variant exists for
  review and for the system preference, not as a user setting.
- A second locale. Copy rules live in the charter's language bar.

## User journeys

1. The PO opens `/dev` on a dev server and scrolls the gallery. Each component section shows the
   light frame next to the dark frame, so a palette is judged in both modes at once.
2. The PO looks at the Tokens section: every colour is a swatch and every documented pair shows its
   contrast ratio with an AA pass or fail. A token below 4.5:1 is visibly a problem, not an opinion.
3. The implementer changes a token value or a component variant, reloads, and sees the result. If
   it is kept, they update the design rule and the decision log below in the same commit.
4. An archer logs a `Schuss` on a phone in daylight: targets are at least 44 by 44 pixels, focus is
   visible for keyboard users, and the `Wertungszone` is readable as text, not only as colour.

## Domain terms and business rules

No new domain terms. The UI uses the charter glossary (`Anlage`, `Parcours`, `Ziel`, `Pflock`,
`Runde`, `Schuss`, `Wertung`, `Wertungszone`).

Rules:

1. **Tokens are the only colour source.** Components use semantic utilities
   (`bg-surface`, `text-muted-foreground`); raw palette classes and hex values do not appear outside
   `globals.css`.
2. **Contrast is a gate.** Documented text/surface pairs meet WCAG 2.2 AA at 4.5:1.
3. **Colour is never the only carrier of meaning.** Status, `Wertungszone`, and validation always
   include text or an icon.
4. **Touch targets are at least 44 by 44 pixels** on logging controls.
5. **Agents do not invent identity.** The seed palette is inherited; every change is proposed by
   editing tokens and accepted by the PO in the gallery.
6. **One primary action per screen.** Destructive actions are never the primary button.

## Requirements

**Functional**

- FR-1: The app renders every page in light and dark, following the system preference by default.
- FR-2: The gallery at `/dev` renders every component, variant, size, and state, in a light and a
  dark frame, without a backend.
- FR-3: The gallery measures the computed contrast ratio of every documented token pair and shows
  an AA pass or fail.
- FR-4: The gallery can outline interactive elements to expose touch targets under 44 by 44 pixels.
- FR-5: The gallery exists only in the development phase; the production export contains no `/dev`
  route and none of its code.

**Non-functional**

- NFR-1: `mise run check` fails when a token in `globals.css` is not named in the design rule, or
  when a raw palette class appears under `frontend/src`.
- NFR-2: `mise run build:frontend` fails when `out/dev` exists or gallery code is present in the
  export.
- NFR-3: Existing component call sites keep compiling; the public variant names of `Button`,
  `Alert`, and `Badge` do not change for existing callers.

## Acceptance criteria

- AC-1: The dev server serves `/dev` with every component section in light and dark frames.
- AC-2: The production build contains no `/dev` route, and `build:frontend` asserts it.
- AC-3: Every documented token pair shows a contrast ratio and an AA verdict in the gallery.
- AC-4: `grep` for raw palette classes under `frontend/src` matches only `globals.css`.
- AC-5: Every token in `globals.css` is named in `.cursor/rules/design-system.mdc`.
- AC-6: `/` and `/schuss` render with the same structure as before, using semantic tokens.
- AC-7: The gallery works with the backend unreachable.

## Open questions

- [ ] Q-1: Design direction and palette. Answered by looking at the gallery; recorded in the
  decision log below.
- [x] Q-2: Typeface. Answered: Nunito Sans (see the decision log).

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-24 | Seed palette stays the inherited zinc/green pair; no identity invented. | TimerErTim |
| 2026-10-24 | Light-mode primary darkened to `#047857` and warning to `#b45309` after the gallery showed white-on-emerald-600 at 3.77:1 and amber-600 at 3.19:1, both below AA. | Agent (Cursor) |
| 2026-10-24 | **Direction set.** Functional UI, no presentation UI: clean, low saturation, natural. Should appeal to an archer, so clear boundaries and natural colours rather than glass, elevation, or material-style tonal surfaces. Modern and conventional in behaviour, but nothing that reinvents UI design. Little rounding, not a lot. | TimerErTim |
| 2026-10-24 | **Palette replaced** to follow the direction: warm paper light, warm brown-black dark (the previous near-black was too dark and sterile), grass-green primary. All documented pairs pass AA; design rules gained flat-by-default, functional rounding, and natural-colour entries. | Agent (Cursor), reviewing with TimerErTim |
| 2026-10-24 | **Slider knob reduced** from 44px to 20px on a 6px track, keeping the 44px hit area via a padded `::before`. The touch minimum applies to the target, not the visible control. Gallery trigger buttons moved into a flex row after they rendered flush. | TimerErTim |
| 2026-10-24 | **Icon button height fixed.** `size="icon"` drew a 44px box next to 40px `md` buttons, so mixed rows were ragged. It now draws at `md` height and takes its 44px target from a `::before` overlay, the same split between drawn size and hit area as the slider knob. | Agent (Cursor) |
| 2026-10-24 | **Dark palette cooled by one step.** Same warm brown-black family, less orange cast (`--background` `#1a1813` → `#1a1917`). Light mode unchanged. All documented pairs still pass AA. | Agent (Cursor), reviewing with TimerErTim |
| 2026-10-24 | **Typeface decided: Nunito Sans**, chosen by the PO from the gallery comparison against Geist, IBM Plex Sans and Source Sans 3. Warmer and friendlier without reading as a consumer app; weights 400–700 loaded. Geist is no longer shipped as the UI face; Geist Mono stays for machine values. | TimerErTim |
| 2026-10-24 | **Geist now actually renders.** The `next/font` variables were declared on `<body>`, where the root-level `--font-sans` mapping cannot see them, so the app silently fell back to `system-ui`. They moved to `<html>`. | Agent (Cursor) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-24 | Draft from issue #7 and the design-system work. | Agent (Cursor) |
