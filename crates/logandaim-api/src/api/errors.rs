//! Refusals of the reads that run inline in [`LogandaimAppApi`](super::LogandaimAppApi).
//!
//! A read is not a use case, so its error enum lives beside the read rather than in
//! `logandaim-core`. Each variant names what the read could not do and keeps the underlying
//! repository error in the `error_stack` chain; `errors/http_status.rs` gives it its status.

// ---------------------------------------------------------------------------------------
// RundenAnzeigen error
// ---------------------------------------------------------------------------------------

/// The refusals of the caller's own Runden list, which the API facade reads inline.
#[derive(Debug, thiserror::Error)]
pub enum RundenAnzeigenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("die Runden konnten nicht geladen werden")]
    Laden,
}

// ---------------------------------------------------------------------------------------
// RundeAnzeigen error
// ---------------------------------------------------------------------------------------

/// The refusals of one Runde's detail read, which the API facade reads inline.
#[derive(Debug, thiserror::Error)]
pub enum RundeAnzeigenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde darf nicht eingesehen werden")]
    NichtErlaubt,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
    #[error("die Schüsse konnten nicht geladen werden")]
    SchüsseLaden,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
}
