//! Typed facade between HTTP and use cases.
//!
//! Everything here is a thin translation: request → use case → schema. No business rules.
//! Use-case errors pass through unchanged; `errors/http_status.rs` maps them to status codes.
//! Nothing here touches `poem`: the HTTP edge lives in `http/mod.rs` behind the `openapi`
//! feature, so the library compiles without it. Requests are grouped in `api/requests/`; the
//! HTTP edge owns JWT encoding and bearer verification (`http/jwt.rs`, `http/auth.rs`).
//!
//! Authorisation is never decided here. Every master-data and Runde method hands the
//! caller's [`AuthContext`] straight to the use case, which loads the aggregates and answers
//! 401/403 itself (SPEC-konto rule 17).

use std::sync::Arc;

use error_stack::{Report, ResultExt};
use logandaim_core::{
    App, ResultReport,
    application::AuthContext,
    domain::{AnlageId, ParcoursId, RundeId, ZielId, konto::BenutzerId},
    ports::{ClientKey, ThrottleAction},
    service::{
        Abmeldung, AbmeldungFehler, AnlageAnzeigen, AnlageBeschreibendeDatenÄndern,
        AnlageDeaktivieren, AnlageVerwaltungEntziehen, AnlageVerwaltungGewähren,
        AnlageVerwaltungÄndern, AnlagenAnzeigen, Anmeldung, AnmeldungFehler, BenutzerLöschen,
        BenutzerLöschenFehler, Erneuerung, ErneuerungFehler, ParcoursAnzeigen, ParcoursEntfernen,
        ParcoursHinzufügen, ParcoursSetupÄndern, ParcoursUmbenennen, PasswortVergessen,
        PasswortVergessenFehler, PasswortWiederherstellen, PasswortWiederherstellenFehler,
        RegistrierungAbschliessenFehler, RegistrierungAbschließen, RegistrierungStarten,
        RegistrierungStartenFehler, RundeAbschliessen, RundeAbschliessenFehler, RundeAnzeigen,
        RundeAnzeigenFehler, RundeStarten, RundeStartenFehler, RundenAnzeigen,
        RundenAnzeigenFehler, SchussRegistrieren, SchussRegistrierenFehler, SessionAusgestellt,
        StammdatenFehler, ZielEntfernen, ZielErsetzen, ZielHinzufügen, ZielÄndern,
    },
};
use tracing::warn;

pub mod requests;

use crate::schema::{Anlage, AnlagenZeile, ParcoursÖffentlich, Runde, RundenZeile, Schuss};

#[derive(Clone)]
pub struct LogandaimAppApi {
    app: Arc<App>,
}

impl LogandaimAppApi {
    pub fn new(app: App) -> Self {
        Self::from_arc(Arc::new(app))
    }

    /// An app the caller already holds behind an `Arc`, for a harness that must also reach
    /// the use cases directly (the e2e suite's superuser bootstrap).
    pub fn from_arc(app: Arc<App>) -> Self {
        Self { app }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }

    // ---------------------------------------------------------------- Anlage

    /// Create an Anlage. Only a Systemadministrator may (AC-1).
    pub async fn anlage_erstellen(
        &self,
        auth: AuthContext,
        anfrage: requests::AnlageErstellungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = anfrage.in_use_case()?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// The Anlagen the caller may administer.
    pub async fn anlagen_anzeigen(
        &self,
        auth: AuthContext,
    ) -> ResultReport<Vec<AnlagenZeile>, StammdatenFehler> {
        let zeilen = self.app.execute(auth, AnlagenAnzeigen).await?;
        Ok(zeilen.into_iter().map(Into::into).collect())
    }

    /// One Anlage with its master data; archived Anlagen stay readable (AC-11).
    pub async fn anlage_anzeigen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let ansicht = self.app.execute(auth, AnlageAnzeigen { anlage_id }).await?;
        Ok(ansicht.into())
    }

    pub async fn anlage_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::AnlageÄnderungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: AnlageBeschreibendeDatenÄndern = anfrage.in_use_case(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Irreversible: no reactivation exists (AC-10).
    pub async fn anlage_deaktivieren(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let anlage = self
            .app
            .execute(auth, AnlageDeaktivieren { anlage_id })
            .await?;
        Ok(anlage.into())
    }

    // ---------------------------------------------------------------- AnlageVerwaltung

    pub async fn verwaltung_gewähren(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::AnlageVerwaltungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: AnlageVerwaltungGewähren = anfrage.als_gewähren(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn verwaltung_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        benutzer_id: BenutzerId,
        anfrage: requests::AnlageVerwaltungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: AnlageVerwaltungÄndern = anfrage.als_ändern(anlage_id, benutzer_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn verwaltung_entziehen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        benutzer_id: BenutzerId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = AnlageVerwaltungEntziehen {
            anlage_id,
            benutzer_id,
        };
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    // ---------------------------------------------------------------- Ziel

    pub async fn ziel_hinzufügen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::ZielRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ZielHinzufügen = anfrage.als_hinzufügen(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Edit details; the identity and its history stay (AC-5).
    pub async fn ziel_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        ziel_id: ZielId,
        anfrage: requests::ZielRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ZielÄndern = anfrage.als_ändern(anlage_id, ziel_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Replace the physical target: a new identity, never the old history (AC-5).
    pub async fn ziel_ersetzen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        ziel_id: ZielId,
        anfrage: requests::ZielRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ZielErsetzen = anfrage.als_ersetzen(anlage_id, ziel_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Archive when history references it, delete when nothing does (AC-11, AC-12).
    pub async fn ziel_entfernen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        ziel_id: ZielId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = ZielEntfernen { anlage_id, ziel_id };
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    // ---------------------------------------------------------------- Parcours

    pub async fn parcours_hinzufügen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::ParcoursErstellungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ParcoursHinzufügen = anfrage.in_use_case(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn parcours_setup_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        parcours_id: ParcoursId,
        anfrage: requests::ParcoursSetupRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ParcoursSetupÄndern = anfrage.in_use_case(anlage_id, parcours_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn parcours_umbenennen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        parcours_id: ParcoursId,
        anfrage: requests::ParcoursUmbenennenRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ParcoursUmbenennen = anfrage.in_use_case(anlage_id, parcours_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Archive when a Runde captured it, delete when none did (AC-11, AC-12).
    pub async fn parcours_entfernen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        parcours_id: ParcoursId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = ParcoursEntfernen {
            anlage_id,
            parcours_id,
        };
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Every active Parcours of every active Anlage. Needs no identity (AC-13).
    pub async fn parcours_anzeigen(
        &self,
        auth: AuthContext,
    ) -> ResultReport<Vec<ParcoursÖffentlich>, StammdatenFehler> {
        let parcours = self.app.execute(auth, ParcoursAnzeigen).await?;
        Ok(parcours.into_iter().map(Into::into).collect())
    }

    // ---------------------------------------------------------------- Runde

    /// Start a Runde; the snapshot is captured in the same transaction (AC-7).
    pub async fn runde_starten(
        &self,
        auth: AuthContext,
        anfrage: requests::RundeStartenRequest,
    ) -> ResultReport<Runde, RundeStartenFehler> {
        let runde = self.app.execute(auth, RundeStarten::from(anfrage)).await?;
        Ok(runde.into())
    }

    /// One Runde with both Wertungen; owner or Systemadministrator (AC-9, AC-13).
    pub async fn runde_anzeigen(
        &self,
        auth: AuthContext,
        runde_id: RundeId,
    ) -> ResultReport<Runde, RundeAnzeigenFehler> {
        let ansicht = self.app.execute(auth, RundeAnzeigen { runde_id }).await?;
        Ok(ansicht.into())
    }

    /// The caller's own Runden, newest first.
    pub async fn runden_anzeigen(
        &self,
        auth: AuthContext,
    ) -> ResultReport<Vec<RundenZeile>, RundenAnzeigenFehler> {
        let runden = self.app.execute(auth, RundenAnzeigen).await?;
        Ok(runden.into_iter().map(Into::into).collect())
    }

    pub async fn runde_abschliessen(
        &self,
        auth: AuthContext,
        runde_id: RundeId,
    ) -> ResultReport<Runde, RundeAbschliessenFehler> {
        let runde = self
            .app
            .execute(auth, RundeAbschliessen { runde_id })
            .await?;
        Ok(runde.into())
    }

    // ---------------------------------------------------------------- Schuss

    /// Log one arrow. The owner comes from the token, never from the body (AC-9).
    pub async fn schuss_registrieren(
        &self,
        auth: AuthContext,
        runde_id: RundeId,
        anfrage: requests::SchussErstellungRequest,
    ) -> ResultReport<Schuss, SchussRegistrierenFehler> {
        let use_case: SchussRegistrieren = anfrage.in_use_case(runde_id)?;
        let schuss = self.app.execute(auth, use_case).await?;
        Ok(schuss.into())
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
        let jetzt = self.app.clock.now();
        self.app
            .throttle
            .check(&client_key, ThrottleAction::Erneuerung, jetzt)
            .await
            .change_context(ErneuerungFehler::Gedrosselt)?;

        let refresh_token = match logandaim_core::domain::konto::RefreshToken::try_from(
            refresh_token,
        ) {
            Ok(token) => token,
            Err(_) => {
                if let Err(fehler) = self
                    .app
                    .throttle
                    .increment(&client_key, ThrottleAction::Erneuerung, jetzt)
                    .await
                {
                    warn!(fehler = %fehler, "der Erneuerungsversuch konnte nicht gezählt werden");
                }
                return Err(Report::new(ErneuerungFehler::SessionUngültig));
            }
        };

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

    pub async fn benutzer_löschen(
        &self,
        auth: AuthContext,
        anfrage: requests::BenutzerLöschungRequest,
    ) -> ResultReport<(), BenutzerLöschenFehler> {
        let use_case: BenutzerLöschen = anfrage.in_use_case()?;
        self.app.execute(auth, use_case).await
    }
}
