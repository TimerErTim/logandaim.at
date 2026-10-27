mod config;
mod tracing_setup;

use std::sync::Arc;

use error_stack::{Report, ResultExt};
use logandaim_api::{errors::InternalServerError, openapi_service};
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
fn lade_mail_sender(
    config: &config::ServerConfig,
) -> Result<Arc<dyn MailSender>, Report<BackendServerError>> {
    let Some(host) = &config.smtp_host else {
        tracing::warn!(
            "kein SMTP-Relay konfiguriert; Transaktionsmails schlagen fehl und werden geloggt"
        );
        return Ok(Arc::new(NullMailSender));
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

    let sender = SmtpMailSender::neu(smtp).change_context(BackendServerError)?;
    Ok(Arc::new(sender))
}

/// Origin allowlist: the configured frontend plus the documented dev-server ports.
fn allowed_origins(config: &config::ServerConfig) -> Vec<poem::http::HeaderValue> {
    let mut origins = vec![config.frontend_basis_url.clone()];

    origins.extend([4173u16, 3000u16].into_iter().flat_map(|port| {
        [
            format!("http://127.0.0.1:{port}"),
            format!("http://localhost:{port}"),
        ]
    }));

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

#[tokio::main]
async fn main() -> Result<(), Report<BackendServerError>> {
    let config = config::load().change_context(BackendServerError)?;
    init_tracing(config.log_dir.as_deref());

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
    let mail_sender = lade_mail_sender(&config)?;
    let konto = IdentityProviderConfiguration::neu(
        config.token_issuer.clone(),
        config.token_audience.clone(),
    );

    let app = App::builder()
        .uow_provider(Box::new(sqlite.clone()))
        .object_store(Arc::new(object_store))
        .signing(Arc::new(signing_key))
        .throttle(Arc::new(InMemoryThrottle::new()))
        .mail_sender(mail_sender)
        .konto(konto)
        .build();

    bootstrap_superuser(&app).await?;

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
    let api_service = openapi_service(app, [api_url.clone()]);

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
