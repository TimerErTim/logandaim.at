//! The AC-22 promise as a compiler check.
//!
//! "A Parkour round's route is captured, never edited" is a statement about which methods
//! exist, and only a failing compilation can prove an absence. The fixtures in
//! `tests/compile_fail/` are compiled against the real crate; each must fail for the reason
//! recorded in its `.stderr`, so a route mutator appearing on `RundeIn<MitParkour>` breaks
//! this test instead of silently widening the aggregate.

#[test]
fn ac22_die_typestates_tragen_ihre_mutatoren_allein() {
    trybuild::TestCases::new().compile_fail("tests/compile_fail/*.rs");
}
