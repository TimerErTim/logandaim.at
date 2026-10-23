# Vertical slices

A **vertical slice** is one thin feature that runs through every layer: domain → port →
use case → unit of work → adapter (fakes, SQLite) → API → frontend. Slicing vertically
keeps each increment shippable and shows every seam at once, instead of building all
repositories first and wiring them up later.

This document describes the shape of a slice and the order to build one. Concrete
features come and go; the seams below stay.

## File map, leaf to root

Copy this order when adding a slice. Names are placeholders (`<begriff>` is the German
ubiquitous-language term of the slice); never invent vocabulary that the specs do not use.

| Layer | File | What it contributes |
|-------|------|---------------------|
| Domain | `crates/logandaim-core/src/domain/<begriff>.rs` | Identity newtypes, the aggregate, its `Neues…` creation input, the `<Begriff>Fehler` enum, domain services, `#[cfg(test)]` unit tests. |
| Domain exports | `crates/logandaim-core/src/domain/mod.rs` | `pub mod <begriff>;` plus re-exports, so callers use `crate::domain::<Begriff>`. |
| Port | `crates/logandaim-core/src/application/ports/repos.rs` | The repository trait for the aggregate. Repositories only store and load. |
| Use case | `crates/logandaim-core/src/service/use_cases/<begriff>.rs` | One struct per operation implementing `UseCase`, plus its error enum. |
| Use case exports | `crates/logandaim-core/src/service/use_cases/mod.rs` | `mod <begriff>; pub use <begriff>::*;`. |
| UoW facade | `crates/logandaim-core/src/application/uow.rs` | Accessor on `UnitOfWork`, on `UnitOfWorkImpl`, and on `LockedUnitOfWorkImpl`. |
| Instrumented UoW | `crates/logandaim-core/src/application/instrumented.rs` | Accessor delegating to the instrumented wrapper. |
| Instrumented repo | `crates/logandaim-core/src/application/instrumented_repos.rs` | One `impl_instrumented_repo!` invocation plus a `RepoStorage` field. |
| Fake | `crates/logandaim-fakes/src/repository.rs` | A datastore map and the repository implementation over it. |
| Fake UoW | `crates/logandaim-fakes/src/uow.rs` | Datastore snapshot, write back on `commit`, accessor. |
| Migration | `crates/logandaim-sqlite/src/migrations/vNNNN_<begriff>.rs` | `UpMigration` with the table, registered in `MIGRATIONS`. |
| SQLite repo | `crates/logandaim-sqlite/src/repos/<begriff>.rs` | Row parsing and saving, using the helpers in `repos/common.rs`. |
| SQLite UoW | `crates/logandaim-sqlite/src/uow.rs` | Repository field plus accessor. |
| API schema | `crates/logandaim-api/src/schema/<begriff>.rs` | OpenAPI DTOs and `From` conversions from the domain. |
| API requests | `crates/logandaim-api/src/api/requests/<begriff>.rs` | Inbound DTOs and `From` into use-case inputs. |
| API facade | `crates/logandaim-api/src/api/mod.rs` | One thin `LogandaimAppApi` method per operation. |
| HTTP routes | `crates/logandaim-api/src/spec.rs` | The `#[oai]` route methods. |
| Status mapping | `crates/logandaim-api/src/errors/http_status.rs` | `HttpStatusMapping` impls for the new error types. |
| Shared cases | `crates/logandaim-core/tests/cases/<begriff>.rs` | Cases reused by the fake and SQLite harnesses. |
| API e2e | `crates/logandaim-api/tests/e2e/<begriff>.rs` | HTTP-level assertions including status codes. |
| Frontend contract | `frontend/src/api/logandaim-api.ts` | The interface method plus its types. |
| Frontend transport | `frontend/src/api/http-client.ts` | The `openapi-fetch` implementation. |
| Frontend hooks | `frontend/src/api/hooks/mutations.ts` | Query/mutation hooks for the operation. |
| Frontend page | `frontend/src/app/<begriff>/page.tsx` | The UI. |
| Frontend nav | `frontend/src/lib/navigation.ts` | Path, sidebar item, breadcrumb label. |

Module boundaries follow the **domain concept**, not the increment that introduced it.
`schuss.rs` keeps growing as long as new code is about Schüsse, no matter which slice it
ships in. Migrations are the exception: they are append-only and ordered by version.

## Order to work in

1. **Domain** — model the aggregate and value objects first. Validate in constructors,
   unit-test the rules. No serde, no SQL, no HTTP. Put every derived-value rule in a
   domain service on the same file so it is testable without a unit of work.
2. **Port** — write the repository trait. It takes the `Neues…` input from the domain and
   returns the aggregate; it never enforces business rules.
3. **Use case** — build the `Neues…` input, `ctx.enter()`, call the port,
   `uow.finish(result, …)`, emit one `info!` event. Orchestration only; aggregation lives
   in the domain.
4. **UoW + instrumented repo** — three small additions (accessors on `UnitOfWork`, on
   `UnitOfWorkImpl`, and on `LockedUnitOfWorkImpl`), then the wrapper and the
   `RepoStorage` field.
5. **Fakes** — run the slice against an in-memory datastore so the use cases are testable
   without SQLite.
6. **Migration + SQLite repo** — table first, then row parsing via `repos/common.rs`.
7. **API** — schema DTOs (separate from the domain), request DTOs with `From`/`TryFrom`,
   facade methods, routes, status mapping.
8. **Tests** — shared cases in `crates/logandaim-core/tests/cases/` (they run against the
   fakes *and* SQLite automatically), plus e2e for the HTTP surface.
9. **Frontend** — interface method, `HttpLogandaimApi` implementation, hooks, page,
   navigation, then `mise run sync:openapi`.

## Why `RepoStorage` has that pointer wiring

`InstrumentedUnitOfWork` wraps the adapter's unit of work and hands out instrumented
repositories. The wrappers must borrow the inner UoW, but the UoW is owned by the
wrapper — a self-referential borrow. `RepoStorage` is `Box::pin`ned so the address is
stable, then the wrappers store `*const dyn UnitOfWorkImpl` and call back through the
trait accessor on every method. That keeps the borrow rules sound without `unsafe`
leaking outside this one file.

Consequence when adding a repository: **declare the `uow` field last** in `RepoStorage`
and set the wrapper pointers after pinning, exactly like the existing code.

## Where things belong

| Concern | Home |
|---------|------|
| Invariants that must always hold | Domain constructor |
| Derived values | Domain service, not the use case |
| Requests that are malformed on arrival | API `ValidationError` |
| Cross-layer plumbing failures | `RepositoryError` from the port |
| HTTP status codes | `HttpStatusMapping` impls, never the domain |

Keep the deliberate layering: an orchestration error variant returns `None` from
`http_status()` so the nested domain error can speak (400) while repository failures stay
500/404. Do the same for a new slice — add a mapping rather than attaching a status.

## Frontend API seam

`LogandaimApi` is an interface; `HttpLogandaimApi` is one implementation and
`createLogandaimApi()` in `frontend/src/api/index.ts` is the only place that chooses it.
To run the core domain in the browser later (wasm), write a second class implementing
`LogandaimApi` and change that one line. Do not add a second provider, a loading gate, or
async config — `config` is static and carries the complete API base URL, prefix included.

## Frontend state rules

- Server state goes through TanStack Query hooks in `frontend/src/api/hooks/`.
- Component state is for user selections that are not server data (for example the IDs
  collected for the next request), not for caching responses.
- Mutations invalidate the smallest matching key prefix.
