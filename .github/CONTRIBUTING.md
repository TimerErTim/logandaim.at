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

`Unreleased` is branch-dependent, not phase-dependent:

- Pull requests into `dev`: `Unreleased` collects the entries for the next release. It may be empty
  when there is nothing new to record yet.
- Release pull requests (`dev` -> `main`): every entry moves into the dated `## [X.Y.Z]` section, so
  `Unreleased` must be **empty** and that dated section must have entries. `mise run check:release`
  enforces both.

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

## Branches

`dev` is the integration branch and the default branch. `main` is the release branch.

| Branch | Purpose | Pull requests into it |
|--------|---------|------------------------|
| `dev` | Integration. Everything merges here first. | Feature, fix, chore, and spec branches. Uses the default pull request template. |
| `main` | Releases. A merge here deploys. | Only `dev` and `hotfix/*`. Uses the release template. |

A release pull request (`dev` -> `main`) bumps the version, writes the dated changelog section, and
its description becomes the release body. Merging it deploys the static frontend to GitHub Pages
and creates a source-only GitHub release tagged `v<version>`. See the
[release template](PULL_REQUEST_TEMPLATE/release.md).

Tags are created by the workflow, never pushed by hand, so a failing gate cannot leave a tag
behind.

### Merge method

Pull requests **merge**; they are never squashed or rebased. Release pull requests carry `dev`'s
merge commits into `main`, which is inherently non-linear, and rewriting history would contradict
`.cursor/AGENTS.md` ("merge, don't rebase, to integrate upstream"). The branch rulesets allow only
merge commits, and `main` keeps no linear-history rule. Conflict resolutions stay visible in
history.

## Checks

| Check | Runs on | Meaning |
|-------|---------|---------|
| `pr/base` | Every pull request, every push to `dev`, and the release gate on `main` | The full gate: `mise run ci`. |
| `pr/dev` | Pull requests into `dev` | Conventional title and a `Closes #N` link. |
| `pr/main` | Pull requests into `main` | Conventional title, `dev` or `hotfix/*` source, and a `## Release description` section. |
| `release-check` | Pull requests into `main`, called by the push to `main` | Versions agree, version bumped past the last release tag, dated changelog section with entries, the release does not exist yet, and the release body can be assembled. |
| `deploy/pages` | Push to `main` | Deploys `frontend/out` to GitHub Pages. |
| `deploy/release` | Push to `main` | Creates the source-only GitHub release. |
| `deploy/backend` | Push to `main` | Reserved for #15 and #24. |

`pr/dev` and `pr/main` both always run and pass when they do not apply, so their names exist on
every pull request and can be required on either branch.

`release-check` lives in its own workflow because `pr.yml` only triggers on pull requests. Being a
reusable workflow, `main.yml` also calls it as a job before `deploy/pages`, so a release that
bypassed its pull request still fails before any deployment or tag. It is the single release gate:
the version, the changelog, the absence of an earlier release, and the ability to assemble the
release body are all validated there, so `deploy/release` only publishes what the gate already
proved.

Required checks: `pr/base` and `pr/dev` on `dev`; `pr/base`, `pr/main`, and `release-check` on
`main`. Deployment jobs are deliberately not required checks: they only run on push to `main`, so
they would never report on a pull request.

## Local gate

`mise run ci` is the whole delivery gate: `check` (formatting, clippy, biome, `tsc`, OpenAPI drift,
version agreement), `test`, `build`, and `verify` (daemon-based smoke tests of the built frontend
and backend). CI runs the same task, so a green local run means a green pipeline unless the
environment differs.

The Pages deployment builds with `NEXT_PUBLIC_LOGANDAIM_API_URL` (repo variable
`LOGANDAIM_API_URL`) and a base path taken from `actions/configure-pages`. The export is deployed
as built; `pr/base` already smoke-tested the same code before the merge.

Useful tasks:

| Task | Purpose |
|------|---------|
| `mise run ci` | Full gate. |
| `mise run verify:frontend`, `mise run verify:server` | Start one service as a daemon and smoke-test it. |
| `mise run check:release` | Validate a release pull request. |
| `mise run release:version <kind>` | Bump the workspace and frontend versions. |
| `mise run release:notes <version> --pr <n>` | Preview the release body. |
| `mise run release:publish` | Create the release for the current commit (the workflow runs it). |
| `mise daemons stop` | Stop the verify daemons after a local run. |

## Workflow

```
Draft spec ──PO approves──▶ issues (Inbox) ──Ready gate──▶ Ready ──▶ In progress ──▶ PR to dev ──▶ In review ──merge──▶ dev
                                                                                                          │
                                                                                          release PR dev -> main ──merge──▶ main (deploy + release)
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
3. `pr/base` passes in CI, on the same runner image. Release pull requests also pass
   `release-check`.
4. `CHANGELOG.md` has an `Unreleased` entry for user-visible changes.
5. A human reviewed product behavior and architecture.

Release pull requests additionally pass `release-check` and `pr/main`: the version differs from the
last release, the changelog has a dated section for it, and the pull request carries a
`## Release description`.

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
3. Branch from `dev` and implement one issue per branch and pull request.
4. Implement the vertical slice in the order `.cursor/AGENTS.md` prescribes. Add tests at each affected layer.
5. Run `mise run ci`. Run `mise run sync:openapi` if the API changed (the gate also detects drift).
6. Update `CHANGELOG.md` and the spec's `issues:` list.
7. Open a pull request from the template, targeting `dev`. Report commands and results.

## Commits

Conventional commits (`feat(backend): …`, `fix(frontend): …`, `docs(specs): …`, `chore(ci): …`).

Agents commit under their own author identity. Agent sessions (`CURSOR_AGENT` set) use a separate
agent account, `TimerErTim - Agent <agent.timerertim@gmail.com>`, with its own SSH signing key, selected
by `~/.config/jj/conf.d/agent.toml`. Do not pass an author, email, or signing key on the command line,
and do not reuse the maintainer identity for agent work.
