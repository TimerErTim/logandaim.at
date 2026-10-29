---
id: SPEC-visual-identity
title: Visual identity and design system
status: Draft
owner: TimerErTim
issues: [7]
superseded_by: null
---

# Visual identity and design system

> **Status.** Identity decided: **Bogenschuss Logbuch (Sleek Paperstyle)** (PO, 2026-10-25). The
> mechanics (tokens, primitives, gallery, gates) are implemented under issue #7. Stays `Draft` until
> the PO formally approves.
>
> **This spec holds intent and history.** The rules an implementer follows live in
> [`.cursor/rules/design-system.mdc`](../../.cursor/rules/design-system.mdc); token values live in
> `frontend/src/app/globals.css`. Neither is restated here.

## Problem

The frontend carried the neutral seed it was built with: zinc surfaces, green primary, raw Tailwind
palette classes in every component, and nowhere to review a proposal. No identity was decided, so
every change risked contradicting the last, and agents had no way to build UI that looked intended.

## Outcome

One described identity a person can look at and approve, implemented as a small set of semantic
tokens and shadcn-style primitives. A reviewer sees every component in every state, light and dark,
on one screen — including one fully composed page — with the backend down. A palette or convention
change is one edit that the app, the agent rule, and this spec all reflect.

## Roles

| Role | Use |
|------|-----|
| Product Owner | Decides the identity by looking at the gallery; approves this spec. |
| Benutzer (archer) | Uses the result outdoors, often one-handed and gloved. The accessibility bar is written for this role. |
| Agent implementer | Reads the design rule, builds UI against tokens, never against raw colours. |

## Scope

**In scope**

- Semantic tokens for colour, radius, shadow, and font in `frontend/src/app/globals.css`, defined for
  light and dark.
- shadcn-style primitives in `frontend/src/components/ui/`, variants via `cva`, public APIs
  compatible with existing call sites.
- Dev-only gallery at `/dev`: light and dark side by side, live contrast checks, touch-target
  overlay, one composed page.
- The agent-facing design rule and the sync obligation between rule, spec, and token file.
- `check:design` gate in `mise run check`.

**Out of scope**

- Logo, favicon, app icons, README banner — issue #7's other output; they follow this spec's tokens.
- Brand name and product copy — rebrand issue #6.
- User-facing theme switcher. Identity is one default; the class-based dark variant exists for review
  and system preference, not as a setting.
- A second locale. Copy rules live in the charter's language bar.
- User journeys. Whether a parcours shows as a map, a progress bar, or a step list is a product
  decision, not settled by this identity.

## User journeys

1. PO opens `/dev` and scrolls: every section shows light next to dark, so a palette is judged in
   both modes at once, and plate 16 shows one whole page assembled.
2. PO reads the Tokens section: every colour a swatch, every documented pair with its contrast ratio
   and AA verdict. A token below 4.5:1 is visibly a problem, not an opinion.
3. Implementer changes a token or variant, reloads, sees the result. If kept, they update the design
   rule and this decision log in the same commit.
4. An archer logs a `Schuss` on a phone in daylight: targets ≥ 44×44 px, focus visible for keyboard
   users, `Wertungszone` readable as text, not only colour.

## Requirements

**Functional**

- FR-1: Every page renders light and dark, following system preference by default.
- FR-2: `/dev` renders every component, variant, size, and state in light and dark frames, no backend.
- FR-3: The gallery measures the computed contrast of every documented pair and shows an AA verdict.
- FR-4: The gallery can outline interactive elements whose target is under 44×44 px.
- FR-5: The gallery exists only in the development phase; the production export has no `/dev` route.

**Non-functional**

- NFR-1: `mise run check` fails on an undocumented token, a raw palette class under `frontend/src`, or
  a retired utility name.
- NFR-2: Existing call sites keep compiling; public variant names of `Button`, `Alert`, and `Badge` do
  not change for existing callers.

## Acceptance criteria

- AC-1: Dev server serves `/dev` with every component section in light and dark frames.
- AC-2: Production build contains no `/dev` route.
- AC-3: Every documented token pair shows a contrast ratio and AA verdict in the gallery.
- AC-4: `grep` for raw palette classes under `frontend/src` matches only `globals.css`.
- AC-5: Every token in `globals.css` is named in `.cursor/rules/design-system.mdc`.
- AC-6: `/` and `/schuss` render with the same structure as before, using semantic tokens.
- AC-7: The gallery works with the backend unreachable.

## Rules

Business rules and the full visual rulebook are in
[`.cursor/rules/design-system.mdc`](../../.cursor/rules/design-system.mdc). The product-level
constraints this spec adds:

1. **Agents do not invent identity.** A change is proposed by editing tokens and judged by the PO in
   the gallery; a superseded direction is kept in the decision log, not on the page.
2. **Colour never the only carrier.** Status, `Wertungszone`, and validation always include a word or
   an icon.
3. **Destructive actions are never the primary button**, and a screen has at most one primary action.
4. **Accessibility is a gate, not a preference.** Contrast, the 44×44 px target minimum, and visible
   focus are measured — in the gallery by hit-testing, in CI by `check:design` — not asserted.

## Open questions

- [x] Q-1: Design direction and palette. Answered: **Bogenschuss Logbuch (Sleek Paperstyle)** —
  hunting logbook on warm paper, one hunter green as the only brand colour.
- [x] Q-2: Typeface. Answered: Lora for titles, Inter for the interface, Geist Mono for machine
  values.

## Decision log

History, newest last. Superseded decisions stay for traceability; each row states a decision and the
reason it had to be made.

| Date | Decision |
|------|----------|
| 2026-10-24 | Seed palette inherited (zinc/green); no identity invented. Two inherited contrast failures fixed with AA-passing shades of the same hues. |
| 2026-10-24 | **Direction set.** Functional, not presentation UI: clean, low saturation, natural, clear boundaries, little rounding. |
| 2026-10-24 | Direction, ten-variant line-up, and two prototypes (Feldbuch, Jagdkarte) explored in the gallery. All superseded. |
| 2026-10-24 | **Pflock colours are master data, not design tokens.** The checkpoint slider takes band colours as a prop; no Pflock palette in `globals.css`. |
| 2026-10-25 | **Scope marker: visual identity only, no user flows.** Work had drifted into journey design; the PO revoked it. |
| 2026-10-25 | **Identity adopted: Jagdkarte & Kompass** (map language). Superseded the same day. |
| 2026-10-25 | **Identity replaced: Bogenschuss Logbuch (Sleek Paperstyle).** The PO supplied a written guide: warm ivory paper, one hunter green as the only brand colour, warm ink, hairline pencil rules. Light is paper on a desk; dark is the same book by lamplight. |
| 2026-10-25 | **Two typefaces, two jobs.** Lora for titles only, Inter for the interface, Geist Mono for machine values. |
| 2026-10-25 | **Canonical palette adopted verbatim.** Token names and hex values match the PO's guide exactly; `--bg-surface-muted` is the one addition, a second sheet one step off the desk. |
| 2026-10-25 | **Role restriction replaces hue substitution.** The prescribed muted ink, amber, and strong contour fail AA as text (2.48:1, 3.08:1, 1.58:1), so each is restricted to the role where it is legible instead of being darkened. |
| 2026-10-25 | **Inputs sit on a raised sheet** (`--bg-elevated`); cards, tables, and header stay on `--bg-surface`. `--color-primary-hover` replaced `bg-primary/90`. |
| 2026-10-25 | **The gallery is domain-free.** Shooting vocabulary removed in favour of neutral German fixtures, so the gallery cannot imply a journey. |
| 2026-10-25 | **One name per colour; alias layer removed.** Two names per colour had contradicted each other (`--text-primary` is ink while the utility `text-primary` was the brand). Ink is now `text-ink`; `check:design` rejects the retired names. |
| 2026-10-25 | **Drawn controls are inked and get a real boundary token.** Thickening the contour exposed that `--border-strong` (1.58:1) cannot bound a control (WCAG 1.4.11 wants 3:1), so `--border-control` was added and `--border-strong` kept its non-interactive role. |
| 2026-10-25 | **Paper furniture as a named layer.** `HatchBand`, `Perforation`, `MarginNote`, `SheetNumber`, `RegisterLine`, `RulerEdge`, `DogEaredSheet`, `StampMark` replace ad-hoc markup. All decorative; none may be the only carrier of state. |
| 2026-10-25 | **A composed page is part of the review.** Plate 16 renders one whole screen, because parts can pass individually while the assembled page fails on density, hierarchy, or neighbouring contrast. |
| 2026-10-25 | **`Field` labels are attached, not drawn.** The composed page exposed that `Label` was a styled `span`, so every field was announced without a name. No token or contrast check could have caught it. |
| 2026-10-26 | **The 44px touch minimum is real, not asserted.** Hit-testing found 40 of 46 controls under 44px despite the rule: `inset-y` without `inset-x-0` collapsed the overlay, `inset` was eaten by the contour, and replaced elements generate no `::before` box at all. Fields are now drawn at 44px, overlays sized for their contour, dense rows grown, captions made part of the target. 182 controls across 16 plates pass. |
| 2026-10-29 | **A Pflock's colour is a value, not a token.** Master data stores a validated `#rrggbb`, so a swatch cannot map it through the palette. `PflockSwatch` sets `--pflock-farbe` to the stored value and paints `bg-[var(--pflock-farbe)]`; the hex is always drawn beside it, so colour never carries the meaning alone. This is the one inline style that references a data value rather than a token. |
| 2026-10-30 | **A form row aligns on the control, not on the bottom edge.** `items-end` looked right only while every field was equally tall; a hint under one field lifted its control above its neighbour. `Field`'s label row is now a fixed 18px, `FormRow` starts at the top, and `FormActions` repeats the label row as top padding so a button lands on the control line. `Button size="field"` (`h-11 px-4`) exists because `Input`/`Select`/`Textarea` are replaced elements drawn at 44px, and a `md` button (`h-10`) could never share their line. |

## Revision history

| Date | Change |
|------|--------|
| 2026-10-24 | Draft from issue #7 and the design-system work. |
| 2026-10-25 | Identity decided, then replaced by Bogenschuss Logbuch; tokens, components, rule, and gallery follow. |
| 2026-10-25 | Canonical palette, role-restricted contrast, domain-free gallery, one name per colour, inked controls with `--border-control`, paper furniture, and the composed page. |
| 2026-10-26 | Touch targets verified by hit-testing. Spec compressed to intent and history; the operational rules now live only in the design rule. |
| 2026-10-29 | Pflock swatch: a data colour painted through `--pflock-farbe`. |
| 2026-10-30 | Pflock picker: the vocabulary becomes swatch buttons plus a native colour input, selection marked by contour, check, and `aria-pressed`; `PflockMarker` shows the stored hex in mono. |
| 2026-10-30 | `Button size="field"` and the `Field`/`FormRow`/`FormActions` row contract: fixed label row, top-aligned row, actions on the control line; `items-end` retired for forms. |
