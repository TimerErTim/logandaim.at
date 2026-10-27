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

use error_stack::Report;
use logandaim_core::{
    App, ResultReport,
    service::{
        AbmeldungFehler, BenutzerAnzeigenFehler, BenutzerLöschenFehler, ErneuerungFehler,
        PasswortWiederherstellenFehler,
    },
};
use poem::web::RealIp;
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject,
    param::{Cookie, Header, Query},
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
            AnmeldungRequest, BenutzerLöschungRequest, PasswortVergessenRequest,
            PasswortWiederherstellenRequest, RegistrierungAbschlussRequest,
            RegistrierungStartRequest, SchussErstellungRequest, WertungRequest,
        },
    },
    schema::{Benutzer, Ok, Schuss, Wertung},
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

    #[oai(path = "/schuss", method = "post")]
    async fn schuss_registrieren(
        &self,
        body: Json<SchussErstellungRequest>,
    ) -> TypicalJsonResponse<Schuss> {
        self.app_api.schuss_registrieren(body.0).await.into()
    }

    #[oai(path = "/wertung", method = "post")]
    async fn wertung_berechnen(&self, body: Json<WertungRequest>) -> TypicalJsonResponse<Wertung> {
        self.app_api.wertung_berechnen(body.0).await.into()
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
        let ergebnis = match refresh.0.filter(|token| !token.is_empty()) {
            Some(token) => self.app_api.session_erneuern(token, client_key).await,
            None => Err(Report::new(ErneuerungFehler::SessionUngültig)),
        };
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

    /// The caller's own Benutzer. `?benutzerId=` names another one and needs
    /// `Systemadministrator`.
    #[oai(path = "/benutzer", method = "get")]
    async fn benutzer_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        #[oai(name = "benutzerId")] benutzer_id: Query<Option<Uuid>>,
    ) -> UncachedJsonResponse<Benutzer> {
        let ergebnis: ResultReport<Benutzer, BenutzerAnzeigenFehler> =
            match self.auth_context(authorization.0.as_deref()).await {
                Ok(auth) => self.app_api.benutzer_anzeigen(auth, benutzer_id.0).await,
                Err(_) => Err(Report::new(BenutzerAnzeigenFehler::NichtAngemeldet)),
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
    let mut service = OpenApiService::new(
        LogandaimApiSpec::from(app),
        "logandaim.at API",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
