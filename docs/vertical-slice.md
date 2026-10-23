# Reference vertical slice — Schuss and Wertung

This document describes the one complete feature slice in the repository at the time of
writing: **register a Schuss at a Ziel** and **summarize the Wertung for a list of
Schüsse**. It exists so a future agent can see every layer wired together after the
veterinary domain was removed, instead of guessing how the seams fit.

Ignore the product spec for a moment: this slice is deliberately smaller than
[`specs/product/wertung.md`](../specs/product/wertung.md). There is no Runde, no
Parcours, no Pflock, no distance, no authentication. What it does show is the *shape* of
a slice.

## The two operations

```
POST /api/schuss   { zielId, pfeil, wertungszone }  -> Schuss
POST /api/wertung  { schussIds: [...] }             -> Wertung
```

## File map, leaf to root

Copy this order when adding a slice. Replace `schuss` with your own German term; never
add veterinary names.

| Layer | File | What it contributes |
|-------|------|---------------------|
| Domain | [`crates/logandaim-core/src/domain/schuss.rs`](../crates/logandaim-core/src/domain/schuss.rs) | `ZielId`, `SchussId`, `Schuss`, `SchussFehler`. Private fields, validated constructors, `#[cfg(test)]` unit tests. |
| Domain (reuse) | [`crates/logandaim-core/src/domain/wertung.rs`](../crates/logandaim-core/src/domain/wertung.rs) | `Wertungszone`, `Punkte`, `Zielwertung`, `Rundenergebnis`. Already had AC-1..AC-8 tests. |
| Domain exports | [`crates/logandaim-core/src/domain/mod.rs`](../crates/logandaim-core/src/domain/mod.rs) | `pub mod schuss;` + re-exports. |
| Port | [`crates/logandaim-core/src/application/ports/schuss.rs`](../crates/logandaim-core/src/application/ports/schuss.rs) | `NeuerSchuss` input struct + `SchussRepository` trait. |
| Port exports | [`crates/logandaim-core/src/application/ports/mod.rs`](../crates/logandaim-core/src/application/ports/mod.rs) | One `mod` / `pub use` pair. |
| Use case | [`crates/logandaim-core/src/service/use_cases/schuss.rs`](../crates/logandaim-core/src/service/use_cases/schuss.rs) | `SchussRegistrieren`, `WertungBerechnen`, their error enums, the result type, and pure aggregation tests. |
| Use case exports | [`crates/logandaim-core/src/service/use_cases/mod.rs`](../crates/logandaim-core/src/service/use_cases/mod.rs) | `mod schuss; pub use schuss::*;` |
| UoW facade | [`crates/logandaim-core/src/application/uow.rs`](../crates/logandaim-core/src/application/uow.rs) | `UnitOfWork::schüsse()`, `UnitOfWorkImpl::schüsse()`, `LockedUnitOfWorkImpl` delegation. |
| Instrumented UoW | [`crates/logandaim-core/src/application/instrumented.rs`](../crates/logandaim-core/src/application/instrumented.rs) | `InstrumentedUnitOfWork::schüsse()`. |
| Instrumented repo | [`crates/logandaim-core/src/application/instrumented_repos.rs`](../crates/logandaim-core/src/application/instrumented_repos.rs) | `impl_instrumented_repo!` + `InstrumentedSchussRepository` + `RepoStorage` pointer wiring. |
| Fake | [`crates/logandaim-fakes/src/repository.rs`](../crates/logandaim-fakes/src/repository.rs) | `FakeDatastore` field + `FakeSchussRepository`. |
| Fake UoW | [`crates/logandaim-fakes/src/uow.rs`](../crates/logandaim-fakes/src/uow.rs) | Snapshot datastore, write through on `commit`, accessor. |
| Migration | [`crates/logandaim-sqlite/src/migrations/v0001_schuss.rs`](../crates/logandaim-sqlite/src/migrations/v0001_schuss.rs) | `UpMigration` with the table, registered in `MIGRATIONS`. |
| SQLite repo | [`crates/logandaim-sqlite/src/repos/schuss.rs`](../crates/logandaim-sqlite/src/repos/schuss.rs) | Row parsing/saving, zone string mapping. |
| SQLite UoW | [`crates/logandaim-sqlite/src/uow.rs`](../crates/logandaim-sqlite/src/uow.rs) | `schuss_repo` field + accessor. |
| API schema | [`crates/logandaim-api/src/schema/schuss.rs`](../crates/logandaim-api/src/schema/schuss.rs) | OpenAPI DTOs and `From` conversions. |
| API requests | [`crates/logandaim-api/src/api/requests/schuss.rs`](../crates/logandaim-api/src/api/requests/schuss.rs) | Inbound DTOs and `From` into use-case inputs. |
| API facade | [`crates/logandaim-api/src/api/mod.rs`](../crates/logandaim-api/src/api/mod.rs) | `LogandaimAppApi` methods. |
| HTTP routes | [`crates/logandaim-api/src/spec.rs`](../crates/logandaim-api/src/spec.rs) | Two `#[oai]` methods. |
| Status mapping | [`crates/logandaim-api/src/errors/http_status.rs`](../crates/logandaim-api/src/errors/http_status.rs) | `HttpStatusMapping` impls for the new errors. |
| Shared cases | [`crates/logandaim-core/tests/cases/schuss.rs`](../crates/logandaim-core/tests/cases/schuss.rs) | Cases reused by both conformance harnesses. |
| API e2e | [`crates/logandaim-api/tests/e2e/schuss.rs`](../crates/logandaim-api/tests/e2e/schuss.rs) | HTTP-level assertions including status codes. |
| Frontend contract | [`frontend/src/api/logandaim-api.ts`](../frontend/src/api/logandaim-api.ts) | The interface plus its types. |
| Frontend transport | [`frontend/src/api/http-client.ts`](../frontend/src/api/http-client.ts) | `openapi-fetch` implementation. |
| Frontend hooks | [`frontend/src/api/hooks/mutations.ts`](../frontend/src/api/hooks/mutations.ts) | `useSchussRegistrierenMutation`, `useWertungMutation`. |
| Frontend page | [`frontend/src/app/schuss/page.tsx`](../frontend/src/app/schuss/page.tsx) | The UI. |
| Frontend nav | [`frontend/src/lib/navigation.ts`](../frontend/src/lib/navigation.ts) | `paths.schuss`, sidebar item, breadcrumb label. |

## Order to work in

1. **Domain** — model the aggregate / value objects, validate in constructors, unit-test the
   rules. No serde, no SQL, no HTTP.
2. **Port** — write the repository trait and the `Neu…` input struct. Repositories only
   store and load.
3. **Use case** — `ctx.enter()`, call the port, `uow.finish(result, …)`, emit one `info!`
   event. Business logic stays in the domain or a pure function, not in `perform`.
4. **UoW + instrumented repo** — three small additions: accessor on `UnitOfWorkImpl`, on
   `UnitOfWork`, and on `LockedUnitOfWorkImpl`; then the wrapper and `RepoStorage` field.
5. **Fakes** — the same slice against an in-memory datastore, so the use cases are testable
   without SQLite.
6. **Migration + SQLite repo** — table first, then row parsing using `repos/common.rs`
   helpers.
7. **API** — schema DTOs (separate from domain), request DTOs with `From`/`TryFrom`, facade
   methods, routes, status mapping.
8. **Tests** — shared cases in `logandaim-core/tests/cases/` (they run against fakes *and*
   SQLite automatically), plus e2e for the HTTP surface.
9. **Frontend** — add methods to the interface, implement in `HttpLogandaimApi`, add hooks,
   build the page, register navigation, then run `mise run sync:openapi`.

## Why `RepoStorage` has that pointer wiring

`InstrumentedUnitOfWork` wraps the adapter's UoW and hands out instrumented repositories.
The wrappers must borrow the inner UoW, but the UoW is owned by the wrapper — a
self-referential borrow. `RepoStorage` is `Box::pin`ned so the address is stable, then the
wrappers store `*const dyn UnitOfWorkImpl` and call back through the trait accessor on
every method. That keeps the borrow rules sound without `unsafe` leaking outside this one
file.

Consequence when adding a repository: **declare the `uow` field last** in `RepoStorage`
and set the wrapper pointers after pinning, exactly like the existing code.

## Where things belong

| Concern | Home |
|---------|------|
| Invariants that must always hold | Domain constructor (`Schuss::from_parts`) |
| Requests that are malformed on arrival | API `ValidationError` |
| Cross-layer plumbing failures | `RepositoryError` from the port |
| HTTP status codes | `HttpStatusMapping` impls, never the domain |

Note the deliberate layering: `SchussRegistrierenFehler::Erstellung` returns `None` from
`http_status()` so the nested `SchussFehler` can speak (400) while repository failures stay
500/404. Do the same for a new slice — add a mapping rather than attaching a status.

## Frontend API seam

`LogandaimApi` is an interface; `HttpLogandaimApi` is one implementation and
`createLogandaimApi()` in [`frontend/src/api/index.ts`](../frontend/src/api/index.ts) is the
only place that chooses it. To run the core domain in the browser later (wasm), write a
second class implementing `LogandaimApi` and change that one line. Do not add a second
provider, a loading gate, or async config — `config` is static.

## Frontend state rules

- Server state goes through TanStack Query hooks in `frontend/src/api/hooks/`.
- The Schuss page keeps the list of *just created* Schüsse in component state on purpose:
  it is a user selection for the next request, not a cache of server data.
- Mutations invalidate the smallest matching key prefix.
