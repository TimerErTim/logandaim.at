//! The permission model, shared by every use case and by the API facade's own reads.
//!
//! Authorisation is always decided from the `Benutzer` and the `Anlage` a caller loaded
//! (NFR-2, SPEC-konto rule 17): no table, no middleware, no role string handed in by an
//! endpoint. The predicates below are the whole model, so a read the facade runs inline
//! and a write a use case runs answer the same question the same way.
//!
//! They answer a plain `bool` and name no use case's error: each caller already loaded the
//! aggregates and decides itself which refusal its own vocabulary has for "no". Nothing here
//! loads, returns a loaded aggregate, or unwraps a report.

use crate::domain::{Anlage, AnlageVerwaltungRolle, konto::Benutzer};

/// Whether a Benutzer may maintain this Anlage's Parkour, Ziele, and Pflöcke (rule 10).
pub fn darf_masterdaten_pflegen(benutzer: &Benutzer, anlage: &Anlage) -> bool {
    benutzer.ist_systemadministrator()
        || anlage
            .rolle_von(benutzer.id())
            .is_some_and(AnlageVerwaltungRolle::darf_masterdaten_pflegen)
}

/// Whether a Benutzer may edit the Anlage's own fields, deactivate it, or manage its
/// `AnlageVerwaltungen` (rule 2, rule 10).
pub fn darf_anlage_verwalten(benutzer: &Benutzer, anlage: &Anlage) -> bool {
    benutzer.ist_systemadministrator()
        || anlage
            .rolle_von(benutzer.id())
            .is_some_and(AnlageVerwaltungRolle::darf_anlage_verwalten)
}

/// Whether a Benutzer may read this Anlage at all.
///
/// An archived Anlage is closed to everyone but a `Systemadministrator`: its history stays
/// reachable through the Runden, not through an admin surface (rule 11, AC-11).
pub fn darf_anlage_lesen(benutzer: &Benutzer, anlage: &Anlage) -> bool {
    !anlage.ist_archiviert() || benutzer.ist_systemadministrator()
}
