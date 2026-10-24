---
id: ADR-0003
title: Semantic design tokens, shadcn-style primitives, and a dev-only component gallery
status: Proposed
date: 2026-10-24
deciders: TimerErTim
---

# ADR-0003: Design system and dev-only gallery

## Context

The frontend was inherited with hand-written Tailwind classes: each component picked raw palette
values (`bg-zinc-100`, `text-emerald-600`), dark mode was a media query only, and there was no
place to review what the product looks like. Issue #7 asks for a visual identity, but the PO cannot
choose one from prose and agents must not invent one.

The frontend is a static export (`output: "export"`), served from GitHub Pages, with no Node
runtime. Any review surface therefore has to be either absent from the build or dead weight in it.

## Decision

1. **One semantic token layer.** `frontend/src/app/globals.css` declares colour, radius, and font
   tokens for light and dark and maps them to Tailwind utilities in `@theme inline`. Components use
   the semantic utilities only. Dark mode is class-based (`@custom-variant dark`) with
   `prefers-color-scheme` as the default, so a subtree can be forced light or dark.
2. **shadcn-style primitives.** Variants are declared with `class-variance-authority`, class merging
   uses `clsx` + `tailwind-merge`, and Radix primitives back the non-trivial behaviours (dialog,
   dropdown, tooltip, tabs, slider, switch, radio group, toggle group, progress, separator). Public
   variant names of the existing components stay compatible with their call sites. `Select` stays a
   native `<select>` for one-handed, gloved use.
3. **A dev-only gallery at `/dev`.** `next.config.ts` exports a function of the Next phase and adds
   `dev.tsx` / `dev.ts` to `pageExtensions` only for `PHASE_DEVELOPMENT_SERVER`. Production keeps
   exactly `["tsx", "ts"]`. The gallery route is `src/app/dev/page.dev.tsx`; its sections live in
   `src/components/dev-gallery/*.tsx`. `build:frontend` asserts that `out/dev` does not exist and
   that gallery code is absent from `out`.
4. **The description is versioned next to the code.** `.cursor/rules/design-system.mdc` (always
   applied) describes the tokens, the component conventions, and the review surface;
   `specs/product/visual-identity.md` records the PO-level intent and the decision log. Token values
   never appear in either document; they live only in `globals.css`.
5. **A cheap automated guard.** `check:design` in `mise run check` fails when a token declared in
   `globals.css` is not named in the design rule, or when a raw palette class appears under
   `frontend/src` outside `globals.css`.

## Consequences

- A palette change is a single edit in one file, and the gallery shows its effect, including the
  contrast verdict, without a rebuild step.
- The design rule and the spec must be updated in the same commit as a token or convention change.
  The mechanical half is enforced by `check:design`; the prose half is a review checklist item.
- `pageExtensions` also feeds metadata-route and middleware resolution. The production list must
  stay identical to the pre-change list; the build assertion catches a mistake here.
- Radix primitives are client components, so the affected files carry `"use client"`, matching the
  existing components.
- The gallery renders inside the real `AppShell`, which is intentional: the shell is part of the
  identity under review. If a chrome-free view is ever needed, the route can move into a route group
  with its own root layout.
- A second Next app for the gallery was rejected: it duplicates dependencies and the token source
  for a page that only ever runs locally.
