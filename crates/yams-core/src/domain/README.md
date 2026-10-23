# Domain model

This is the single source of truth for **how** domain types are modeled. **What** they model comes
from the `PO-approved` specs in `specs/product/`.

Keep these domain-agnostic rules when replacing domain types. Examples use placeholder names
(`Aggregat`, `Wert`, …); real types use the German ubiquitous language of the specs.

## Layout

- One file per aggregate or value-object family, named after the German domain term
  (`ziel.rs`, `wertung.rs`), never after a technical role (`model.rs`, `types.rs`).
- `mod.rs` declares the modules and re-exports the public types (`pub use ziel::Ziel;`), so callers
  write `crate::domain::Ziel`.
- The domain depends on nothing outside `crate` except value libraries (`uuid`, `chrono`,
  `rust_decimal`, `thiserror`, `error_stack`). No serde, OpenAPI, SQL, or HTTP types.
- Domain types and services may use ports from `application::ports` (for example `Clock`)
  directly.

## Make invalid state unrepresentable

Prefer types over runtime checks. If a value can be invalid, it must be impossible to construct
without validation.

### Identity newtypes

Every aggregate and child entity has its own ID newtype. IDs cannot be mixed up at compile time.

```rust,ignore
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AggregatId(pub Uuid);
```

### Validated value objects

- Tuple struct with a **private** field. The only ways in are `new()` and `TryFrom`, which validate.
- `new` returns a plain `Result<Self, WertFehler>` with a dedicated `thiserror` error, not a
  `Report`. Construction has no cross-cutting failure.
- Read access via `value()` or `AsRef`. Named constants as constructors (`zero()`, `one()`).
- Encode units and ranges in the type: a ratio is `0..=1` (100 % = `1`), so a 20 % rate is `0.20`,
  never `20`. Never divide by 100 in business code.
- Implement `std::ops` when the result cannot leave the valid range (sum of two non-negative
  amounts, amount × non-negative quantity). Such operators are infallible. Operations that can
  leave the range are named methods that return `Result`.

```rust,ignore
#[derive(Debug, thiserror::Error)]
#[error("Wert muss zwischen 0 und 1 liegen: {0}")]
pub struct WertFehler(pub Decimal);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wert(Decimal);

impl Wert {
    pub fn new(value: Decimal) -> Result<Self, WertFehler> { /* range check */ }
    pub fn value(&self) -> Decimal { self.0 }
}
```

### Closed enums

Use an `enum` when the set of values is closed (country codes, scoring zones, hit/miss). Never a
free-form `String` or integer code.

### Variant data in enums

When fields only make sense together, model the alternatives as enum variants that carry their
fields (`Quelle::Produkt { menge, einzelpreis }` vs `Quelle::Manuell { preis }`). No
`Option` fields that are only valid in combination.

### Type-state aggregates

An aggregate with a lifecycle is a generic struct over a state marker. Each state is its own type,
so a transition is only callable in the state where it is legal.

```rust,ignore
pub struct Offen;                                   // state without data: unit struct
pub struct Abgeschlossen { abgeschlossen_am: DateTime<Utc> } // state data lives in the marker

pub struct AggregatIn<S> { id: AggregatId, /* shared fields */ state: S }
pub type AggregatOffen = AggregatIn<Offen>;
pub type AggregatAbgeschlossen = AggregatIn<Abgeschlossen>;

/// Sum of all valid states, for code that does not know the state at compile time.
pub enum Aggregat {
    Offen(AggregatOffen),
    Abgeschlossen(AggregatAbgeschlossen),
}
```

- Shared accessors on `impl<S> AggregatIn<S>`. State-only accessors and transitions on the alias
  (`impl AggregatOffen`).
- Transitions consume `self` and return the target state (`fn abschließen(self, …) ->
  AggregatAbgeschlossen`). A transition with preconditions returns `ResultReport<Target, Fehler>`.
- Mutations within one state take `&mut self` and check invariants before changing fields.
- The sum enum implements `From<AggregatIn<S>>` for every state and delegates shared accessors with
  a `match`.
- A lifecycle without behavior differences between states may use a plain status enum on the
  entity instead.

### Creation input and construction

- `Neues*` / `Neuer*` / `Neue*` (German gender of the term) is the creation input: all fields of
  the aggregate except the ID and server-managed state. It is a plain record with
  `neu(...)` and accessors. It holds validated value objects but does **not** check aggregate
  invariants.
- `Aggregat::neu(id, NeuesAggregat) -> ResultReport<Self, AggregatFehler>` is the only place
  that builds a valid aggregate and checks its invariants (non-empty names, consistent ranges). An
  aggregate without invariants may return `Self`.
- `Aggregat::from_parts(id, …raw fields, state parts)` rebuilds a loaded aggregate. It
  re-validates value objects from primitives and routes through `neu` and the transitions, so a
  loaded aggregate obeys the same invariants as a new one. For type-state aggregates, pass the
  state data as an `*ZustandTeile` enum or `Option`.
- Lifecycle: a use case builds `NeuesAggregat` and passes it to the repository `create`. The
  repository assigns the ID and calls `neu`. On load, the repository calls `from_parts`. Nothing
  else assembles an aggregate. API request DTOs never do.

### Encapsulation

- Aggregate and entity fields are private. Expose read-only accessors (`&T` for owned data, `T`
  for `Copy`). No setters.
- Change state only through named domain methods in the ubiquitous language
  (`buchung_anlegen`, `absagen`). Child entities are changed only through their aggregate root.
- Plain data value objects without invariants (an address) may have `pub` fields.

### Derived values

Compute derived values in getters (`fn betrag(&self) -> Preis`). Never store a value that can be
computed from other fields, because stored duplicates drift.

### Snapshots

When a record must keep the value valid at the time of the event (a booked price, a scoring rule
version), copy that value into the record. Do not look it up from mutable reference data later.

### References between aggregates

Reference other aggregates by ID only. Store each link once, on the owning side (child holds
`parent_id`). No back-reference lists on the parent.

## Errors

- One `thiserror` enum per aggregate, named `<Aggregat>Fehler`. Variants name the violated rule
  (`KapazitätErreicht`, `NameLeer`). Messages are German UL.
- Aggregate constructors, transitions, and domain services return
  `ResultReport<T, <Aggregat>Fehler>` (`error_stack::Report`). Raise with `Report::new(...)`.
  Wrap value-object errors with `.change_context(<Aggregat>Fehler::Konstruktion)`.
- Constructors attach a module constant `const CONSTRUCTING: &str = "while constructing
  <aggregat>";` via `.attach(CONSTRUCTING)`, so the report shows the failing site.
- Value objects are the exception: plain `Result<T, WertFehler>` (see above).
- The domain does not know HTTP status codes. `yams-api` maps error variants to statuses.

## Tests

Unit tests live in a `#[cfg(test)] mod tests` at the bottom of the same file and use
`#[test_log::test]`. Cover every invariant, both value-object range edges, every transition, and
the `from_parts` round trip. Assert on the error variant with `err.current_context()` and on the
`CONSTRUCTING` attachment. Use-case flows belong in `yams-core/tests/cases/`, not here.
