// Shared test cases for every `base_app_builder` harness.
//
// Each feature slice adds one module here. The same cases then run against the fakes
// (`business_conform.rs` in core) and against SQLite (`business_conform.rs` in
// `logandaim-sqlite`), which is how adapter conformance is proven.

mod schuss;
