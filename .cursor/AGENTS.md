# logandaim.at — Repository Context

logandaim.at is an Austria-wide online logbook for 3D archery: a Rust backend (edition **2024**,
**async** throughout) and a Next.js App Router frontend that is exported statically. The backend
follows **Domain-Driven Design** and **hexagonal architecture**. Crates are prefixed `logandaim-`.

## Where the truth lives

Read the one that owns the question; do not restate it here.

| Question | Document |
|----------|----------|
| How work flows (issues, PRs, labels, CI, commits) | [`.github/CONTRIBUTING.md`](../.github/CONTRIBUTING.md) |
| What the product should do | [`specs/`](../specs/README.md) — `PO-approved` specs only |
| In which order to build one feature slice | [`docs/vertical-slice.md`](../docs/vertical-slice.md) |
| How to model domain types | [`crates/logandaim-core/src/domain/README.md`](../crates/logandaim-core/src/domain/README.md) — module docs |
| How the UI looks and behaves | [`.cursor/rules/design-system.mdc`](rules/design-system.mdc) |
| Server configuration and environment variables | [`backend/README.md`](../backend/README.md) |
| The code map (symbols, dependencies) | `graphify` (see [`graphify.mdc`](rules/graphify.mdc)) |

## Workspace layout

```
crates/
├── logandaim-core/         # Domain, use cases, port definitions
├── logandaim-api/          # Public API surface: DTOs, LogandaimAppApi, OpenAPI
├── logandaim-sqlite/       # SQLite/libsql repository adapter
├── logandaim-fakes/        # In-memory adapters, FixedClock
├── logandaim-filesystemstore/  # Object-store adapter
└── molting/                # Generic async migration framework
backend/server/             # Standalone HTTP server (logandaim-server)
frontend/src/               # Next.js UI (App Router, TanStack Query)
tasks/                      # mise task definitions
specs/                      # Product intent (PO-owned)
mise.toml                   # Tool, env, and task orchestration
```

## Layers

```
backend/server (Poem OpenAPI)          driving adapter
        ↓
logandaim-api — LogandaimAppApi, schema DTOs, HTTP status mapping
        ↓
logandaim-core — App, use cases, domain, ports
        ↓
logandaim-sqlite — SQLite UoW, repositories, migrations
```

Dependency rule: **inward only**. Core knows nothing about HTTP or SQLite.

## Language & naming

Docs, comments, commit messages, and all non-domain source are **English**. German is reserved for
the ubiquitous language: domain types and fields (`Anlage`, `Parcours`, `Ziel`, `Pflock`, `Runde`,
`Schuss`, `Wertung`, `Wertungszone`, `Benutzer`, `Konto`, `PasswortWiederherstellung`), SQLite
tables and columns, HTTP paths, JSON keys, and spec prose.

Naming ladder: `NeuerSchuss` (domain) → `SchussRegistrieren` (use case) → request DTO. A use-case
type keeps its German domain noun; the module it lives in does not.

**The domain concept stays German; the technical makeup around it is English.** A port keeps its
German noun (`BenutzerRepository`, `PasswortWiederherstellung`, `Drossel`) because that noun *is* the
ubiquitous language. Everything that dresses it is English: methods (`find_by_id`, `find_all`,
`create`, `update`, `delete`, `delete_by_benutzer`, `check_and_count`, `is_authenticated`), modules
(`repos.rs`, `throttle.rs`, `login.rs`, `password.rs`, `requests/benutzer.rs`), DTO suffixes
(`…Request`), and error plumbing. `KundenRepository::find_by_id`, never
`KundenSpeicher::bekomme_durch_identität`.

So: German nouns, English verbs and structure. A German *verb* in the application layer
(`prüfe_und_zähle`, `speichere`, `setze_rolle`) is a mistake — it is not domain vocabulary, it is
plumbing that happens to be spelled in German. A German noun with an English verb is correct:
`BenutzerRepository::delete_by_benutzer`.

**Auth-technical nouns are the one exception, and their English spelling is fixed.** German
engineering usage names these in English, so the code says `Session`, `AccessToken`,
`RefreshToken`, `SecretToken`, `SecretTokenHash`, `SigningKey`, and `PasswordHash` — see the
vocabulary table in [`specs/product/konto.md`](../specs/product/konto.md). The product term
`Anmelde-Sitzung` maps to the code term `Session`, one-to-one, and it applies to **every**
identifier built on the noun, not only the aggregate: `SessionAusgestellt`, `SessionPrüfen`,
`SessionAntwort`, `SessionLaden`. Never mix the two (`SitzungPrüfen` next to `SessionId`): one
concept, one spelling. German stays in the error text a user reads (`"die Sitzung ist nicht mehr
gültig"`) and in the product term in prose.

Use **proper UTF-8** in identifiers and persisted names. Never transliterate umlauts to
`ae`/`oe`/`ue` (`BenutzerLöschenFehler`, not `BenutzerLoeschenFehler`; `Bestätigung`, not
`Bestaetigung`); never paper over it with `serde(rename)`. This is enforced by review, not by a task.

## Patterns an agent can silently break

- **`UseCase`** — one struct per business operation; `async fn perform(self, ctx)` returns
  `ResultReport<Output, Context>`. Use cases enter and finish the UoW themselves.
- **UoW** — `ctx.enter()` for an owned UoW, `ctx.sub(&uow)` for nested work (nested `enter` joins
  the outer TX; nested `commit`/`rollback` are no-ops). Collect work without `?` across the UoW,
  then `uow.finish(result, ctx)`. Dropping a UoW that neither committed nor rolled back panics.
- **Repositories are dumb** — they persist and load domain types; state transitions belong in the
  domain or the use case. Every repository has the same five shapes: `find_by_id`, `find_all` (or a
  scoped `find_by_*`), `create(Neues*)`, `update(&mut Versioned<T>) -> RepositoryResult<()>`,
  `delete(Versioned<T>) -> RepositoryResult<()>`. `update` writes the whole row back and bumps the
  version **in place**; `delete` takes the aggregate as it was loaded, so a stale version is a
  `VersionMismatch` in both directions. The load → domain mutation → write-back loop is the
  caller's, and a purpose-built mutator (`revoke_all`, `set_role`) is a sign the logic belongs in a
  use-case loop instead.
- **Errors** — `Report<E>` at every boundary; use `.change_context()` directly. Value-object
  validation is the one exception: plain `Result` with a dedicated `thiserror` enum.
- **Concurrency** — `Versioned<T>` bundles an entity with its optimistic-lock version.
- **All mutations go through `App::execute` / `App::execute_fn`.** No adapter calls a repository
  directly. `App` only builds an `ExecutionContext`; it does not wrap the use-case result.

## Comments

A comment earns its place by saying something the code cannot. That is a narrow bar and it is the
reviewer's, but two cases are always wrong:

1. **Narrating the obvious.** `// set the field`, `// return the result`, `// loop over the ids`, a
   doc comment restating the signature, a paragraph where a sentence carries the same fact. Delete
   it; the identifier already says it.
2. **More comment than code.** Within any function, item, or module doc, comment lines must not
   outnumber code lines. A long "why" belongs in the module doc or the spec, not stapled above the
   function that happens to need it.

Write the reason, not the mechanics: why a shape is this way, what a wrong alternative would break,
which acceptance criterion or rule pins it. One line beats three, and a module doc that explains the
file beats a comment on every block. If removing a comment loses nothing, it was never a comment.

## Tracing

Instrumentation lives at boundaries, in `application/instrumented.rs` and
`application/instrumented_repos.rs`. `App::execute` and `ExecutionContext::enter` carry spans;
adapters emit `debug!`/`trace!` events only, never duplicate spans.

| Level | Use for |
|-------|---------|
| `info` | Use-case business milestones |
| `debug` | Transaction commit/rollback, port I/O, repository writes |
| `trace` | Repository reads, `SystemClock` |

1. No `#[instrument]` on a use-case `perform` — `App::execute` already names it, and a second span
   hides the first.
2. No spans on the public `UnitOfWork`; wrap at `InstrumentedUnitOfWork` and the ports.
3. New repository trait: add an `Instrumented*Repository` with `impl_instrumented_repo!`, add a
   `RepoStorage` field, and return it from `InstrumentedUnitOfWork`. The `uow` field stays last —
   [`docs/vertical-slice.md`](../docs/vertical-slice.md) explains the pointer wiring.
4. New non-repository port: add an `Instrumented*` wrapper and wire it in `ExecutionContext`.

`backend/server/src/tracing_setup.rs` reads `LOGANDAIM_LOG_LEVEL` (default `info`).

## API layer

`LogandaimAppApi` is a thin translation: request → use case → schema DTO. No business rules, no
`poem` or OpenAPI types leaking into core, and none in the facade either: the library compiles with
`--no-default-features`. Schema DTOs are separate from domain types; request DTOs are `…Request` and
convert with `From`/`TryFrom`.

`LogandaimApiSpec` in `spec.rs` is the whole HTTP edge: the `#[OpenApi]` handlers, the response
envelopes with their headers, and the cookie. Handlers take native `poem_openapi` extractors — never
a raw `&Request`. It also resolves the HTTP status: a `StatusCode` attached to the report first, then
`HttpStatusMapping` on the current context, then the first nested mapped context, else 500. An
orchestration error variant returns `None` so the domain error beneath it can speak. Statuses are
attached at the API boundary, never in the domain.

After any schema or route change, run `mise run sync:openapi`; `mise run check` fails on drift.

## Testing

| Layer | Where | What |
|-------|-------|------|
| Domain unit | `#[cfg(test)]` in the domain source file | Isolated value-object and aggregate rules |
| Business conformance | `crates/logandaim-core/tests/cases/` | Full use-case flows against `logandaim-fakes` |
| Adapter conformance | `crates/logandaim-sqlite/tests/` | The same cases against real SQLite |
| E2E / API | `crates/logandaim-api/tests/e2e/` | `LogandaimApiTestClient` over Poem `TestClient` |

Cases live once in `logandaim-core/tests/cases/` and are shared into the SQLite harness with
`#[path]`; each harness supplies its own `base_app_builder()`. Tests that need a fixed date use
`logandaim_fakes::FixedClock`. Never copy a case to run it twice.

`cases/` proves UoW parity across adapters. A test that validates one adapter, or one use case
against one adapter, is a normal integration test beside it (`crates/logandaim-core/tests/*.rs`),
not a shared case.

`mise run test` wraps two nextest profiles (`.config/nextest.toml`): `test:unit` runs everything that
needs no external service, `test:e2e` runs the `e2e` binary with the Mailpit daemon.

## Frontend

`LogandaimApi` (`frontend/src/api/logandaim-api.ts`) is a transport-agnostic interface;
`HttpLogandaimApi` implements it and `createLogandaimApi()` in `frontend/src/api/index.ts` is the
only place that picks one. Components never see HTTP and never call the API directly: all server
state goes through the TanStack Query hooks in `frontend/src/api/hooks/`, with keys in
`query-keys.ts`. Never mirror server state in `useState`; use `queryClient.setQueryData` for
optimistic updates.

Configuration is static and baked in at build time (`NEXT_PUBLIC_LOGANDAIM_*`); there is no runtime
config file and no async bootstrap. See [`docs/vertical-slice.md`](../docs/vertical-slice.md) for
the seam and the state rules.

**UI work starts by reading [`.cursor/rules/design-system.mdc`](rules/design-system.mdc).** It owns
tokens, components, contrast, touch targets, and the `/dev` gallery; this file does not. Values live
only in `frontend/src/app/globals.css`, used through semantic utilities. A new token, pairing,
variant, or convention updates that rule and `specs/product/visual-identity.md` in the same commit.

## Tooling

`mise` owns the toolchain and every task; see `mise.toml` and `tasks/`. `mise run ci` is the whole
gate (`check`, `build`, `test`, `verify`) and is what CI runs.

The Rust toolchain is pinned in `rust-toolchain.toml`; do not bump the nightly without testing,
because newer nightlies break poem-openapi's lifetime capture across `.await`.

## Version Control

- **Merge, don't rebase, to integrate upstream.** When a branch conflicts with or falls behind its
  base, create a merge commit (`jj new <branch> <base>@github`, resolve, commit). Conflict
  resolutions stay visible in history; never rewrite pushed commits to hide them.
- Rebase only for unpushed local work or explicit maintainer instruction. Never force-push a shared
  or reviewed branch.
- Commit identity and signing key come from the `CURSOR_AGENT`-conditional jj config. Never override
  `user.name`, `user.email`, or `signing.key` inline, and never disable signing.

## Adding a feature

Walk [`docs/vertical-slice.md`](../docs/vertical-slice.md) in order — domain → port → use case →
UoW wiring → fakes → migration → SQLite repository → API → tests → frontend — and copy its shape,
not its concrete names. Domain changes stay in core. Tests at every affected layer reference their
acceptance criterion (`ac_3_…`).

The changelog is end-user facing: log the change a user experiences, never the work behind it. The
rule, including why `Unreleased` may stay empty before the first release, is in
[`CONTRIBUTING.md`](../.github/CONTRIBUTING.md#changelog-scope).
