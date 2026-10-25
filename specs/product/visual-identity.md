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
> under issue #7 and reviewed by looking at `/dev`. The identity is decided: **Bogenschuss Logbuch
> (Sleek Paperstyle)** (PO instruction, 2026-10-25), and the tokens, components, and screen
> composition follow it. It supersedes the earlier Jagdkarte & Kompass proposal. The spec stays in
> `Draft` until the PO formally approves this document.

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

- [x] Q-1: Design direction and palette. Answered: **Bogenschuss Logbuch (Sleek Paperstyle)** — the
  app reads like a handwritten hunting logbook on warm paper, with one hunter green as the only
  brand colour (see the decision log; supersedes the earlier Jagdkarte & Kompass answer).
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
| 2026-10-24 | **Pflock colours are master data, not design tokens.** The checkpoint slider takes its band colours from the Parcours/Ziel data as a prop. No Pflock palette exists in `globals.css`; a stake's colour is content, and the design system must not own it. | TimerErTim |
| 2026-10-24 | **Stress-test prototypes in the gallery:** a `CheckpointSlider` whose track is banded by the Pflock colours with free movement and a named legend, plus a Pfeil capture table with a per-row Wertungszone control, an optional-arrow flag, a per-row distance drawer, a draft row that creates the next Pfeil, and drag-and-drop reordering. Both are dev-only; the logging screen itself waits for the spec in issue #11. | TimerErTim |
| 2026-10-24 | **Identity proposal "Feldbuch & Wegmarkierung" prototyped (Draft, not decided).** A gallery section shows four signature elements drawn from the material culture of the parcours instead of screen culture: the history as a ledger entry (`Feldbuch`) rather than a dashboard, the rotated square as the "you are here" marker (`Wegmarke`), the ruler tick as divider and read-only distance display (`Maßband`), and the `Schuss` drawn as a hit on the ring diagram of the target face rather than as a badge. Every element carries its meaning in text or shape as well as colour; no token or production component changes. Q-1 stays open; the PO decides in the gallery. | Agent (Cursor), proposing for TimerErTim |
| 2026-10-24 | **Ten-variant identity line-up prototyped (Draft, not decided).** A gallery section renders ten distinct design philosophies as the same mini-kit (action pair, Wertungszonen, distance, capture-row), each with a sketched local palette, a thesis, its one structural signature, and its named risk: Feldbuch, Jagdkarte, Werkstattnotiz, Werkzeug, Siegel, Signal, Bergführer, Schrebergarten, Druckstock, Nachtjagd. Sketched palettes are local to the section; the tokens in `globals.css` are untouched. Q-1 stays open. | Agent (Cursor), proposing for TimerErTim |
| 2026-10-24 | **"Jagdkarte & Kompass" deepened after the PO picked variant 2 (Draft, not decided).** The line-up's map proposal is developed into its own gallery section in a day and a night version: a map-sheet header with title, sheet number, and a labelled north arrow; section separators as hatched contour bands; the four Wertungszonen with cartographic fills (dots = Spot, solid = Kill, hatching = Body, empty dashed = Miss) plus a written legend, so the pattern carries meaning together with the text, never instead of it; the Runde drawn as a dashed way with numbered Pflock marks, labelled stage distances, and a filled triangle as the position marker; distance shown as a real alternating scale bar with the metre rule written out ("ein Strich = 10 m") next to the value; capture rows carrying map-style coordinates in mono. The map language is always commentary, never the only signal, and every sketched value stays local to the section. Q-1 stays open. | Agent (Cursor), proposing for TimerErTim |
| 2026-10-24 | **Geist now actually renders.** The `next/font` variables were declared on `<body>`, where the root-level `--font-sans` mapping cannot see them, so the app silently fell back to `system-ui`. They moved to `<html>`. | Agent (Cursor) |
| 2026-10-25 | **Identity chosen: Jagdkarte & Kompass.** PO instruction: chase the map-and-compass philosophy through the whole application. The app reads like a map sheet from the 3D parcours: map paper in light, night-map green-black in dark, one map green for the primary action, and five signature elements (Kartenkopf, Konturband, Weg, Maßstab, Koordinaten). Q-1 closes. | TimerErTim |
| 2026-10-25 | **Token layer and components follow the identity.** `globals.css` moves to the map palette and gains `--contour` (the hairline behind hatching, scale bars, and sheet rules) plus the `kontur-band`, `kontur-band-quiet`, and `weg-linie` pattern classes. `CardHeader` carries the quiet hatch band, `Separator` draws the contour band, `WorkflowSteps` draws the way with a triangle at the current position, and the active sidebar item is marked by a waymark triangle. All documented pairs pass AA in both themes. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Design rule rewritten as a rulebook.** `.cursor/rules/design-system.mdc` states the identity, the map language, the rules, and screen composition as instructions an agent can follow literally; the history lives only in this decision log. A "Screen composition" section defines how any screen is assembled from the map language, so subagent-built mockups can be checked for consistency against it. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Scope marker: visual identity only, no user flows.** The PO revoked work that drifted into journey design (an atlas of sessions, a routed parcours map). Whether a parcours is shown as a map, a progress bar, or a plain step list is a product decision and is explicitly *not* settled by this identity. The gallery's Kartographie section therefore shows the map vocabulary as elements only — contour lines, elevation profile, compass rose, subdivided scale bar, graticule, neatline, legend — with no screens and no navigation. "Screen composition" in the design rule governs appearance, not flow. | TimerErTim |
| 2026-10-25 | **Elements retuned to the instrument reading of "sleek".** Rounded corners tightened to `rounded-sm` for controls and `rounded-md` for containers, so the interface reads as a measuring instrument rather than a toy. `Progress` and `Slider` tracks are divided by contour hairlines at the quarter points (a scale that is divided, not just filled) and the thumbs are squared. `Badge` is set as a map label: small, hairline, mono, tracked. `Tabs` is a sheet index on a hairline base with the active item marked by a filled rule *and* a change of ink. `Table` headers and `DropdownMenuLabel` use tracked mono caps. The `Switch` thumb drops its shadow for a hairline, per flat-by-default. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Identity replaced: Bogenschuss Logbuch (Sleek Paperstyle).** The PO supplied a written design guide and asked for it to be applied and extended for dark mode. The map-and-compass language is superseded. The app is now a hunting logbook on paper: warm ivory paper, one hunter green (`#1B3B2B`) as the only brand colour, warm ink, and hairline pencil rules. Light is paper on a desk; dark is the same book read by lamplight — warm charcoal, never pure black, with the green lightened to sage so it keeps its contrast. `Jagdkarte & Kompass` stays in the history and in the archived gallery line-up, but no longer governs any token. | TimerErTim |
| 2026-10-25 | **Two typefaces, two jobs.** Lora is the book face and is used for titles only — page, card, and dialog headings — so the logbook's character lives in the headings. Inter replaces Nunito Sans as the interface face: menus, tables, forms, and all figures. Geist Mono stays for machine values. Weight sets are declared in `layout.tsx`. | TimerErTim |
| 2026-10-25 | **Amber darkened to stay legible.** The guide's amber (`#D97706`) measures 3.08:1 on paper and fails WCAG AA as text, so `--warning` uses its darker sibling (`#B45309`, 4.86:1). The hue family is preserved; only the lightness changed. Every documented pair now passes AA in both themes (18 measurements in the gallery). | Agent (Cursor) |
| 2026-10-25 | **Paper and stamp vocabulary.** `Card` gains a `papier` shadow (soft in light, `none` in dark) so sheets lift off the desk without a heavy border. `Badge` becomes a rubber stamp: hairline box, tracked caps, tinted rather than filled. `Separator` draws a tick rule instead of a hatch band. `Input`/`Select`/`Textarea` stay flat and turn `--primary` on focus. `CardHeader`'s band drops to a 1px rule. The gallery's map vocabulary is deleted and replaced by a Datenbilder section: target face with plotted hits, a measurement series on millimetre paper, a ruled scale, and a stamp legend. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Ground-up component and gallery rebuild.** Every primitive was rewritten against the identity rather than recoloured: `--schatten-papier` became `--shadow-paper` and a second `--shadow-overlay` separates sheets from overlays; the `papier`/`kontur-band`/`weg-linie` classes became `paper`/`paper-overlay`/`rule-ticks`/`rule-ticks-quiet`/`leader`, plus `select-chevron` and `checkbox-tick` masks that inherit `currentColor` instead of hardcoding a stroke colour. `Button`'s `icon` size draws at `md` and takes its 44px target from a `before` overlay; `secondary` is now a hairline on paper. `CardHeader` closes with a tick rule. `Alert` became a margin note with a coloured left rule. `Table` gained `TableKeyValue` and lost zebra banding. `Progress` gained fifth-ticks; `ToggleGroup` became a shared-contour segmented strip; `Tabs` is a sheet index; `Label` is tracked caps. The `/dev` page is now a printed specimen book — cover title block, table-of-contents index, 13 numbered plates, each drawn on `.light` and `.dark` — and the archived proposal sections (`Philosophien`, `Feldbuch`) were deleted, since a superseded direction belongs in this log, not on the page. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Design rule rewritten in English, scoped to appearance only.** `.cursor/rules/design-system.mdc` was translated and reframed: it now describes *how* any screen must look and never *what* the product does — no parcours, no data analysis, no domain workflows. The product's own guide's `DO / DON'T` table became the rulebook's spine, stated first and declared to win over any rule further down. Domain nouns were stripped from token descriptions, component doc blocks, and gallery rationales, so the identity can be reviewed without knowing the product. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Three guide violations fixed in code.** `app-shell`'s sticky header carried `bg-surface/80 backdrop-blur-md` — glassmorphism, explicitly forbidden — and is now opaque `bg-surface`. A dragging table row used `shadow-lg`; it is now marked by `ring-1 ring-primary`, keeping the system flat. `--background` moved from `#F5F1E8` to the guide's `#F4F1EA`; contrast is unchanged (12.66:1 foreground, 5.04:1 muted) and all 18 documented pairs still pass AA. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Tokens renamed to the guide's canonical palette.** The PO asked for the strict semantic system to be applied verbatim, so the variable names now match the guide's own vocabulary: `--bg-canvas`, `--bg-surface`, `--bg-elevated`, `--border-subtle`, `--border-strong`, `--text-primary`, `--text-secondary`, `--text-muted`, `--color-primary`, `--color-primary-hover`, `--color-primary-subtle`, `--semantic-impact`, `--semantic-warning`, `--semantic-success`, `--semantic-info`. Every prescribed hex value is in use exactly as written. `@theme inline` keeps the established utility names (`bg-surface`, `text-muted-foreground`, `border-border`, …) as aliases, so component markup did not churn, and adds `bg-surface-elevated`, `text-placeholder`, `border-border-strong`, `bg-primary-hover`, and `text-info`. `--bg-surface-muted` is an addition not in the guide: a second sheet one step off the desk (`#F0ECE3`), chosen because the guide's muted ink measures 4.47:1 on the first candidate and would have failed AA on it. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Supersedes the "amber darkened" decision: the prescribed amber is restored and role-restricted instead.** `--semantic-warning` is `#D97706` again, exactly as the guide specifies, and `--text-muted` is `#9E9A93`, `--border-strong` is `#CFCAC0`. Those three measure 3.08:1, 2.48:1, and 1.58:1 on paper and cannot carry text, so the rulebook now defines a role restriction table rather than substituting a darker hue: muted ink is placeholders and disabled controls only; amber is rules, washes, icons, and borders, and a warning that must be *read* is ink on the amber wash; the strong contour is a 1px field boundary, never a text carrier. `--semantic-success` (`#15803D`, 4.45:1 on canvas) is limited to text on `--bg-surface`, where it measures 4.85:1. `Badge`'s `warning` variant is therefore ink on an amber tint, not amber text. All nine documented pairs still pass AA in both themes. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **Inputs sit on a raised sheet.** `Input`, `Textarea`, `Select`, and `SearchInput` moved from a transparent fill with a hairline to `--bg-elevated` with a `--border-strong` contour, per the guide's role for that token. Dialogs and dropdown menus also moved to `--bg-elevated`; cards, tables, and the header stay on `--bg-surface` and keep `--border-subtle`. `--color-primary-hover` replaced the `bg-primary/90` opacity trick on primary buttons, because a hover state is now a token rather than a derived value. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **The gallery is domain-free.** All shooting vocabulary was removed from `/dev`: `Pfeil`, `Pflock`, `Wertungszone`, `Kill`, `Body`, `Miss`, `Spot`, `Treffer`, `Parcours`, `Runde`, `Schuss`, and the place name are gone, replaced by neutral German UI fixtures (`Eintrag`, `Vorgang`, `Zeile`, `Status`, `Kategorie`, `Fortschritt`, `Alpha`/`Beta`/`Gamma`/`Delta`). The domain-based `pfeil-table-section.tsx` and the target-face `datenbilder` drawings were deleted in favour of the already domain-free `sortable-table-section.tsx` and `data-figures-*` pair, both of which were written in English and were ported to the German plate titles. Every gallery string is German while code identifiers and comments stay English, per the project rule. Checkpoint band colours, which were raw hex literals, now use `var(--…)` tokens. | TimerErTim (direction), Agent (Cursor) |
| 2026-10-25 | **One name per colour; the alias layer removed.** The `@theme inline` block had been keeping the shadcn utility names (`text-foreground`, `text-muted-foreground`, `border-border`, `text-destructive`, `bg-background`, `bg-surface-elevated`) as aliases of the canonical tokens, so the app carried two vocabularies and the aliases quietly contradicted the design system: `--text-primary` is ink, but the utility `text-primary` was the brand green. Precedence now goes to the design-system names, and the utilities mirror the tokens one to one — `bg-canvas`, `bg-surface`, `bg-elevated`, `bg-surface-muted`, `border-subtle`, `border-strong`, `text-secondary`, `text-muted`, `text-impact`, `bg-primary`, `border-primary`, `outline-primary`. The `text-primary` collision is resolved in favour of the brand: `primary` means hunter green in every utility, and ink is **`text-ink`**. All 34 files that used the old names were migrated, `--color-ring` and `--color-contour` were dropped in favour of `outline-primary` and `border-subtle`, and `check:design` now rejects the retired names so the alias layer cannot creep back. | TimerErTim (direction), Agent (Cursor) |


## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-24 | Draft from issue #7 and the design-system work. | Agent (Cursor) |
| 2026-10-25 | Identity decided (Jagdkarte & Kompass); token layer, components, and design rule follow it; rule rewritten as a rulebook. | TimerErTim, Agent (Cursor) |
| 2026-10-25 | Identity replaced by Bogenschuss Logbuch (Sleek Paperstyle): paper palette, Lora + Inter, stamp and paper vocabulary, Datenbilder gallery. All pairs pass AA in both themes. | TimerErTim, Agent (Cursor) |
| 2026-10-25 | Design rule translated to English and scoped to appearance only; DO/DON'T table added as the rulebook's spine. Three guide violations fixed: sticky-header glassmorphism, drag shadow, and the background moving to the guide's exact value. | TimerErTim, Agent (Cursor) |
| 2026-10-25 | Tokens renamed to the guide's canonical semantic palette and all prescribed hex values adopted verbatim; contrast handled by role restriction instead of hue substitution; inputs and overlays moved to the elevated sheet; dev gallery made fully domain-free and German. | TimerErTim, Agent (Cursor) |
