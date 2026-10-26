//! The `konto` route handlers.
//!
//! Free functions taking `&LogandaimApiSpec`, not an `#[OpenApi]` impl: the crate has one
//! `#[OpenApi]` impl per module and this module must not produce a second one for the same
//! type. The handlers still live here, next to each other, and `spec.rs` only wires them.
//!
//! Three things every handler enforces and a caller cannot opt out of:
//!
//! - **The refresh token never appears in a body.** It is written to the `__Host-` cookie
//!   by the response builder; the access token travels in the body only (AC-7a).
//! - **Every auth response is `Cache-Control: no-store`.** A shared cache that stores an
//!   access token hands it to the next caller.
//! - **Protected handlers answer 401 instead of running with `Anonymous`.** The identity
//!   comes from `require_authenticated`, and each use case checks it again.

use logandaim_core::ResultReport;
use logandaim_core::service::KontoDienstFehler;
use poem::Request;
use poem_openapi::payload::Json;

use crate::{
    LogandaimApiSpec,
    api::{
        requests,
        session::{BeendendeAntwort, KontoAntwort, SitzungAntwort},
    },
    schema::{BenutzerAntwort, OkAntwort},
};

/// An edge-level refusal (401) expressed in the same error type the use cases answer with,
/// so one response enum covers both layers without a second DTO.
fn nicht_angemeldet<T>() -> ResultReport<T, KontoDienstFehler> {
    Err(error_stack::Report::new(KontoDienstFehler::NichtAngemeldet))
}

pub async fn registrierung_starten(
    spec: &LogandaimApiSpec,
    req: &Request,
    body: Json<requests::RegistrierungStartAnfrage>,
) -> KontoAntwort<OkAntwort> {
    let kunde = spec.api().kunde(req).await;
    KontoAntwort::from(
        spec.api()
            .registrierung_starten(body.0, kunde)
            .await
            .map(|()| OkAntwort { ok: true }),
    )
}

pub async fn registrierung_abschließen(
    spec: &LogandaimApiSpec,
    req: &Request,
    body: Json<requests::RegistrierungAbschlussAnfrage>,
) -> SitzungAntwort {
    let kunde = spec.api().kunde(req).await;
    SitzungAntwort::neu(spec.api().registrierung_abschließen(body.0, kunde).await)
}

pub async fn anmelden(
    spec: &LogandaimApiSpec,
    req: &Request,
    body: Json<requests::AnmeldeAnfrage>,
) -> SitzungAntwort {
    let kunde = spec.api().kunde(req).await;
    SitzungAntwort::neu(spec.api().anmelden(body.0, kunde).await)
}

pub async fn erneuerung(spec: &LogandaimApiSpec, req: &Request) -> SitzungAntwort {
    let kunde = spec.api().kunde(req).await;
    let ergebnis = match refresh_aus_cookie(req) {
        Some(token) => spec.api().erneuerung(token, kunde).await,
        None => nicht_angemeldet(),
    };
    SitzungAntwort::neu(ergebnis)
}

pub async fn abmelden(spec: &LogandaimApiSpec, req: &Request) -> BeendendeAntwort<OkAntwort> {
    let ergebnis: ResultReport<(), KontoDienstFehler> =
        match spec.api().require_authenticated(req).await {
            Ok(auth) => spec.api().abmelden(auth).await,
            Err(_) => nicht_angemeldet(),
        };
    BeendendeAntwort::neu(ergebnis.map(|()| OkAntwort { ok: true }))
}

pub async fn passwort_vergessen(
    spec: &LogandaimApiSpec,
    req: &Request,
    body: Json<requests::PasswortVergessenAnfrage>,
) -> KontoAntwort<OkAntwort> {
    let kunde = spec.api().kunde(req).await;
    KontoAntwort::from(
        spec.api()
            .passwort_vergessen(body.0, kunde)
            .await
            .map(|()| OkAntwort { ok: true }),
    )
}

pub async fn passwort_wiederherstellen(
    spec: &LogandaimApiSpec,
    body: Json<requests::PasswortWiederherstellenAnfrage>,
) -> BeendendeAntwort<OkAntwort> {
    let ergebnis = spec.api().passwort_wiederherstellen(body.0).await;
    BeendendeAntwort::neu(ergebnis.map(|()| OkAntwort { ok: true }))
}

pub async fn benutzer_anzeigen(
    spec: &LogandaimApiSpec,
    req: &Request,
    benutzer_id: Option<uuid::Uuid>,
) -> KontoAntwort<BenutzerAntwort> {
    let ergebnis: ResultReport<BenutzerAntwort, KontoDienstFehler> =
        match spec.api().require_authenticated(req).await {
            Ok(auth) => spec.api().benutzer_anzeigen(auth, benutzer_id).await,
            Err(_) => nicht_angemeldet(),
        };
    KontoAntwort::from(ergebnis)
}

pub async fn benutzer_löschen(
    spec: &LogandaimApiSpec,
    req: &Request,
    body: Json<requests::BenutzerLöschungAnfrage>,
) -> BeendendeAntwort<OkAntwort> {
    let ergebnis: ResultReport<(), KontoDienstFehler> =
        match spec.api().require_authenticated(req).await {
            Ok(auth) => spec.api().benutzer_löschen(auth, body.0).await,
            Err(_) => nicht_angemeldet(),
        };
    BeendendeAntwort::neu(ergebnis.map(|()| OkAntwort { ok: true }))
}

/// The refresh token from the request cookie, if any.
pub fn refresh_aus_cookie(req: &Request) -> Option<String> {
    let kopf = req.headers().get("cookie")?.to_str().ok()?;
    let prefix = format!("{}=", crate::auth::cookie::REFRESH_COOKIE);
    for teil in kopf.split(';') {
        let teil = teil.trim();
        if let Some(wert) = teil.strip_prefix(&prefix) {
            return (!wert.is_empty()).then(|| wert.to_owned());
        }
    }
    None
}
