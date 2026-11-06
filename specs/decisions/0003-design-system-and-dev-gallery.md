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
values (`bg-zinc-100`, `text-emerald-600`), dark mode was a media query only, and there was nowhere to
review what the product looks like. Issue #7 asks for a visual identity, but the PO cannot choose one
from prose and agents must not invent one.

The frontend is a static export (`output: "export"`) served from GitHub Pages, with no Node runtime.
A review surface therefore has to be absent from the build or dead weight in it.

## Decision

1. **One semantic token layer.** `globals.css` declares colour, radius, shadow, and font tokens for
   light and dark, mapped one-to-one to Tailwind utilities in `@theme inline`. One name per colour, no
   aliases: every utility suffix is the token suffix, except ink, which is `text-ink` because the
   brand owns every `primary` utility. Dark mode is class-based (`@custom-variant dark`) defaulting to
   `prefers-color-scheme`, so a subtree can be forced light or dark.
2. **shadcn-style primitives.** Variants via `class-variance-authority`, class merging via `clsx` +
   `tailwind-merge`, Radix behind non-trivial behaviour. Existing variant names stay compatible.
   `Select` stays a native `<select>` for one-handed, gloved use.
3. **A dev-only gallery at `/dev`.** `next.config.ts` exports a function of the Next phase and adds
   `dev.tsx` / `dev.ts` to `pageExtensions` only for `PHASE_DEVELOPMENT_SERVER`; production keeps
   exactly `["tsx", "ts"]`. Route: `src/app/dev/page.dev.tsx`, sections in
   `src/components/dev-gallery/`. The gate is trusted; the build does not re-assert it.
4. **Two documents, one job each.** `.cursor/rules/design-system.mdc` is the operative rulebook an
   implementer follows (tokens by name, component conventions, accessibility, screen composition).
   `specs/product/visual-identity.md` holds PO-level intent, acceptance criteria, and the decision
   log. Neither contains token values; those live only in `globals.css`. The rule also states the
   sync obligation: a token or convention change lands in both, in the same commit.
5. **A cheap automated guard.** `tasks/scripts/check-design.mjs` (run by `check:design` in
   `mise run check`) fails when a token is undocumented in the design rule, a raw palette class
   appears under `frontend/src` outside `globals.css`, or a retired utility name returns.
6. **Boundaries are split by role, not by weight.** `--border-subtle` rules structure,
   `--border-strong` contours non-interactive things, and `--border-control` bounds anything a user
   operates. The last is separate because a control outline is meaningful content under WCAG 2.2
   §1.4.11 and needs 3:1; `--border-strong` measures 1.58:1 and cannot carry that role. Thickening a
   failing hairline does not fix it — the colour has to change. Consequence for reviews: a new
   interactive control must use `border-control`, or it silently regresses accessibility without
   failing any automated check.

## Consequences

- A palette change is one edit in one file; the gallery shows the effect, including the contrast
  verdict, without a rebuild step.
- The mechanical half of "keep the docs true" is `check:design`; the prose half is the PR checklist.
- `pageExtensions` also feeds metadata-route and middleware resolution, so the production list must
  stay identical to the pre-change list. This is trusted rather than asserted at build time.
- Radix primitives are client components, so affected files carry `"use client"`.
- The gallery renders inside the real `AppShell` on purpose: the shell is part of the identity under
  review. A chrome-free view would need its own route group and root layout.
- A second Next app for the gallery was rejected: it duplicates dependencies and the token source for
  a page that only runs locally.
