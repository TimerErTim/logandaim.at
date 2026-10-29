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
    domain::{AnlageId, ParcoursId, RundeId, ZielId, konto::BenutzerId},
    service::{
        AbmeldungFehler, BenutzerLöschenFehler, PasswortWiederherstellenFehler,
        RundeAbschliessenFehler, RundeAnzeigenFehler, RundeStartenFehler, RundenAnzeigenFehler,
        SchussRegistrierenFehler, StammdatenFehler,
    },
};
use poem::web::RealIp;
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject,
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
            AnmeldungRequest, BenutzerLöschungRequest, ParcoursErstellungRequest,
            ParcoursSetupRequest, ParcoursUmbenennenRequest, PasswortVergessenRequest,
            PasswortWiederherstellenRequest, RegistrierungAbschlussRequest,
            RegistrierungStartRequest, RundeStartenRequest, SchussErstellungRequest, ZielRequest,
        },
    },
    schema::{
        Anlage, AnlagenZeile, AnlageÖffentlich, Benutzer, Ok, ParcoursÖffentlich, Runde,
        RundenZeile, Schuss,
    },
};

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
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    // ------------------------------------------------------------------ master data

    /// Only a Systemadministrator may create an Anlage (AC-1).
    #[oai(path = "/anlagen", method = "post")]
    async fn anlage_erstellen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        body: Json<AnlageErstellungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.anlage_erstellen(auth, body.0).await,
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// The Anlagen the caller may administer. Archived ones are not listed.
    #[oai(path = "/anlagen", method = "get")]
    async fn anlagen_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
    ) -> TypicalJsonResponse<Vec<AnlagenZeile>> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.anlagen_anzeigen(auth).await,
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Public: the active Anlagen and the Parcours each offers. No header needed.
    //
    // The static segment wins over `/anlagen/:anlage_id`, so registration order is safe.
    #[oai(path = "/anlagen/oeffentlich", method = "get")]
    async fn anlagen_verzeichnis(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
    ) -> TypicalJsonResponse<Vec<AnlageÖffentlich>> {
        // A broken token is answered as anonymous here, never as an error: this route is
        // public, and a stale token must not hide the directory from its own reader.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(logandaim_core::application::AuthContext::Anonymous);
        self.app_api.anlagen_verzeichnis(auth).await.into()
    }

    /// An archived Anlage answers only to a Systemadministrator; its writes still 409 (AC-11).
    #[oai(path = "/anlagen/:anlage_id", method = "get")]
    async fn anlage_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Rewrites the Anlage's own fields (name, address, contact). Administrator only (AC-3).
    #[oai(path = "/anlagen/:anlage_id", method = "put")]
    async fn anlage_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Irreversible: no route reactivates an Anlage (AC-10).
    #[oai(path = "/anlagen/:anlage_id/deaktivierung", method = "post")]
    async fn anlage_deaktivieren(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Grant a role to an existing Benutzer. Administrator only (AC-2).
    #[oai(path = "/anlagen/:anlage_id/verwaltungen", method = "post")]
    async fn verwaltung_gewähren(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Exact-email lookup used only by a caller who may manage this active Anlage.
    #[oai(path = "/anlagen/:anlage_id/benutzer-suche", method = "get")]
    async fn benutzer_nach_email_suchen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        anlage_id: Path<Uuid>,
        e_mail_adresse: Query<String>,
    ) -> TypicalJsonResponse<Vec<crate::schema::BenutzerSuchtreffer>> {
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

    /// Change an existing role. The target is in the path, so the body only names the role.
    #[oai(path = "/anlagen/:anlage_id/verwaltungen/:benutzer_id", method = "put")]
    async fn verwaltung_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Revoke a role. The last Administrator cannot be revoked (AC-2).
    #[oai(
        path = "/anlagen/:anlage_id/verwaltungen/:benutzer_id",
        method = "delete"
    )]
    async fn verwaltung_entziehen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Add a physical Ziel (AC-5). A Mitarbeiter may; the Anlage fields are not touched.
    #[oai(path = "/anlagen/:anlage_id/ziele", method = "post")]
    async fn ziel_hinzufügen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Edit details; the identity and its history stay (AC-5).
    #[oai(path = "/anlagen/:anlage_id/ziele/:ziel_id", method = "put")]
    async fn ziel_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Replace the physical target: a new identity that inherits the slot, not the history.
    #[oai(path = "/anlagen/:anlage_id/ziele/:ziel_id/ersatz", method = "post")]
    async fn ziel_ersetzen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Archive when history references it, delete when nothing does (AC-11, AC-12).
    #[oai(path = "/anlagen/:anlage_id/ziele/:ziel_id", method = "delete")]
    async fn ziel_entfernen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Create a Parcours with its Pflock setup and route (AC-4, AC-6).
    #[oai(path = "/anlagen/:anlage_id/parcours", method = "post")]
    async fn parcours_hinzufügen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        anlage_id: Path<Uuid>,
        body: Json<ParcoursErstellungRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parcours_hinzufügen(auth, AnlageId(anlage_id.0), body.0)
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Replace the whole shooting setup; existing Runden keep their snapshot (AC-7).
    #[oai(
        path = "/anlagen/:anlage_id/parcours/:parcours_id/setup",
        method = "put"
    )]
    async fn parcours_setup_ändern(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        anlage_id: Path<Uuid>,
        parcours_id: Path<Uuid>,
        body: Json<ParcoursSetupRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parcours_setup_ändern(
                        auth,
                        AnlageId(anlage_id.0),
                        ParcoursId(parcours_id.0),
                        body.0,
                    )
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Rename a Parcours. Its identity and every existing snapshot stay (AC-9).
    #[oai(path = "/anlagen/:anlage_id/parcours/:parcours_id", method = "put")]
    async fn parcours_umbenennen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        anlage_id: Path<Uuid>,
        parcours_id: Path<Uuid>,
        body: Json<ParcoursUmbenennenRequest>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parcours_umbenennen(
                        auth,
                        AnlageId(anlage_id.0),
                        ParcoursId(parcours_id.0),
                        body.0,
                    )
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Archive when a Runde captured it, delete when none did (AC-11, AC-12).
    #[oai(path = "/anlagen/:anlage_id/parcours/:parcours_id", method = "delete")]
    async fn parcours_entfernen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        anlage_id: Path<Uuid>,
        parcours_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Anlage> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => {
                self.app_api
                    .parcours_entfernen(auth, AnlageId(anlage_id.0), ParcoursId(parcours_id.0))
                    .await
            }
            Err(_) => Err(Report::new(StammdatenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Public: every active Parcours of every active Anlage, no header needed (AC-13).
    #[oai(path = "/parcours", method = "get")]
    async fn parcours_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
    ) -> TypicalJsonResponse<Vec<ParcoursÖffentlich>> {
        // A broken token is answered as anonymous here, never as an error: this route is
        // public, and a stale token must not hide master data from its own reader.
        let auth = self
            .auth_context(authorization.0.as_deref())
            .await
            .unwrap_or(logandaim_core::application::AuthContext::Anonymous);
        self.app_api.parcours_anzeigen(auth).await.into()
    }

    // ------------------------------------------------------------------ Runden

    /// Start a Runde. The snapshot is captured in the same transaction (AC-7).
    #[oai(path = "/runden", method = "post")]
    async fn runde_starten(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        body: Json<RundeStartenRequest>,
    ) -> TypicalJsonResponse<Runde> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.runde_starten(auth, body.0).await,
            Err(_) => Err(Report::new(RundeStartenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// The caller's own Runden, newest first.
    #[oai(path = "/runden", method = "get")]
    async fn runden_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
    ) -> TypicalJsonResponse<Vec<RundenZeile>> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.runden_anzeigen(auth).await,
            Err(_) => Err(Report::new(RundenAnzeigenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Owner or Systemadministrator; staff never read another Benutzer's Runde (AC-13).
    #[oai(path = "/runden/:runde_id", method = "get")]
    async fn runde_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        runde_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Runde> {
        let ergebnis = match self.auth_context(authorization.0.as_deref()).await {
            Ok(auth) => self.app_api.runde_anzeigen(auth, RundeId(runde_id.0)).await,
            Err(_) => Err(Report::new(RundeAnzeigenFehler::NichtAngemeldet)),
        };
        ergebnis.into()
    }

    /// Close the caller's own Runde; afterwards it refuses further Schüsse.
    #[oai(path = "/runden/:runde_id/abschluss", method = "post")]
    async fn runde_abschliessen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Log one arrow. The owner comes from the token; the Ziel must be in the snapshot.
    #[oai(path = "/runden/:runde_id/schüsse", method = "post")]
    async fn schuss_registrieren(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

    /// Step 1. Answers `200` whether the address is free or taken (AC-2).
    #[oai(path = "/registrierung/start", method = "post")]
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

    /// Step 2: creates Benutzer + Konto and answers the first token pair.
    #[oai(path = "/registrierung/abschluss", method = "post")]
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

    #[oai(path = "/anmeldung", method = "post")]
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

    /// Rotate the refresh token from the cookie.
    #[oai(path = "/session/erneuern", method = "post")]
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

    #[oai(path = "/session/abmelden", method = "post")]
    async fn abmelden(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
    ) -> EndSessionResponse<Ok> {
        let ergebnis: ResultReport<(), AbmeldungFehler> =
            match self.auth_context(authorization.0.as_deref()).await {
                Ok(auth) => self.app_api.abmelden(auth).await,
                Err(_) => Err(Report::new(AbmeldungFehler::NichtAngemeldet)),
            };
        EndSessionResponse::from(ergebnis.map(|()| Ok { ok: true }))
    }

    #[oai(path = "/passwort/vergessen", method = "post")]
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

    /// Single use, and ends every session: the cookie is cleared with them.
    #[oai(path = "/passwort/wiederherstellen", method = "post")]
    async fn passwort_wiederherstellen(
        &self,
        body: Json<PasswortWiederherstellenRequest>,
    ) -> EndSessionResponse<Ok> {
        let ergebnis: ResultReport<(), PasswortWiederherstellenFehler> =
            self.app_api.passwort_wiederherstellen(body.0).await;
        EndSessionResponse::from(ergebnis.map(|()| Ok { ok: true }))
    }

    /// The authenticated Benutzer, including its account-level role.
    #[oai(path = "/benutzer", method = "get")]
    async fn benutzer_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
    ) -> UncachedJsonResponse<Benutzer> {
        let ergebnis: ResultReport<Benutzer, StammdatenFehler> =
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

    /// Deletes the caller's own Benutzer. The cookie is cleared with the identity.
    #[oai(path = "/benutzer/löschung", method = "post")]
    async fn benutzer_löschen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
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

/// The same service over a facade the caller already holds, so a harness can keep an
/// `Arc<App>` for direct use-case calls without building a second app.
pub fn openapi_service_from_api(
    app_api: LogandaimAppApi,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<LogandaimApiSpec, ()> {
    let mut service = OpenApiService::new(
        LogandaimApiSpec::from(app_api),
        "logandaim.at API",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
