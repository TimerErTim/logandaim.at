//! The HTTP edge: one thin `#[OpenApi]` wrapper over [`LogandaimAppApi`].
//!
//! Everything HTTP-specific lives here and only here — response envelopes with their
//! headers, the cookie, the client key — so the facade in `api/` stays free of `poem` and
//! the library compiles without the `openapi` feature. Handlers take native `poem_openapi`
//! extractors, never a raw request.
//!
//! Three things every handler enforces and a caller cannot opt out of:
//!
//! - **The refresh token never appears in a body.** It is written to the `__Host-` cookie
//!   by the response builder; the access token travels in the body only (AC-7a).
//! - **Every auth response is `Cache-Control: no-store`.**
//! - **Protected handlers answer 401 instead of running with `Anonymous`.**

use error_stack::{IntoReport, Report, ResultExt};
use logandaim_core::{
    App, ResultReport,
    application::AuthContext,
    domain::{AnlageId, AnlageZielId, ParkourId, RundeId, RundenZielId, konto::BenutzerId},
    service::{
        AbmeldungFehler, BenutzerLöschenFehler, PasswortWiederherstellenFehler, StammdatenFehler,
    },
};
use poem::web::RealIp;
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject, Tags,
    param::{Cookie, Header, Path, Query},
    payload::Json,
};
use std::sync::{Arc, RwLock, atomic::AtomicBool};
use uuid::Uuid;

mod auth;
mod cookie;
mod extractors;
mod health;
pub mod jwt;
mod responses;
use extractors::*;
pub use responses::*;

use crate::{
    api::{
        LogandaimAppApi,
        requests::{
            AnlageErstellungRequest, AnlageVerwaltungRequest, AnlageÄnderungRequest,
            AnmeldungRequest, BenutzerLöschungRequest, FreieRundeStartenRequest,
            ParkourErstellungRequest, ParkourSetupRequest, ParkourUmbenennenRequest,
            PasswortVergessenRequest, PasswortWiederherstellenRequest,
            RegistrierungAbschlussRequest, RegistrierungStartRequest,
            RundeMitParkourStartenRequest, SchussErstellungRequest, SchussÄnderungRequest,
            ZielHinzufuegenRequest, ZielRequest, ZielUmbenennenRequest,
        },
    },
    schema::{
        Anlage, AnlageZugriff, AnlageÖffentlich, AuthenticatedBenutzer, Benutzer, MailHealth, Ok,
        ParkourÖffentlich, Runde, RundeBasic, Schuss,
    },
};

/// The sections the published API is grouped into.
#[derive(Tags)]
pub enum ApiTag {
    /// Anlage master data: identity, address, contact, roles, Ziele, and the public
    /// directory.
    Anlagen,
    /// Parkour: Pflock setup, route order, and the distances between them.
    Parkour,
    /// Runden and the Schüsse logged in them.
    Runden,
    /// Registration, sign-in, session, password recovery, and the own Benutzer.
    Konto,
    /// Liveness of the service.
    Betrieb,
}

pub struct LogandaimApiSpec {
    app_api: LogandaimAppApi,
    // Kept out of `App` so no use case can branch on the relay.
    mail_health: Arc<RwLock<MailHealth>>,
    mail_probe_running: Arc<AtomicBool>,
}

impl LogandaimApiSpec {
    pub fn new(app_api: LogandaimAppApi) -> Self {
        Self {
            app_api,
            mail_health: Arc::default(),
            mail_probe_running: Arc::default(),
        }
    }
}

impl From<LogandaimAppApi> for LogandaimApiSpec {
    fn from(app_api: LogandaimAppApi) -> Self {
        Self::new(app_api)
    }
}

impl From<App> for LogandaimApiSpec {
    fn from(app: App) -> Self {
        Self::new(LogandaimAppApi::new(app))
    }
}

#[OpenApi]
impl LogandaimApiSpec {
    /// Report that the service is up.
    ///
    /// Returns `OK` when the service is running.
    #[oai(path = "/health", method = "get", tag = "ApiTag::Betrieb")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    /// Report the latest result from the mail relay check.
    ///
    /// The response describes the latest probe result, or `Unknown` before the first probe.
    #[oai(path = "/health/mail", method = "get", tag = "ApiTag::Betrieb")]
    async fn health_mail(&self) -> Json<MailHealth> {
        Json(self.mail_health())
    }

    // ------------------------------------------------------------------ master data

    /// Create an Anlage. Systemadministrator only.
    ///
    /// The caller becomes its first Administrator.
    #[oai(path = "/anlagen", method = "post", tag = "ApiTag::Anlagen")]
    async fn anlage_erstellen(
        &self,
        authorization: RequiredBearerAuthorization,
        body: Json<AnlageErstellungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = self.app_api.anlage_erstellen(auth, body.0).await;
        ergebnis.into()
    }

    /// List the Anlagen the caller may administer.
    ///
    /// Each row includes the caller's role in that Anlage.
    #[oai(path = "/anlagen", method = "get", tag = "ApiTag::Anlagen")]
    async fn anlagen_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> TypicalJsonResponse<Vec<AnlageZugriff>> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = self.app_api.anlagen_anzeigen(auth).await;
        ergebnis.into()
    }

    /// List the active Anlagen with the Parkour each offers. No sign-in needed.
    ///
    /// This public directory contains no contact data.
    //
    // The static segment wins over `/anlagen/:anlage_id`, so registration order is safe.
    #[oai(path = "/anlagen/oeffentlich", method = "get", tag = "ApiTag::Anlagen")]
    async fn anlagen_verzeichnis(
        &self,
        #[oai(name = "Authorization")] authorization: BearerAuthorization,
    ) -> TypicalJsonResponse<Vec<AnlageÖffentlich>> {
        // A broken token is answered as anonymous here, never as an error: this route is
        // public, and a stale token must not hide the directory from its own reader.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(logandaim_core::application::AuthContext::Anonymous);
        self.app_api.anlagen_verzeichnis(auth).await.into()
    }

    /// Read one Anlage with its whole master data.
    ///
    /// Only a Systemadministrator can access an archived Anlage.
    #[oai(path = "/anlagen/:anlage_id", method = "get", tag = "ApiTag::Anlagen")]
    async fn anlage_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .anlage_anzeigen(auth, AnlageId(anlage_id.0))
                .await
        };
        ergebnis.into()
    }

    /// Rewrite the Anlage's own fields: name, address, and contact. Administrator only.
    ///
    /// The whole set is replaced; omitted contact fields become empty.
    #[oai(path = "/anlagen/:anlage_id", method = "put", tag = "ApiTag::Anlagen")]
    async fn anlage_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        body: Json<AnlageÄnderungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .anlage_ändern(auth, AnlageId(anlage_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Deactivate an Anlage once and for all.
    ///
    /// An archived Anlage remains available for reading but cannot be changed.
    #[oai(
        path = "/anlagen/:anlage_id/deaktivierung",
        method = "post",
        tag = "ApiTag::Anlagen"
    )]
    async fn anlage_deaktivieren(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .anlage_deaktivieren(auth, AnlageId(anlage_id.0))
                .await
        };
        ergebnis.into()
    }

    /// Grant a role in the Anlage to an existing Benutzer. Administrator only.
    ///
    /// The Benutzer is named by identity; find it with `benutzer-suche` first.
    #[oai(
        path = "/anlagen/:anlage_id/verwaltungen",
        method = "post",
        tag = "ApiTag::Anlagen"
    )]
    async fn verwaltung_gewähren(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        body: Json<AnlageVerwaltungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .verwaltung_gewähren(auth, AnlageId(anlage_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Find a Benutzer by exact e-mail address, for granting a role.
    ///
    /// The result contains the matching Benutzer, if one exists.
    #[oai(
        path = "/anlagen/:anlage_id/benutzer-suche",
        method = "get",
        tag = "ApiTag::Anlagen"
    )]
    async fn benutzer_nach_email_suchen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        e_mail_adresse: Query<String>,
    ) -> TypicalJsonResponse<Vec<Benutzer>> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .benutzer_nach_email_suchen(auth, AnlageId(anlage_id.0), e_mail_adresse.0)
                .await
        };
        ergebnis.into()
    }

    /// Change an existing role. Administrator only.
    ///
    /// The target is in the path, so the body only names the role.
    #[oai(
        path = "/anlagen/:anlage_id/verwaltungen/:benutzer_id",
        method = "put",
        tag = "ApiTag::Anlagen"
    )]
    async fn verwaltung_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        benutzer_id: Path<Uuid>,
        body: Json<AnlageVerwaltungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .verwaltung_ändern(
                    auth,
                    AnlageId(anlage_id.0),
                    BenutzerId(benutzer_id.0),
                    body.0,
                )
                .await
        };
        ergebnis.into()
    }

    /// Revoke a role. Administrator only.
    ///
    /// The last Administrator of an Anlage cannot be revoked.
    #[oai(
        path = "/anlagen/:anlage_id/verwaltungen/:benutzer_id",
        method = "delete",
        tag = "ApiTag::Anlagen"
    )]
    async fn verwaltung_entziehen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        benutzer_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .verwaltung_entziehen(auth, AnlageId(anlage_id.0), BenutzerId(benutzer_id.0))
                .await
        };
        ergebnis.into()
    }

    /// Add a physical Ziel to the Anlage.
    ///
    /// The new Ziel is added to the Anlage's target list.
    #[oai(
        path = "/anlagen/:anlage_id/ziele",
        method = "post",
        tag = "ApiTag::Anlagen"
    )]
    async fn ziel_hinzufügen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        body: Json<ZielRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .ziel_hinzufügen(auth, AnlageId(anlage_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Edit a Ziel's details.
    ///
    /// The identity stays, and so does everything already recorded against it.
    #[oai(
        path = "/anlagen/:anlage_id/ziele/:ziel_id",
        method = "put",
        tag = "ApiTag::Anlagen"
    )]
    async fn ziel_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        ziel_id: Path<Uuid>,
        body: Json<ZielRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .ziel_ändern(auth, AnlageId(anlage_id.0), AnlageZielId(ziel_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Replace the physical Ziel with a new one in the same slot.
    ///
    /// The replacement has a new identity; the history of the old Ziel stays as recorded.
    #[oai(
        path = "/anlagen/:anlage_id/ziele/:ziel_id/ersatz",
        method = "post",
        tag = "ApiTag::Anlagen"
    )]
    async fn ziel_ersetzen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        ziel_id: Path<Uuid>,
        body: Json<ZielRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .ziel_ersetzen(auth, AnlageId(anlage_id.0), AnlageZielId(ziel_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Remove a Ziel: archive it when recorded Runden reference it, delete it otherwise.
    ///
    /// Answers the Anlage as it stands afterwards, so the Ziel is absent from `ziele` either
    /// way and an archived one no longer appears among the active ones.
    #[oai(
        path = "/anlagen/:anlage_id/ziele/:ziel_id",
        method = "delete",
        tag = "ApiTag::Anlagen"
    )]
    async fn ziel_entfernen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        ziel_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .ziel_entfernen(auth, AnlageId(anlage_id.0), AnlageZielId(ziel_id.0))
                .await
        };
        ergebnis.into()
    }

    /// Create a Parkour with its Pflock setup and its route.
    ///
    /// Provide the Pflöcke in the order in which they should appear.
    #[oai(
        path = "/anlagen/:anlage_id/parkour",
        method = "post",
        tag = "ApiTag::Parkour"
    )]
    async fn parkour_hinzufügen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        body: Json<ParkourErstellungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .parkour_hinzufügen(auth, AnlageId(anlage_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Replace the whole shooting setup: Pflöcke and the route that places Ziele among them.
    ///
    /// Existing Runden keep the snapshot they captured; only future Runden see the change.
    #[oai(
        path = "/anlagen/:anlage_id/parkour/:parkour_id/setup",
        method = "put",
        tag = "ApiTag::Parkour"
    )]
    async fn parkour_setup_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        parkour_id: Path<Uuid>,
        body: Json<ParkourSetupRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .parkour_setup_ändern(auth, AnlageId(anlage_id.0), ParkourId(parkour_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Rename a Parkour.
    ///
    /// The identity stays, and every captured snapshot keeps the name it recorded.
    #[oai(
        path = "/anlagen/:anlage_id/parkour/:parkour_id",
        method = "put",
        tag = "ApiTag::Parkour"
    )]
    async fn parkour_umbenennen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        parkour_id: Path<Uuid>,
        body: Json<ParkourUmbenennenRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .parkour_umbenennen(auth, AnlageId(anlage_id.0), ParkourId(parkour_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Remove a Parkour: archive it when a Runde captured it, delete it otherwise.
    ///
    /// Answers the Anlage as it stands afterwards.
    #[oai(
        path = "/anlagen/:anlage_id/parkour/:parkour_id",
        method = "delete",
        tag = "ApiTag::Parkour"
    )]
    async fn parkour_entfernen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        parkour_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Anlage> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .parkour_entfernen(auth, AnlageId(anlage_id.0), ParkourId(parkour_id.0))
                .await
        };
        ergebnis.into()
    }

    /// List every active Parkour of every active Anlage. No sign-in needed.
    ///
    /// Use this list to choose a Parkour when starting a round.
    #[oai(path = "/parkour", method = "get", tag = "ApiTag::Parkour")]
    async fn parkour_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: BearerAuthorization,
    ) -> TypicalJsonResponse<Vec<ParkourÖffentlich>> {
        // A broken token is answered as anonymous here, never as an error: this route is
        // public, and a stale token must not hide master data from its own reader.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(logandaim_core::application::AuthContext::Anonymous);
        self.app_api.parkour_anzeigen(auth).await.into()
    }

    // ------------------------------------------------------------------ Runden

    /// Start a Runde on one active Parkour.
    ///
    /// Later changes to the Parkour do not change the setup captured by this Runde.
    #[oai(path = "/runden/mit-parkour", method = "post", tag = "ApiTag::Runden")]
    async fn runde_starten(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        body: Json<RundeMitParkourStartenRequest>,
    ) -> TypicalJsonResponse<Runde> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = self.app_api.runde_mit_parkour_starten(auth, body.0).await;
        ergebnis.into()
    }

    /// List the caller's own Runden, newest first.
    ///
    /// Only the owner can see these Runden.
    #[oai(path = "/runden", method = "get", tag = "ApiTag::Runden")]
    async fn runden_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> TypicalJsonResponse<Vec<RundeBasic>> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = self.app_api.runden_anzeigen(auth).await;
        ergebnis.into()
    }

    /// Start a `Freie Runde`: no Anlage, no Parkour, and its own name.
    ///
    /// The response identifies this round as having no Anlage or Parkour.
    #[oai(path = "/runden/frei", method = "post", tag = "ApiTag::Runden")]
    async fn runde_starten_frei(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        body: Json<FreieRundeStartenRequest>,
    ) -> TypicalJsonResponse<Runde> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        self.app_api.freie_runde_starten(auth, body.0).await.into()
    }

    /// Append one `Freies Ziel` to the caller's own open `Freie Runde`.
    ///
    /// Targets in a Parkour round cannot be changed.
    #[oai(
        path = "/runden/:runde_id/ziele",
        method = "post",
        tag = "ApiTag::Runden"
    )]
    async fn runde_ziel_hinzufuegen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
        body: Json<ZielHinzufuegenRequest>,
    ) -> TypicalJsonResponse<Runde> {
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        self.app_api
            .runde_ziel_hinzufuegen(auth, RundeId(runde_id.0), body.0)
            .await
            .into()
    }

    /// Rename one `Freies Ziel` in a running free Runde.
    ///
    /// A name containing only whitespace is invalid. Only an open free round can be changed.
    #[oai(
        path = "/runden/:runde_id/ziele/:ziel_id",
        method = "put",
        tag = "ApiTag::Runden"
    )]
    async fn runde_ziel_umbenennen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
        ziel_id: Path<Uuid>,
        body: Json<ZielUmbenennenRequest>,
    ) -> TypicalJsonResponse<Runde> {
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        self.app_api
            .runde_ziel_umbenennen(auth, RundeId(runde_id.0), RundenZielId(ziel_id.0), body.0)
            .await
            .into()
    }

    /// Remove one `Freies Ziel` from a free Runde.
    ///
    /// Removing a target also removes its recorded shots. The response contains the updated round.
    #[oai(
        path = "/runden/:runde_id/ziele/:ziel_id",
        method = "delete",
        tag = "ApiTag::Runden"
    )]
    async fn runde_ziel_entfernen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
        ziel_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Runde> {
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        self.app_api
            .runde_ziel_entfernen(auth, RundeId(runde_id.0), RundenZielId(ziel_id.0))
            .await
            .into()
    }

    /// Read one Runde with its captured route and both Wertungen.
    ///
    /// The Runde is visible to its owner and to a Systemadministrator.
    #[oai(path = "/runden/:runde_id", method = "get", tag = "ApiTag::Runden")]
    async fn runde_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Runde> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = self.app_api.runde_anzeigen(auth, RundeId(runde_id.0)).await;
        ergebnis.into()
    }

    /// Delete the caller's Runde and every Schuss that belongs to it.
    ///
    /// This removes the Runde from history and clears its per-Ziel shot records.
    #[oai(path = "/runden/:runde_id", method = "delete", tag = "ApiTag::Runden")]
    async fn runde_loeschen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
    ) -> TypicalJsonResponse<crate::schema::Ok> {
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        self.app_api
            .runde_loeschen(auth, RundeId(runde_id.0))
            .await
            .map(|()| crate::schema::Ok { ok: true })
            .into()
    }

    /// Close the caller's own Runde.
    ///
    /// Once closed, the Runde no longer accepts Schüsse.
    #[oai(
        path = "/runden/:runde_id/abschluss",
        method = "post",
        tag = "ApiTag::Runden"
    )]
    async fn runde_abschliessen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Runde> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .runde_abschliessen(auth, RundeId(runde_id.0))
                .await
        };
        ergebnis.into()
    }

    /// Log one arrow at a Ziel of the caller's own open Runde.
    ///
    /// The target must belong to the round.
    #[oai(
        path = "/runden/:runde_id/schuesse",
        method = "post",
        tag = "ApiTag::Runden"
    )]
    async fn schuss_registrieren(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
        body: Json<SchussErstellungRequest>,
    ) -> TypicalJsonResponse<Schuss> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis = {
            self.app_api
                .schuss_registrieren(auth, RundeId(runde_id.0), body.0)
                .await
        };
        ergebnis.into()
    }

    /// Replace the scoring zone and position of one of the caller's shots.
    ///
    /// Sending `position: null` clears the saved position.
    #[oai(
        path = "/runden/:runde_id/schuesse/:schuss_id",
        method = "put",
        tag = "ApiTag::Runden"
    )]
    async fn schuss_aendern(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
        schuss_id: Path<Uuid>,
        body: Json<SchussÄnderungRequest>,
    ) -> TypicalJsonResponse<Schuss> {
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        self.app_api
            .schuss_aendern(
                auth,
                RundeId(runde_id.0),
                logandaim_core::domain::SchussId(schuss_id.0),
                body.0,
            )
            .await
            .into()
    }

    /// Delete one of the caller's shots and renumber later shots at the same Ziel.
    ///
    /// The scoring total is recomputed from the remaining shots.
    #[oai(
        path = "/runden/:runde_id/schuesse/:schuss_id",
        method = "delete",
        tag = "ApiTag::Runden"
    )]
    async fn schuss_loeschen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
        schuss_id: Path<Uuid>,
    ) -> TypicalJsonResponse<crate::schema::Ok> {
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        self.app_api
            .schuss_loeschen(
                auth,
                RundeId(runde_id.0),
                logandaim_core::domain::SchussId(schuss_id.0),
            )
            .await
            .map(|()| crate::schema::Ok { ok: true })
            .into()
    }

    // ------------------------------------------------------------------ konto

    /// Registration step 1: request a confirmation mail for an e-mail address.
    ///
    /// Returns the same response whether the address is free or already registered.
    #[oai(path = "/registrierung/start", method = "post", tag = "ApiTag::Konto")]
    async fn registrierung_starten(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<RegistrierungStartRequest>,
    ) -> UncachedJsonResponse<Ok> {
        let client_key = client_key(
            real_ip,
            user_agent.0.as_deref(),
            accept_language.0.as_deref(),
        );
        UncachedJsonResponse::from(
            self.app_api
                .registrierung_starten(body.0, client_key)
                .await
                .map(|()| Ok { ok: true }),
        )
    }

    /// Complete registration with the token from the confirmation email.
    ///
    /// Returns the new session credentials.
    #[oai(
        path = "/registrierung/abschluss",
        method = "post",
        tag = "ApiTag::Konto"
    )]
    async fn registrierung_abschließen(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<RegistrierungAbschlussRequest>,
    ) -> NewSessionResponse {
        let client_key = client_key(
            real_ip,
            user_agent.0.as_deref(),
            accept_language.0.as_deref(),
        );
        NewSessionResponse::from(
            self.app_api
                .registrierung_abschließen(body.0, client_key)
                .await
                .map(|ausgestellt| {
                    jwt::encode_session(ausgestellt, self.app_api.inner_app().signing.as_ref())
                        .expect("the JOSE header serialises")
                }),
        )
    }

    /// Sign in with e-mail address and password.
    ///
    /// Returns the new session credentials.
    #[oai(path = "/anmeldung", method = "post", tag = "ApiTag::Konto")]
    async fn anmelden(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<AnmeldungRequest>,
    ) -> NewSessionResponse {
        let client_key = client_key(
            real_ip,
            user_agent.0.as_deref(),
            accept_language.0.as_deref(),
        );
        NewSessionResponse::from(self.app_api.anmelden(body.0, client_key).await.map(
            |ausgestellt| {
                jwt::encode_session(ausgestellt, self.app_api.inner_app().signing.as_ref())
                    .expect("the JOSE header serialises")
            },
        ))
    }

    /// Renew the current session.
    ///
    /// Returns a new session when the refresh token is valid.
    #[oai(path = "/session/erneuern", method = "post", tag = "ApiTag::Konto")]
    async fn session_erneuern(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        #[oai(name = "__Host-logandaim_refresh")] refresh: Cookie<Option<String>>,
    ) -> NewSessionResponse {
        let client_key = client_key(
            real_ip,
            user_agent.0.as_deref(),
            accept_language.0.as_deref(),
        );
        let ergebnis = self
            .app_api
            .session_erneuern(refresh.0.unwrap_or_default(), client_key)
            .await;
        NewSessionResponse::from(ergebnis.map(|ausgestellt| {
            jwt::encode_session(ausgestellt, self.app_api.inner_app().signing.as_ref())
                .expect("the JOSE header serialises")
        }))
    }

    /// End the current session.
    ///
    /// The current session is ended.
    #[oai(path = "/session/abmelden", method = "post", tag = "ApiTag::Konto")]
    async fn abmelden(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> EndSessionResponse<Ok> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis: ResultReport<(), AbmeldungFehler> = self.app_api.abmelden(auth).await;
        EndSessionResponse::from(ergebnis.map(|()| Ok { ok: true }))
    }

    /// Request a password reset mail for an e-mail address.
    ///
    /// Returns the same response whether the address belongs to an account or not.
    #[oai(path = "/passwort/vergessen", method = "post", tag = "ApiTag::Konto")]
    async fn passwort_vergessen(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<PasswortVergessenRequest>,
    ) -> UncachedJsonResponse<Ok> {
        let client_key = client_key(
            real_ip,
            user_agent.0.as_deref(),
            accept_language.0.as_deref(),
        );
        UncachedJsonResponse::from(
            self.app_api
                .passwort_vergessen(body.0, client_key)
                .await
                .map(|()| Ok { ok: true }),
        )
    }

    /// Set a new password from a mailed reset token.
    ///
    /// The reset token can be used once. All existing sessions end when the password changes.
    #[oai(
        path = "/passwort/wiederherstellen",
        method = "post",
        tag = "ApiTag::Konto"
    )]
    async fn passwort_wiederherstellen(
        &self,
        body: Json<PasswortWiederherstellenRequest>,
    ) -> EndSessionResponse<Ok> {
        let ergebnis: ResultReport<(), PasswortWiederherstellenFehler> =
            self.app_api.passwort_wiederherstellen(body.0).await;
        EndSessionResponse::from(ergebnis.map(|()| Ok { ok: true }))
    }

    /// Read the authenticated caller, including its account-level role.
    ///
    /// Requires a valid sign-in.
    #[oai(path = "/benutzer", method = "get", tag = "ApiTag::Konto")]
    async fn benutzer_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> UncachedJsonResponse<AuthenticatedBenutzer> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis: ResultReport<AuthenticatedBenutzer, StammdatenFehler> = self
            .app_api
            .inner_app()
            .execute_fn(auth, async |ctx| {
                let benutzer_id = ctx
                    .auth()
                    .require_benutzer_id()
                    .change_context(StammdatenFehler::NichtAngemeldet)?;
                let uow = ctx
                    .enter()
                    .await
                    .change_context(StammdatenFehler::BenutzerLaden)?;
                let ergebnis = async {
                    let benutzer = uow
                        .benutzer()
                        .find_by_id(benutzer_id)
                        .await
                        .map_err(IntoReport::into_report)
                        .change_context(StammdatenFehler::BenutzerLaden)?;
                    let konten = uow
                        .konten()
                        .find_by_benutzer(benutzer_id)
                        .await
                        .map_err(IntoReport::into_report)
                        .change_context(StammdatenFehler::BenutzerLaden)?;
                    let Some(konto) = konten.into_iter().next() else {
                        return Err(Report::new(StammdatenFehler::BenutzerLaden));
                    };
                    Ok((benutzer.into_data(), konto.email().clone()))
                }
                .await;
                let (benutzer, email) = uow
                    .finish(ergebnis, StammdatenFehler::BenutzerLaden)
                    .await?;
                Ok((benutzer, email).into())
            })
            .await;
        UncachedJsonResponse::from(ergebnis)
    }

    /// Delete the caller's own Benutzer.
    ///
    /// The caller confirms the deletion by re-entering their password.
    #[oai(path = "/benutzer/löschung", method = "post", tag = "ApiTag::Konto")]
    async fn benutzer_löschen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        body: Json<BenutzerLöschungRequest>,
    ) -> EndSessionResponse<Ok> {
        // A missing or unusable token is anonymous here; the use case answers 401.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(AuthContext::Anonymous);
        let ergebnis: ResultReport<(), BenutzerLöschenFehler> =
            self.app_api.benutzer_löschen(auth, body.0).await;
        EndSessionResponse::from(ergebnis.map(|()| Ok { ok: true }))
    }
}

pub fn openapi_service(
    app: App,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<LogandaimApiSpec, ()> {
    openapi_service_from_api(LogandaimAppApi::new(app), server_urls)
}

/// The service: the spec, its servers, and a description of how to call it.
pub fn openapi_service_from_api(
    app_api: LogandaimAppApi,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<LogandaimApiSpec, ()> {
    let mut service = OpenApiService::new(
        LogandaimApiSpec::from(app_api),
        "logandaim.at API",
        env!("CARGO_PKG_VERSION"),
    )
    .summary("Logbook for shooting ranges: master data, Runden, and the Wertungen.")
    .description(include_str!("README.md"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
