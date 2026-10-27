//! Shared setup for the e2e suite.
//!
//! One app per test, on SQLite in a temp dir, with the real domain crypto and the fakes
//! only for the two ports a test must inspect (mail) or move (clock). Nothing here stubs a
//! hasher or a signer: a test that passes against a fake algorithm would prove nothing
//! about the product.
//!
//! The suite talks to Mailpit for AC-19 only, and fails loudly when it is not running
//! rather than skipping — a silently skipped AC-19 is the exact failure this suite exists
//! to prevent.

use std::sync::Arc;

use chrono::{DateTime, Duration};
use logandaim_api::openapi_service;
use logandaim_core::{
    App, adapters::InMemorySigning, application::IdentityProviderConfiguration, domain::konto::SigningKeyId,
    ports::Clock,
};
use logandaim_fakes::{FakeMailSender, FixedClock, InMemoryThrottle};
use logandaim_filesystemstore::FileSystemObjectStore;
use logandaim_smtp::{SmtpKonfiguration, SmtpMailSender};
use logandaim_sqlite::SQLiteInstance;
use poem::{Route, http::StatusCode, test::TestClient};
use serde_json::Value;

pub mod konto;
pub mod mailpit;
pub mod schuss;

/// Frontend routes the mailed links point at.
/// The frontend origin every e2e case's mailed links are spelled from.
pub const FRONTEND_BASIS_URL: &str = "https://logandaim.at";
pub const BESTÄTIGUNG_LINK: &str = "https://logandaim.at/registrierung";
pub const WIEDERHERSTELLUNG_LINK: &str = "https://logandaim.at/passwort";
pub const ISSUER: &str = "logandaim.at";
pub const AUDIENCE: &str = "logandaim-api";
pub const PASSWORT: &str = "korrekt-pferd-batterie";

/// The key every test signs with. A fixed seed, so a token from one run is verifiable in
/// another and a test failure is reproducible.
pub fn test_signing_key() -> InMemorySigning {
    InMemorySigning::from_secret_bytes(
        SigningKeyId::new("test-key").expect("static kid is valid"),
        &[7u8; 32],
    )
}

pub fn test_konfiguration() -> IdentityProviderConfiguration {
    IdentityProviderConfiguration::neu(ISSUER, AUDIENCE)
}

/// A built app plus the fakes a test has to reach into.
pub struct E2eHarness {
    pub client: LogandaimApiTestClient,
    pub mail: Arc<FakeMailSender>,
    pub clock: Arc<FixedClock>,
}

impl E2eHarness {
    pub async fn new() -> Self {
        let sqlite = SQLiteInstance::in_temp_dir().await.unwrap();
        sqlite.migrate_repos_to_latest().await.unwrap();

        let clock = Arc::new(FixedClock::new(
            DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
        ));
        let mail = Arc::new(FakeMailSender::new());

        let app = App::builder()
            .uow_provider(Box::new(sqlite))
            .object_store(Arc::new(FileSystemObjectStore::in_temp_dir().unwrap()))
            .signing(Arc::new(test_signing_key()))
            .throttle(Arc::new(InMemoryThrottle::new()))
            .mail_sender(Arc::clone(&mail) as Arc<dyn logandaim_core::ports::MailSender>)
            .konto(test_konfiguration())
            .clock(Arc::clone(&clock) as Arc<dyn Clock>)
            .build();

        Self {
            client: LogandaimApiTestClient::new(app),
            mail,
            clock,
        }
    }

    /// The same harness, but mail leaves over real SMTP to Mailpit.
    ///
    /// AC-19 is the only case that needs this: it asserts the link in a mail that survived
    /// the adapter, the transport, and a real relay. Everything else uses the fake, because
    /// a suite where every case needs a daemon is a suite that gets skipped.
    pub async fn mit_smtp() -> Self {
        mailpit::verlange_mailpit().await;

        let sqlite = SQLiteInstance::in_temp_dir().await.unwrap();
        sqlite.migrate_repos_to_latest().await.unwrap();

        let clock = Arc::new(FixedClock::new(
            DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
        ));
        let mail = Arc::new(FakeMailSender::new());

        let absender = logandaim_core::domain::EmailAdresse::normalisiert("noreply@logandaim.at")
            .expect("static address");
        let mut konfiguration = SmtpKonfiguration::mailpit(absender);
        konfiguration.frontend_basis_url = FRONTEND_BASIS_URL.to_owned();
        if let Ok(url) = std::env::var("LOGANDAIM_MAILPIT_SMTP") {
            if let Some((host, port)) = url.rsplit_once(':') {
                konfiguration.host = host.to_owned();
                konfiguration.port = port.parse().expect("SMTP-Port ist eine Zahl");
            }
        }
        let smtp = Arc::new(SmtpMailSender::neu(konfiguration).expect("der SMTP-Transport baut"));

        let app = App::builder()
            .uow_provider(Box::new(sqlite))
            .object_store(Arc::new(FileSystemObjectStore::in_temp_dir().unwrap()))
            .signing(Arc::new(test_signing_key()))
            .throttle(Arc::new(InMemoryThrottle::new()))
            .mail_sender(smtp as Arc<dyn logandaim_core::ports::MailSender>)
            .konto(test_konfiguration())
            .clock(Arc::clone(&clock) as Arc<dyn Clock>)
            .build();

        Self {
            client: LogandaimApiTestClient::new(app),
            mail,
            clock,
        }
    }

    /// Move time forward for every subsequent request.
    pub fn vorspulen(&self, dauer: Duration) {
        self.clock.advance_by(dauer);
    }

    /// The token of the most recently mailed link.
    pub fn letztes_token(&self) -> String {
        let mail = self.mail.letzte();
        token_aus(&mail.text(FRONTEND_BASIS_URL))
    }
}

pub struct LogandaimApiTestClient {
    client: TestClient<Route>,
}

impl LogandaimApiTestClient {
    pub fn new(app: App) -> Self {
        let service = openapi_service(app, std::iter::empty::<String>());
        let route = Route::new().nest("/api", service);
        Self {
            client: TestClient::new(route),
        }
    }

    pub fn client(&self) -> &TestClient<Route> {
        &self.client
    }

    pub async fn post(&self, path: &str) -> (StatusCode, Value) {
        json_response(self.client.post(path).send().await).await
    }

    pub async fn post_json(&self, path: &str, body: Value) -> (StatusCode, Value) {
        json_response(self.client.post(path).body_json(&body).send().await).await
    }

    /// A POST that also carries the `Authorization` header.
    pub async fn post_json_auth(
        &self,
        path: &str,
        body: Value,
        bearer: &str,
    ) -> (StatusCode, Value) {
        json_response(
            self.client
                .post(path)
                .header("Authorization", format!("Bearer {bearer}"))
                .body_json(&body)
                .send()
                .await,
        )
        .await
    }

    pub async fn get_json(&self, path: &str) -> (StatusCode, Value) {
        json_response(self.client.get(path).send().await).await
    }

    pub async fn get_json_auth(&self, path: &str, bearer: &str) -> (StatusCode, Value) {
        json_response(
            self.client
                .get(path)
                .header("Authorization", format!("Bearer {bearer}"))
                .send()
                .await,
        )
        .await
    }

    /// A request whose `Set-Cookie` header the caller needs, e.g. login or renewal.
    pub async fn post_json_cookie(
        &self,
        path: &str,
        body: Value,
        cookie: Option<&str>,
    ) -> (StatusCode, Value, Option<String>) {
        let mut request = self.client.post(path).body_json(&body);
        if let Some(cookie) = cookie {
            request = request.header("Cookie", cookie);
        }
        cookie_response(request.send().await).await
    }

    pub async fn post_cookie(
        &self,
        path: &str,
        cookie: Option<&str>,
    ) -> (StatusCode, Value, Option<String>) {
        let mut request = self.client.post(path);
        if let Some(cookie) = cookie {
            request = request.header("Cookie", cookie);
        }
        cookie_response(request.send().await).await
    }

    /// A POST that carries both the refresh cookie and the access token.
    pub async fn post_cookie_auth(
        &self,
        path: &str,
        cookie: Option<&str>,
        bearer: &str,
    ) -> (StatusCode, Value, Option<String>) {
        let mut request = self
            .client
            .post(path)
            .header("Authorization", format!("Bearer {bearer}"));
        if let Some(cookie) = cookie {
            request = request.header("Cookie", cookie);
        }
        cookie_response(request.send().await).await
    }

    pub async fn get_bytes(&self, path: &str) -> (StatusCode, Vec<u8>, String) {
        let resp = self.client.get(path).send().await;
        let status = resp.0.status();
        let content_type = resp
            .0
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_string();
        let body = resp.0.into_body().into_vec().await.unwrap_or_default();
        (status, body, content_type)
    }

    pub async fn put_json(&self, path: &str, body: Value) -> (StatusCode, Value) {
        json_response(self.client.put(path).body_json(&body).send().await).await
    }
}

async fn json_response(resp: poem::test::TestResponse) -> (StatusCode, Value) {
    let status = resp.0.status();
    let content_type = resp
        .0
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string();
    let body = resp.0.into_body().into_string().await.unwrap_or_default();
    let value = if status.is_success() {
        assert!(
            content_type.contains("application/json"),
            "expected JSON content type, got {content_type:?} for {status}"
        );
        serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("invalid JSON for {status}: {err}; body={body}"))
    } else if let Ok(value) = serde_json::from_str(&body) {
        value
    } else {
        Value::String(body)
    };
    (status, value)
}

/// Like [`json_response`], but also returns the `Set-Cookie` header.
async fn cookie_response(resp: poem::test::TestResponse) -> (StatusCode, Value, Option<String>) {
    let cookie = resp
        .0
        .headers()
        .get("set-cookie")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let (status, body) = json_response(resp).await;
    (status, body, cookie)
}

/// The `name=value` pair of a `Set-Cookie` header, ready to send back as `Cookie`.
pub fn cookie_pair(set_cookie: &str) -> String {
    set_cookie
        .split(';')
        .next()
        .expect("a Set-Cookie header has a name=value pair")
        .to_owned()
}

/// The token of a confirmation or recovery link in `text`.
///
/// One helper for `mailpit` and the fake, so both read a link the same way: the token is a
/// query parameter, and every token character is URL-safe base64.
pub fn token_aus(text: &str) -> String {
    let marker = "token=";
    let start = text
        .find(marker)
        .unwrap_or_else(|| panic!("mail enthält keinen token-link: {text}"))
        + marker.len();
    text[start..]
        .chars()
        .take_while(|zeichen| zeichen.is_ascii_alphanumeric() || *zeichen == '-' || *zeichen == '_')
        .collect()
}

/// A helper the health test and the schuss slice share.
pub async fn base_app_builder() -> App {
    let sqlite = SQLiteInstance::in_temp_dir().await.unwrap();
    sqlite.migrate_repos_to_latest().await.unwrap();
    App::builder()
        .uow_provider(Box::new(sqlite))
        .object_store(Arc::new(FileSystemObjectStore::in_temp_dir().unwrap()))
        .signing(Arc::new(test_signing_key()))
        .throttle(Arc::new(InMemoryThrottle::new()))
        .mail_sender(Arc::new(FakeMailSender::new()))
        .konto(test_konfiguration())
        .build()
}

pub fn assert_status_ok(status: StatusCode) {
    assert_eq!(status, StatusCode::OK, "expected 200 OK, got {status}");
}
