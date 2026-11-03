# Specifications

`specs/` is the version-controlled source of **product intent** for logandaim.at.
It is not a backlog. Work items, priorities, and progress live in
[GitHub Issues](https://github.com/TimerErTim/logandaim.at/issues) and the GitHub Project
(see [`.github/CONTRIBUTING.md`](../.github/CONTRIBUTING.md)).

| Record | Lives in | Owner |
|--------|----------|-------|
| Product intent, rules, acceptance criteria | `specs/` | Product Owner (PO) |
| Work items, priority, status | GitHub Issues + Project | PO (priority), implementer (status) |
| Delivery and review | GitHub Pull Requests | Implementer, reviewer |
| User-visible changes | [`CHANGELOG.md`](../CHANGELOG.md) | Implementer |
| Architecture decisions | `specs/decisions/` | Tech lead |

## Layout

```
specs/
├── README.md                    # This scheme
├── _template.md                 # Feature spec template (copy, do not edit in place)
├── product/
│   ├── charter.md               # Vision, roles, MVP boundaries, non-goals
│   └── <feature>.md             # One feature spec per product capability
├── decisions/
│   └── NNNN-<title>.md          # Architecture decision records (ADR)
├── 3d_archery_tracking_austria_product_requirements_document.md  # Original PRD (input)
└── legacy/                      # Superseded proof-of-concept specs
```

The original PRD is **input**, not approved scope. Approved scope lives in
[`product/charter.md`](product/charter.md) and the feature specs.

## Language

Specs are written in English. Domain terms use the German ubiquitous language
(`Anlage`, `Parkour`, `Ziel`, `Pflock`, `Runde`, `Schuss`, `Wertung`), matching code, API, and UI.

## Lifecycle

Every spec starts with a front-matter block:

```yaml
---
id: SPEC-<slug>
title: <short title>
status: Draft            # Draft | PO-approved | In delivery | Accepted | Superseded
owner: <github handle>   # PO responsible for the spec
issues: []               # GitHub issue numbers delivering this spec
superseded_by: null
---
```

| Status | Meaning | Who changes it |
|--------|---------|----------------|
| `Draft` | Being written or discussed. Not implementable. | Anyone (agents included) |
| `PO-approved` | Scope, priority, acceptance criteria approved. Can be split into issues. | PO only |
| `In delivery` | At least one linked issue is in progress. | Implementer |
| `Accepted` | All acceptance criteria verified in a released build. | PO only |
| `Superseded` | Replaced; `superseded_by` points to the successor. | PO only |

Rules:

1. Agent-written specs are always `Draft`. Only a human PO sets `PO-approved`, `Accepted`, or `Superseded`.
2. A change to approved behavior is a spec change first. Edit the spec, add a revision entry, and
   move it back to `Draft` if scope or acceptance criteria change materially.
3. Open questions block approval. Resolve them in the spec's decision log before approval.

## Traceability

- Every delivery issue links one spec section (`specs/product/<feature>.md#<anchor>`) and copies
  the acceptance criteria it covers. The spec lists the issue numbers in `issues:`.
- Every PR links its issue (`Closes #N`) and the spec section.
- Acceptance criteria are numbered (`AC-1`, `AC-2`, …) and stable. Tests and PRs reference them by number.
- User-visible changes get a `CHANGELOG.md` entry under `Unreleased` in the same PR.

## Writing a feature spec

1. Copy [`_template.md`](_template.md) to `product/<feature>.md`.
2. Fill problem, outcome, roles, scope, rules, and acceptance criteria. Keep technical design out;
   link an ADR in `decisions/` when a technical constraint matters to the PO.
3. List open questions. Ask the PO; record answers in the decision log.
4. Request PO approval through a `needs-PO` issue or PR review.

## Legacy

[`legacy/`](legacy/) holds the superseded proof-of-concept specs (billing, seminars, backend and
frontend architecture notes) that logandaim.at started from. They are `Superseded` as product
specs. The architecture notes stay useful as reference for the inherited Rust and Next.js structure
until replaced by ADRs.
