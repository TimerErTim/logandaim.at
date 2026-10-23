# Contributing and project management

logandaim.at is built mostly by AI agents under a human Product Owner (PO). This file defines
how work flows. GitHub is the hub: issues, project board, milestones, pull requests.

## Records

| Record | Where | Purpose |
|--------|-------|---------|
| Product intent | [`specs/`](../specs/README.md) | What and why. PO-owned. |
| Work items | [Issues](https://github.com/TimerErTim/logandaim.at/issues) | One bounded, reviewable unit each. |
| Roadmap and status | GitHub Project "logandaim.at" | Priority order and workflow state. |
| Milestone gates | [Milestones](https://github.com/TimerErTim/logandaim.at/milestones) | `M0 Rebrand`, `M1 MVP`, `M2 Offline`. No dates unless committed. |
| Delivery | Pull requests | Code, tests, evidence. |
| User-visible changes | [`CHANGELOG.md`](../CHANGELOG.md) | Keep a Changelog format. End-user facing only. |

No backlog lives in local files. `tasks/` holds `mise` task definitions only.

## CHANGELOG scope

`CHANGELOG.md` is written **for end users**, not for the team. It answers one question: *what is
different for someone using the product?* Only record changes a user can observe:

- new or changed user-facing behavior, fixes to broken behavior, removed features
- user-visible performance, reliability, or compatibility changes
- breaking changes users must react to

Do **not** record work that has no observable effect for users. This includes spec refinements,
spec and ADR drafting or approval, issue triage, label and milestone changes, refactors that keep
behavior identical, internal architecture or documentation changes, CI, tooling, dependency
updates, and test-only changes. The changelog is not a project-management or PO record; that is
what issues, PRs, specs, and the project board are for.

When unsure, ask: *if this shipped alone, would a user notice?* If not, leave it out.

Entries are phrased from the user's perspective — describe the effect, not the implementation.

### Before the first release

logandaim.at is a new project being built on top of the architecture of a previous one (see the
rebrand and domain-removal work in `M0 Rebrand`). Until the first release ships, there is no
released product and therefore no user who could notice a difference: everything in the tree is
either inherited base architecture, a placeholder, or work in progress. `Unreleased` may be empty
during that phase, and an empty `Unreleased` is not a missing entry.

Consequences:

- A change that only removes or reshapes inherited base material is **not** an end-user change.
  Example: dropping the Tauri desktop shell. No user ever ran logandaim.at as a desktop app, so it
  is not a removal they can notice — it is a fact about how the project was seeded, and it belongs
  in the issue/PR/spec history instead.
- The same holds for anything a user cannot run yet: scaffolding, ports adapted from the base
  project, placeholder screens, and behavior that exists only in specs.
- Write the first real entries when users can actually use the feature or the release. "First
  release" means the first build a person outside the team can run and compare against nothing.

## Labels and priority

Labels are defined in [`labels.yml`](labels.yml) and synced by a workflow. Edit the file, not the UI.

| Priority | Meaning |
|----------|---------|
| `P0` | Stop-ship. Production broken or data at risk. |
| `P1` | Required for the current milestone. |
| `P2` | Important, not blocking the milestone. |
| `P3` | Later or nice to have. |

Markers: `needs-PO` (blocked on a PO decision), `ready` (meets the Ready gate), `agent-ok`
(suitable for autonomous agent work), `blocked` (waiting on another issue).

## GitHub Project

Project: [logandaim.at](https://github.com/users/TimerErTim/projects/3), linked to this repository.

- Status: `Inbox`, `Ready`, `In progress`, `In review`, `Done`.
- `Priority` single-select `P0`–`P3`, mirrors the label.
- Milestones: `M0 Rebrand`, `M1 MVP`, `M2 Offline`, `Later` (see [charter](../specs/product/charter.md#milestones)).

Agents add new issues with `gh project item-add 3 --owner TimerErTim --url <issue-url>` and set
Status and Priority with `gh project item-edit`.

UI-only settings (no API exists), set once by a maintainer:

1. Views: "Board" grouped by Status, and "MVP" table filtered to milestone `M1 MVP`, sorted by Priority.
2. Built-in workflows: item added → `Inbox`, PR merged → `Done`, issue closed → `Done`.
3. Auto-add for issues and PRs from this repository.

Add more views only when a real coordination question needs one.

## Workflow

```
Draft spec ──PO approves──▶ issues (Inbox) ──Ready gate──▶ Ready ──▶ In progress ──▶ PR ──▶ In review ──merge──▶ Done
```

### Ready gate

An issue may move to `Ready` (label `ready`) only when:

1. Its spec is `PO-approved`.
2. Outcome and out-of-scope are written.
3. Acceptance criteria are numbered and observable.
4. Dependencies are closed or explicitly not blocking.
5. Priority and milestone are set.
6. Verification is named (test layer and commands).

Chores and spikes need no spec; they still need outcome and verification.

### Delivery gate

A PR may merge only when:

1. It links the issue (`Closes #N`) and the spec section.
2. Every covered AC is checked off with evidence.
3. `mise run check` and `mise run test` pass in CI; affected builds pass.
4. `CHANGELOG.md` has an `Unreleased` entry for user-visible changes.
5. A human reviewed product behavior and architecture.

## Agents

Agents may:

- Draft specs and ADRs (always `Draft` / `Proposed`).
- Split `PO-approved` specs into issues.
- Implement one `ready` issue per branch and PR, with tests.
- File bugs and spikes.

Agents must not:

- Approve specs, accept ADRs, or change priorities and milestones.
- Invent behavior a spec does not define. Stop, comment on the issue, add `needs-PO`.
- Merge their own PRs.

Agent session loop:

1. Pick the highest-priority open issue labeled `ready` and `agent-ok` in the current milestone.
2. Read the linked spec section and [`.cursor/AGENTS.md`](../.cursor/AGENTS.md).
3. Implement the vertical slice in the order `.cursor/AGENTS.md` prescribes. Add tests at each affected layer.
4. Run `mise run check` and `mise run test`. Run `mise run sync:openapi` if the API changed.
5. Update `CHANGELOG.md` and the spec's `issues:` list.
6. Open a PR from the template. Report commands and results.

## Commits

Conventional commits (`feat(backend): …`, `fix(frontend): …`, `docs(specs): …`, `chore(ci): …`).
Agents commit under their own author identity.
