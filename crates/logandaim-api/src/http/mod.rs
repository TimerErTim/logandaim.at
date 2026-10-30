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
    domain::{AnlageId, ParkourId, RundeId, ZielId, konto::BenutzerId},
    service::{
        AbmeldungFehler, BenutzerLöschenFehler, PasswortWiederherstellenFehler,
        RundeAbschliessenFehler, RundeAnzeigenFehler, RundeStartenFehler, RundenAnzeigenFehler,
        SchussRegistrierenFehler, StammdatenFehler,
    },
};
use poem::web::RealIp;
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject, Tags,
    param::{Cookie, Header, Path, Query},
    payload::Json,
};
use uuid::Uuid;

mod auth;
mod cookie;
mod extractors;
pub mod jwt;
mod responses;
use extractors::*;
pub use responses::*;

use crate::{
    api::{
        LogandaimAppApi,
        requests::{
            AnlageErstellungRequest, AnlageVerwaltungRequest, AnlageÄnderungRequest,
            AnmeldungRequest, BenutzerLöschungRequest, ParkourErstellungRequest,
            ParkourSetupRequest, ParkourUmbenennenRequest, PasswortVergessenRequest,
            PasswortWiederherstellenRequest, RegistrierungAbschlussRequest,
            RegistrierungStartRequest, RundeStartenRequest, SchussErstellungRequest, ZielRequest,
        },
    },
    schema::{
        Anlage, AnlageZugriff, AnlageÖffentlich, AuthenticatedBenutzer, Benutzer, Ok,
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
}

impl LogandaimApiSpec {
    pub fn new(app_api: LogandaimAppApi) -> Self {
        Self { app_api }
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
    /// Answers the literal `OK`. It touches no dependency, so a green answer means the
    /// process is listening, not that its database is reachable.
    #[oai(path = "/health", method = "get", tag = "ApiTag::Betrieb")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    // ------------------------------------------------------------------ master data

    /// Create an Anlage. Systemadministrator only.
    ///
    /// The caller becomes its first Administrator. The address is required, the contact
    /// details are optional.
    #[oai(path = "/anlagen", method = "post", tag = "ApiTag::Anlagen")]
    async fn anlage_erstellen(
        &self,
        authorization: RequiredBearerAuthorization,
        body: Json<AnlageErstellungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.anlage_erstellen(auth, body.0).await,
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// List the Anlagen the caller may administer.
    ///
    /// Archived Anlagen are not listed. Each row carries the caller's role in that Anlage
    /// so a client can gate its controls.
    #[oai(path = "/anlagen", method = "get", tag = "ApiTag::Anlagen")]
    async fn anlagen_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> TypicalJsonResponse<Vec<AnlageZugriff>> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.anlagen_anzeigen(auth).await,
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// List the active Anlagen with the Parkour each offers. No sign-in needed.
    ///
    /// This is the public directory. It carries no contact data, and a broken token is
    /// answered as anonymous rather than as an error.
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
    /// An archived Anlage answers only to a Systemadministrator; every write to one is
    /// refused with `409`.
    #[oai(path = "/anlagen/:anlage_id", method = "get", tag = "ApiTag::Anlagen")]
    async fn anlage_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .anlage_anzeigen(auth, AnlageId(anlage_id.0))
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Rewrite the Anlage's own fields: name, address, and contact. Administrator only.
    ///
    /// The whole set is replaced; omitted contact fields become empty. Ziele, Parkour, and
    /// role assignments are untouched.
    #[oai(path = "/anlagen/:anlage_id", method = "put", tag = "ApiTag::Anlagen")]
    async fn anlage_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        anlage_id: Path<Uuid>,
        body: Json<AnlageÄnderungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .anlage_ändern(auth, AnlageId(anlage_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Deactivate an Anlage once and for all.
    ///
    /// Irreversible: no route reactivates one. Reads keep working, writes answer `409`.
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .anlage_deaktivieren(auth, AnlageId(anlage_id.0))
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .verwaltung_gewähren(auth, AnlageId(anlage_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Find a Benutzer by exact e-mail address, for granting a role.
    ///
    /// Usable only by a caller who may manage this active Anlage. Answers a single match or
    /// an empty list.
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .benutzer_nach_email_suchen(auth, AnlageId(anlage_id.0), e_mail_adresse.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .verwaltung_ändern(
                        auth,
                        AnlageId(anlage_id.0),
                        BenutzerId(benutzer_id.0),
                        body.0,
                    )
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .verwaltung_entziehen(auth, AnlageId(anlage_id.0), BenutzerId(benutzer_id.0))
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Add a physical Ziel to the Anlage.
    ///
    /// A Mitarbeiter may do this; the Anlage's own fields are not touched.
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .ziel_hinzufügen(auth, AnlageId(anlage_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .ziel_ändern(auth, AnlageId(anlage_id.0), ZielId(ziel_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .ziel_ersetzen(auth, AnlageId(anlage_id.0), ZielId(ziel_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .ziel_entfernen(auth, AnlageId(anlage_id.0), ZielId(ziel_id.0))
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Create a Parkour with its Pflock setup and its route.
    ///
    /// The Pflöcke and their order are given explicitly; a colour never implies an order.
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parkour_hinzufügen(auth, AnlageId(anlage_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parkour_setup_ändern(
                        auth,
                        AnlageId(anlage_id.0),
                        ParkourId(parkour_id.0),
                        body.0,
                    )
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parkour_umbenennen(
                        auth,
                        AnlageId(anlage_id.0),
                        ParkourId(parkour_id.0),
                        body.0,
                    )
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parkour_entfernen(auth, AnlageId(anlage_id.0), ParkourId(parkour_id.0))
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// List every active Parkour of every active Anlage. No sign-in needed.
    ///
    /// This is the list a client starts a Runde from.
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
    /// The setup is captured as a snapshot in the same transaction, so a later change to the
    /// Parkour cannot alter this Runde.
    #[oai(path = "/runden", method = "post", tag = "ApiTag::Runden")]
    async fn runde_starten(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        body: Json<RundeStartenRequest>,
    ) -> TypicalJsonResponse<Runde> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.runde_starten(auth, body.0).await,
            Err(_) => Err(Report::new(RundeStartenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// List the caller's own Runden, newest first.
    ///
    /// No one but the caller and a Systemadministrator ever sees a Runde here.
    #[oai(path = "/runden", method = "get", tag = "ApiTag::Runden")]
    async fn runden_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> TypicalJsonResponse<Vec<RundeBasic>> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.runden_anzeigen(auth).await,
            Err(_) => Err(Report::new(RundenAnzeigenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Read one Runde with its whole snapshot and both Wertungen.
    ///
    /// The owner and a Systemadministrator may read it; staff may not read another
    /// Benutzer's Runde.
    #[oai(path = "/runden/:runde_id", method = "get", tag = "ApiTag::Runden")]
    async fn runde_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Runde> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.runde_anzeigen(auth, RundeId(runde_id.0)).await,
            Err(_) => Err(Report::new(RundeAnzeigenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Close the caller's own Runde.
    ///
    /// Afterwards it refuses further Schüsse and its Wertungen are final.
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
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .runde_abschliessen(auth, RundeId(runde_id.0))
                    .await
            }
            Err(_) => Err(Report::new(RundeAbschliessenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Log one arrow at a Ziel of the caller's own open Runde.
    ///
    /// The owner comes from the token, and the Ziel must be part of the Runde's snapshot.
    #[oai(
        path = "/runden/:runde_id/schüsse",
        method = "post",
        tag = "ApiTag::Runden"
    )]
    async fn schuss_registrieren(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        runde_id: Path<Uuid>,
        body: Json<SchussErstellungRequest>,
    ) -> TypicalJsonResponse<Schuss> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .schuss_registrieren(auth, RundeId(runde_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(SchussRegistrierenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    // ------------------------------------------------------------------ konto

    /// Registration step 1: request a confirmation mail for an e-mail address.
    ///
    /// Answers `200` whether the address is free or already taken, so the route cannot be
    /// used to probe for accounts.
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

    /// Registration step 2: create the Benutzer and its Konto from the mailed token.
    ///
    /// Answers the first token pair; the refresh token leaves as the `__Host-` cookie.
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
    /// Answers an access token in the body and a refresh token in the `__Host-` cookie.
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

    /// Rotate the refresh token from the `__Host-` cookie.
    ///
    /// Answers a fresh token pair. A `401` clears the unusable cookie.
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
    /// The refresh token is revoked and the cookie is cleared with it.
    #[oai(path = "/session/abmelden", method = "post", tag = "ApiTag::Konto")]
    async fn abmelden(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> EndSessionResponse<Ok> {
        let ergebnis: ResultReport<(), AbmeldungFehler> =
            match self.auth_context(authorization.0.as_deref()).await {
                Ok(auth) => self.app_api.abmelden(auth).await,
                Err(_) => Err(Report::new(AbmeldungFehler::NichtAngemeldet)),
            };
        EndSessionResponse::from(ergebnis.map(|()| Ok { ok: true }))
    }

    /// Request a password reset mail for an e-mail address.
    ///
    /// Answers `200` whether the address exists or not, so the route cannot be used to probe
    /// for accounts.
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
    /// The token is single use and ends every session; the cookie is cleared with them.
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
    /// Answers `401` when the token is missing or unusable; there is no anonymous answer.
    #[oai(path = "/benutzer", method = "get", tag = "ApiTag::Konto")]
    async fn benutzer_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
    ) -> UncachedJsonResponse<AuthenticatedBenutzer> {
        let ergebnis: ResultReport<AuthenticatedBenutzer, StammdatenFehler> =
            match self.auth_context(authorization.0.as_deref()).await {
                Ok(auth) => {
                    self.app_api
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
                        .await
                }
                Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
            };
        UncachedJsonResponse::from(ergebnis)
    }

    /// Delete the caller's own Benutzer.
    ///
    /// The password is re-entered and the intent must be confirmed explicitly. The cookie
    /// is cleared with the identity.
    #[oai(path = "/benutzer/löschung", method = "post", tag = "ApiTag::Konto")]
    async fn benutzer_löschen(
        &self,
        #[oai(name = "Authorization")] authorization: RequiredBearerAuthorization,
        body: Json<BenutzerLöschungRequest>,
    ) -> EndSessionResponse<Ok> {
        let ergebnis: ResultReport<(), BenutzerLöschenFehler> =
            match self.auth_context(authorization.0.as_deref()).await {
                Ok(auth) => self.app_api.benutzer_löschen(auth, body.0).await,
                Err(_) => Err(Report::new(BenutzerLöschenFehler::NichtAngemeldet)),
            };
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
    .description(concat!(
        "## Sections\n",
        "\n",
        "`Anlagen` holds master data: an Anlage with its address, contact details, role\n",
        "assignments, Ziele, and Parkour, plus the public directory. `Parkour` covers the\n",
        "shooting setup itself: Pflöcke, their order, and the distances to the Ziele placed\n",
        "among them. `Runden` covers shooting a Parkour and logging Schüsse. `Konto` covers\n",
        "registration, sign-in, sessions, password recovery, and the caller's own Benutzer.\n",
        "\n",
        "## Authentication\n",
        "\n",
        "A protected route takes an access token as `Authorization: Bearer <token>`. Sign-in,\n",
        "registration step 2, and session renewal hand that token back in the response body\n",
        "and place the refresh token in the `__Host-logandaim_refresh` cookie. Every session\n",
        "response is `Cache-Control: no-store`; the refresh token never appears in a body.\n",
        "Routes that accept a token also work without one, as an anonymous caller, and then\n",
        "usually answer `401`.\n",
        "\n",
        "## Errors\n",
        "\n",
        "Every failed request answers the same envelope: `message` says what failed,\n",
        "`attachments` carries the free-text detail attached to it, and `sources` nests the\n",
        "causes beneath it. A `500` answers a constant body and never reveals its cause. The\n",
        "HTTP status carries the meaning: `400` for an invalid request, `401` for a missing or\n",
        "unusable token, `403` for a caller that may not act, `404` for what is not there, and\n",
        "`409` for a write an archived object refuses.\n",
    ));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
