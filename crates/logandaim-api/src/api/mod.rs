//! Typed facade between HTTP and use cases.
//!
//! Everything here is a thin translation: request → use case → schema. No business rules.
//! Use-case errors pass through unchanged; `HttpStatusMapping` in `errors/http_status.rs`
//! turns them into status codes. Nothing here touches `poem`: the HTTP edge lives in
//! `spec.rs` behind the `openapi` feature, so the library compiles without it.
//!
//! Two responsibilities are genuinely this layer's:
//!
//! - **The access token's envelope.** Core signs bytes; this facade serialises the claims,
//!   base64url-encodes them, and asks core for the signature (`auth::jwt`).
//! - **Turning a bearer header into an [`AuthContext`].** A token that verifies is then
//!   handed to `SessionPrüfen`, which is what makes a *revoked* session fail even though
//!   its signature is still valid (AC-17).

use std::sync::Arc;

use error_stack::Report;
use logandaim_core::{
    App, ResultReport,
    application::AuthContext,
    ports::ClientKey,
    service::{
        Abmeldung, AbmeldungFehler, Anmeldung, AnmeldungFehler, BenutzerAnzeigen,
        BenutzerAnzeigenFehler, BenutzerLöschen, BenutzerLöschenFehler, Erneuerung,
        ErneuerungFehler, PasswortVergessen, PasswortVergessenFehler, PasswortWiederherstellen,
        PasswortWiederherstellenFehler, RegistrierungAbschliessenFehler, RegistrierungAbschließen,
        RegistrierungStarten, RegistrierungStartenFehler, SchussRegistrieren,
        SchussRegistrierenFehler, SessionAusgestellt, WertungBerechnen, WertungFehler,
    },
};

pub mod requests;

use crate::schema::{
    Benutzer, Schuss, Wertung, schema_schuss_from_domain, schema_wertung_from_domain,
};

#[derive(Clone)]
pub struct LogandaimAppApi {
    app: Arc<App>,
}

impl LogandaimAppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }

    // ---------------------------------------------------------------- Schuss slice

    pub async fn schuss_registrieren(
        &self,
        erstellung: requests::SchussErstellungRequest,
    ) -> ResultReport<Schuss, SchussRegistrierenFehler> {
        let schuss = self
            .app
            .execute(AuthContext::Anonymous, SchussRegistrieren::from(erstellung))
            .await?;
        Ok(schema_schuss_from_domain(schuss))
    }

    pub async fn wertung_berechnen(
        &self,
        anfrage: requests::WertungRequest,
    ) -> ResultReport<Wertung, WertungFehler> {
        let ergebnis = self
            .app
            .execute(AuthContext::Anonymous, WertungBerechnen::from(anfrage))
            .await?;
        Ok(schema_wertung_from_domain(ergebnis))
    }

    // ---------------------------------------------------------------- Registration

    /// Step 1. Always answers `Ok`: a taken address is not disclosed (AC-2).
    pub async fn registrierung_starten(
        &self,
        anfrage: requests::RegistrierungStartRequest,
        client_key: ClientKey,
    ) -> ResultReport<(), RegistrierungStartenFehler> {
        let use_case: RegistrierungStarten = anfrage.in_use_case(client_key)?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    /// Step 2. Creates the Benutzer, its Konto, and the first session.
    pub async fn registrierung_abschließen(
        &self,
        anfrage: requests::RegistrierungAbschlussRequest,
        client_key: ClientKey,
    ) -> ResultReport<SessionAusgestellt, RegistrierungAbschliessenFehler> {
        let use_case: RegistrierungAbschließen = anfrage.in_use_case(client_key)?;
        let ausgestellt = self.app.execute(AuthContext::Anonymous, use_case).await?;
        Ok(ausgestellt)
    }

    // ---------------------------------------------------------------- Login

    pub async fn anmelden(
        &self,
        anfrage: requests::AnmeldungRequest,
        client_key: ClientKey,
    ) -> ResultReport<SessionAusgestellt, AnmeldungFehler> {
        let use_case: Anmeldung = anfrage.in_use_case(client_key)?;
        let ausgestellt = self.app.execute(AuthContext::Anonymous, use_case).await?;
        Ok(ausgestellt)
    }

    // TODO: serializable dto for session ausgestellt (as per api standard)
    /// Rotate the refresh token. The presented one is spent; a replay ends the family.
    pub async fn session_erneuern(
        &self,
        refresh_token: String,
        client_key: ClientKey,
    ) -> ResultReport<SessionAusgestellt, ErneuerungFehler> {
        let refresh_token = logandaim_core::domain::konto::RefreshToken::try_from(refresh_token)
            .map_err(|_| Report::new(ErneuerungFehler::SessionUngültig))?;

        let ausgestellt = self
            .app
            .execute(
                AuthContext::Anonymous,
                Erneuerung {
                    refresh_token,
                    client_key,
                },
            )
            .await?;
        Ok(ausgestellt)
    }

    pub async fn abmelden(&self, auth: AuthContext) -> ResultReport<(), AbmeldungFehler> {
        self.app.execute(auth, Abmeldung).await
    }

    // ---------------------------------------------------------------- Passwort

    pub async fn passwort_vergessen(
        &self,
        anfrage: requests::PasswortVergessenRequest,
        client_key: ClientKey,
    ) -> ResultReport<(), PasswortVergessenFehler> {
        let use_case: PasswortVergessen = anfrage.in_use_case(client_key)?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    /// Single use, and ends every session of the Benutzer.
    pub async fn passwort_wiederherstellen(
        &self,
        anfrage: requests::PasswortWiederherstellenRequest,
    ) -> ResultReport<(), PasswortWiederherstellenFehler> {
        let use_case: PasswortWiederherstellen = anfrage.in_use_case()?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    // ---------------------------------------------------------------- Benutzer

    /// The caller's own Benutzer, or another one when a Systemadministrator asks.
    pub async fn benutzer_anzeigen(
        &self,
        auth: AuthContext,
        benutzer_id: Option<uuid::Uuid>,
    ) -> ResultReport<Benutzer, BenutzerAnzeigenFehler> {
        let ansicht = self
            .app
            .execute(
                auth,
                BenutzerAnzeigen {
                    benutzer_id: benutzer_id.map(logandaim_core::domain::konto::BenutzerId),
                },
            )
            .await?;

        Ok((ansicht.benutzer, ansicht.email).into())
    }

    pub async fn benutzer_löschen(
        &self,
        auth: AuthContext,
        anfrage: requests::BenutzerLöschungRequest,
    ) -> ResultReport<(), BenutzerLöschenFehler> {
        let use_case: BenutzerLöschen = anfrage.in_use_case()?;
        self.app.execute(auth, use_case).await
    }
}
