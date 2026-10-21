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
| User-visible changes | [`CHANGELOG.md`](../CHANGELOG.md) | Keep a Changelog format. |

No backlog lives in local files. `tasks/` holds `mise` task definitions only.

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

## GitHub Project setup

Created once by a maintainer (the API tools agents use cannot create projects or milestones):

0. Create milestones `M0 Rebrand`, `M1 MVP`, `M2 Offline`, `Later` (see [charter](../specs/product/charter.md#milestones)).
   Assign seeded issues by their `Milestone:` line.
1. Create a project "logandaim.at" (Board layout) under the repository owner and link the repository.
2. Status field options: `Inbox`, `Ready`, `In progress`, `In review`, `Done`.
3. Add a single-select field `Priority` with `P0`–`P3` (mirrors the label).
4. Views: "Board" grouped by Status, and "MVP" table filtered to milestone `M1 MVP`, sorted by Priority.
5. Enable the built-in workflows: item added → `Inbox`, PR merged → `Done`, issue closed → `Done`.
6. Enable auto-add for issues from this repository.

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
