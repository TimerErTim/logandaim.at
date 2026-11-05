mod config;
mod tracing_setup;

use std::sync::Arc;

use error_stack::{Report, ResultExt};
use logandaim_api::{
    errors::InternalServerError, health::MailProbe, openapi_service_with_mail_health,
};
use logandaim_core::{
    App,
    adapters::{InMemorySigning, InMemoryThrottle, NullMailSender},
    application::{AuthContext, IdentityProviderConfiguration},
    domain::EmailAdresse,
    ports::MailSender,
    service::{Beförderung, SystemadministratorBefördern},
};
use logandaim_filesystemstore::FileSystemObjectStore;
use logandaim_smtp::{SmtpKonfiguration, SmtpMailSender, Verschlüsselung};
use logandaim_sqlite::SQLiteInstance;
use poem::http::StatusCode;
use poem::middleware::{CatchPanic, Compression, Cors, Middleware, RequestId, ReuseId, Tracing};
use poem::{EndpointExt, IntoResponse, Route, Server, listener::TcpListener};
use poem_openapi::payload::PlainText;
use thiserror::Error;
use tracing_setup::init_tracing;

#[derive(Debug, Error)]
#[error("Backend server fatal error")]
pub struct BackendServerError;

/// Read the configured PEM, or generate a throwaway key.
///
/// A malformed configured file is fatal: a deployment that points at a key expects that
/// key. Without a path every restart invalidates the sessions it issued.
fn build_signing_key(
    config: &config::ServerConfig,
) -> Result<InMemorySigning, Report<BackendServerError>> {
    let Some(path) = &config.signing_key_path else {
        tracing::info!("no signing key configured; generated a throwaway key for this process");
        return Ok(InMemorySigning::random());
    };

    let pem = std::fs::read_to_string(path).map_err(|source| {
        Report::new(BackendServerError).attach(format!(
            "signing key `{}` cannot be read ({source})",
            path.display()
        ))
    })?;

    let keypair =
        logandaim_core::domain::konto::SigningKeypair::from_pkcs8_pem(&pem).map_err(|_| {
            Report::new(BackendServerError).attach(format!(
                "signing key `{}` is not a valid PKCS#8 Ed25519 key",
                path.display()
            ))
        })?;
    Ok(InMemorySigning::new(keypair))
}

fn catch_panic() -> CatchPanic<impl poem::middleware::PanicHandler> {
    CatchPanic::new().with_handler(|err| {
        tracing::error!("Panic: {:?}", dbg!(err));
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            PlainText(InternalServerError),
        )
            .into_response()
    })
}

/// Build the mail port. No relay is not an error: mail fails and is logged (NFR-5).
///
/// The sender is returned together with the probe the health loop uses, so the SMTP
/// transport is built once and both roles share it.
fn lade_mail_sender(
    config: &config::ServerConfig,
) -> Result<Mailversand, Report<BackendServerError>> {
    let Some(host) = &config.smtp_host else {
        tracing::warn!(
            "kein SMTP-Relay konfiguriert; Transaktionsmails schlagen fehl und werden geloggt"
        );
        return Ok(Mailversand {
            sender: Arc::new(NullMailSender),
            probe: None,
        });
    };

    let absender = EmailAdresse::normalisiert(&config.smtp_absender).map_err(|_| {
        Report::new(BackendServerError).attach(format!(
            "SMTP sender `{}` is not an address",
            config.smtp_absender
        ))
    })?;

    let smtp = SmtpKonfiguration {
        host: host.clone(),
        port: config.smtp_port,
        verschlüsselung: match config.smtp_verschlüsselung {
            config::SmtpVerschlüsselung::Klar => Verschlüsselung::Klar,
            config::SmtpVerschlüsselung::StartTls => Verschlüsselung::StartTls,
            config::SmtpVerschlüsselung::Implizit => Verschlüsselung::Implizit,
        },
        benutzername: config.smtp_benutzername.clone(),
        passwort: config.smtp_passwort.clone(),
        absender,
        absender_name: Some("logandaim".to_owned()),
        frontend_basis_url: config.frontend_basis_url.clone(),
    };

    let sender = Arc::new(SmtpMailSender::neu(smtp).change_context(BackendServerError)?);
    Ok(Mailversand {
        sender: sender.clone(),
        probe: Some(Arc::new(SmtpProbe(sender))),
    })
}

/// The two roles one SMTP configuration serves: sending, and being probed for health.
struct Mailversand {
    sender: Arc<dyn MailSender>,
    /// Absent when no relay is configured: there is nothing to probe, and the endpoint
    /// reports `unknown` rather than a verdict nobody produced.
    probe: Option<Arc<SmtpProbe>>,
}

/// The SMTP transport as a health probe.
///
/// A local newtype because `MailProbe` and `SmtpMailSender` both live in other crates, and
/// the error is rendered here into the sentence the endpoint serves.
struct SmtpProbe(Arc<SmtpMailSender>);

#[async_trait::async_trait]
impl MailProbe for SmtpProbe {
    async fn check_connection(&self) -> Result<bool, String> {
        match self.0.prüfe_verbindung().await {
            Ok(erreichbar) => Ok(erreichbar),
            Err(fehler) => Err(format!("{fehler:?}")),
        }
    }
}

/// Origin allowlist: the configured frontend plus the documented dev-server ports.
fn allowed_origins(config: &config::ServerConfig) -> Vec<poem::http::HeaderValue> {
    let mut origins = vec![config.frontend_basis_url.clone()];

    origins.extend(
        [4173u16, 3001, 3002, 3003, 3004, 3005, 3006]
            .into_iter()
            .flat_map(|port| {
                [
                    format!("http://127.0.0.1:{port}"),
                    format!("http://localhost:{port}"),
                ]
            }),
    );

    origins.push(format!(
        "http://{}:{}",
        config.bind_address, config.bind_port
    ));

    if let Ok(extra) = std::env::var("LOGANDAIM_CORS_ORIGIN") {
        origins.extend(
            extra
                .split(',')
                .map(str::trim)
                .filter(|origin| !origin.is_empty())
                .map(str::to_owned),
        );
    }

    origins
        .into_iter()
        .filter_map(|origin| origin.parse().ok())
        .collect()
}

/// CORS for the browser app; credentials are allowed because the refresh cookie is one.
fn cors(config: &config::ServerConfig) -> Cors {
    Cors::new()
        .allow_credentials(true)
        .allow_origins(allowed_origins(config))
        .allow_methods([
            poem::http::Method::GET,
            poem::http::Method::POST,
            poem::http::Method::PUT,
            poem::http::Method::DELETE,
            poem::http::Method::OPTIONS,
        ])
        .allow_headers([
            poem::http::header::AUTHORIZATION,
            poem::http::header::CONTENT_TYPE,
        ])
}

/// Promote the configured address to `Systemadministrator`; idempotent, and an address
/// without a Konto yet is not an error.
async fn bootstrap_superuser(app: &App) -> Result<(), Report<BackendServerError>> {
    let Some(email) = std::env::var("LOGANDAIM_SUPERUSER_EMAIL")
        .ok()
        .filter(|wert| !wert.trim().is_empty())
    else {
        return Ok(());
    };

    let adresse = EmailAdresse::normalisiert(&email).map_err(|_| {
        Report::new(BackendServerError)
            .attach(format!("superuser address `{email}` is not an address"))
    })?;

    let ergebnis = app
        .execute(
            AuthContext::Anonymous,
            SystemadministratorBefördern { email: adresse },
        )
        .await
        .change_context(BackendServerError)?;

    match ergebnis {
        Beförderung::Befördert => tracing::info!("superuser befördert"),
        Beförderung::BereitsBefördert => {
            tracing::info!("superuser war bereits befördert")
        }
        Beförderung::KeinKonto => {
            tracing::warn!("superuser-adresse hat noch kein konto; beim nächsten start erneut")
        }
    }
    Ok(())
}

/// Create the directory a configured database path lives in.
///
/// libsql creates the database file but not its parent, and the configured path is only
/// resolved here, so this is where a missing directory has to be made. In-memory URLs and
/// a bare file name have no parent to create.
fn ensure_database_directory(database_url: &str) -> Result<(), Report<BackendServerError>> {
    let Some(parent) = std::path::Path::new(database_url)
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    else {
        return Ok(());
    };
    std::fs::create_dir_all(parent)
        .change_context(BackendServerError)
        .attach(format!(
            "while creating the database directory `{}`",
            parent.display()
        ))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Report<BackendServerError>> {
    let config = config::load().change_context(BackendServerError)?;
    init_tracing(config.log_dir.as_deref());

    ensure_database_directory(&config.database_url)?;
    let sqlite = Arc::new(
        SQLiteInstance::local(&config.database_url)
            .await
            .change_context(BackendServerError)?,
    );
    sqlite
        .migrate_repos_to_latest()
        .await
        .change_context(BackendServerError)?;

    let object_store = FileSystemObjectStore::new(config.object_store_dir.clone())
        .change_context(BackendServerError)?;

    let signing_key = build_signing_key(&config)?;
    let mail = lade_mail_sender(&config)?;
    let konto = IdentityProviderConfiguration::neu(
        config.token_issuer.clone(),
        config.token_audience.clone(),
    );

    let app = App::builder()
        .uow_provider(Box::new(sqlite.clone()))
        .object_store(Arc::new(object_store))
        .signing(Arc::new(signing_key))
        .throttle(Arc::new(InMemoryThrottle::new()))
        .mail_sender(mail.sender)
        .konto(konto)
        .build();

    bootstrap_superuser(&app).await?;

    // The mail verdict lives beside the app, not inside it: the route reads it, and no use
    // case may, so a request can never wait on or branch on the relay (NFR-5).
    let mail_health = logandaim_api::health::MailHealthState::new();
    if let Some(probe) = mail.probe {
        tokio::spawn(logandaim_api::health::monitor_mail(
            probe,
            mail_health.clone(),
        ));
    } else {
        tracing::warn!("kein SMTP-Relay konfiguriert; Mailgesundheit bleibt unbeantwortet");
    }

    let base_path = config.subpath.trim_matches('/');
    let subpath = if base_path.is_empty() {
        "/".to_string()
    } else {
        format!("/{}/", base_path)
    };

    let api_url = format!(
        "http://{}:{}{}api",
        config.bind_address, config.bind_port, subpath
    );

    // TODO: add dynamic version loading
    let api_service = openapi_service_with_mail_health(app, mail_health, [api_url.clone()]);

    let tracing = Tracing.combine(RequestId::new().reuse_id(ReuseId::Use));

    let app = Route::new()
        .nest("/swagger", api_service.swagger_ui())
        .nest("/redoc", api_service.redoc())
        .nest("/spec.json", api_service.spec_endpoint())
        .nest(
            "/spec.yaml",
            api_service.spec_endpoint_yaml().after(|res| async move {
                res.map(IntoResponse::into_response).map(|mut resp| {
                    resp.headers_mut().remove("content-disposition");
                    resp
                })
            }),
        )
        .nest("/api", api_service)
        .with(Compression::new())
        .with(tracing)
        .with(catch_panic())
        .with(cors(&config));

    tracing::info!("Server started at {}", api_url);
    Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.bind_address, config.bind_port
    )))
    .run(app)
    .await
    .change_context(BackendServerError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression: a configured database path may name a directory that does not exist
    /// yet. libsql creates the file but not its parent, so a first run with a nested path
    /// failed with `Unable to open connection ... (14)`. Creating the parent is the
    /// consumer's job, because the path is only known here.
    #[test_log::test]
    fn a_missing_database_directory_is_created() {
        let root = tempdir::TempDir::new("logandaim-db-directory").unwrap();
        let database = root.path().join("data.local").join("logandaim.db");
        assert!(
            !database.parent().unwrap().exists(),
            "the test needs a parent that does not exist yet"
        );

        ensure_database_directory(database.to_str().unwrap()).expect("create the directory");

        assert!(database.parent().unwrap().is_dir());
    }

    /// A bare file name has no directory to create, and an in-memory database has none
    /// either; neither may fail.
    #[test_log::test]
    fn a_bare_file_name_needs_no_directory() {
        ensure_database_directory("logandaim.db").expect("no directory to create");
        ensure_database_directory(":memory:").expect("no directory to create");
    }
}
