mod config;
mod tracing_setup;

use std::sync::Arc;

use error_stack::{Report, ResultExt};
use logandaim_api::{errors::InternalServerError, openapi_service};
use logandaim_core::{
    App,
    application::{AuthContext, KontoKonfiguration},
    domain::{
        EmailAdresse,
        konto::{SigningKeyId, SigningKeypair},
    },
    ports::MailSender,
    service::{Beförderung, SystemadministratorBefördern},
};
use logandaim_filesystemstore::FileSystemObjectStore;
use logandaim_smtp::{KeinVersand, SmtpKonfiguration, SmtpMailSender, Verschlüsselung};
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

/// Load the signing key from disk.
///
/// A missing or malformed key is fatal. The alternative — inventing one — would invalidate
/// every issued access token on the next restart, and a key committed to the repo would be
/// a key every checkout shares.
fn lade_signing_key(
    config: &config::ServerConfig,
) -> Result<SigningKeypair, Report<BackendServerError>> {
    let pem = std::fs::read_to_string(&config.signing_key_path).map_err(|source| {
        Report::new(BackendServerError).attach(format!(
            "signing key `{}` cannot be read ({source}); generate one with \
             `openssl genpkey -algorithm ed25519 -out {}`",
            config.signing_key_path.display(),
            config.signing_key_path.display()
        ))
    })?;

    let kid = SigningKeyId::new(config.signing_key_id.clone()).map_err(|_| {
        Report::new(BackendServerError).attach(format!(
            "signing key id `{}` is not a valid row id",
            config.signing_key_id
        ))
    })?;

    SigningKeypair::from_pkcs8_pem(kid, &pem).map_err(|_| {
        Report::new(BackendServerError).attach(format!(
            "signing key `{}` is not a valid PKCS#8 Ed25519 key",
            config.signing_key_path.display()
        ))
    })
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

/// Build the mail port from configuration.
///
/// No relay configured is not an error: the server starts, and every attempted mail logs a
/// warning. Refusing to start would make a mail misconfiguration an outage of the whole
/// product, including the parts that do not send mail.
fn lade_mail_sender(
    config: &config::ServerConfig,
) -> Result<Arc<dyn MailSender>, Report<BackendServerError>> {
    let Some(host) = &config.smtp_host else {
        tracing::warn!(
            "kein SMTP-Relay konfiguriert; Transaktionsmails schlagen fehl und werden geloggt"
        );
        return Ok(Arc::new(KeinVersand));
    };

    let absender = EmailAdresse::new(&config.smtp_absender).map_err(|_| {
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

/// Origins the browser app is served from.
///
/// An allowlist, not a prefix match on `http://localhost:`: an origin that merely *starts*
/// with a loopback prefix is a different origin, and the old prefix test let one through.
/// The frontend origin is deployment data, so it comes from configuration; the dev server's
/// documented ports are added for a developer machine that runs the app on another port.
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

    // A deployment behind another origin names it rather than widening the list.
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

/// CORS for the browser app.
///
/// Credentials are allowed because the refresh cookie is one. The `__Host-` cookie is
/// `SameSite=Lax`, so it is not sent on a cross-site POST either way; this is the second
/// line, not the first.
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

/// Promote the configured address to `Systemadministrator`.
///
/// `Systemadministrator` is never granted by an API path (rule 9), so a deployment
/// promotes one address from configuration. Idempotent: a second start finds the role
/// already set. An address that has not registered yet is not an error — the account is
/// promoted on a later start — so a fresh deployment boots before its superuser exists.
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

    let signing_key = lade_signing_key(&config)?;
    let mail_sender = lade_mail_sender(&config)?;
    let konto = KontoKonfiguration::neu(config.token_issuer.clone(), config.token_audience.clone());

    let app = App::builder()
        .uow_provider(Box::new(sqlite.clone()))
        .object_store(Arc::new(object_store))
        .signing_key(signing_key)
        .drossel(Arc::new(logandaim_core::adapters::InMemoryDrossel::new()))
        .mail_sender(mail_sender)
        .konto(konto)
        .build();

    // Publish the key row and check it matches what is stored. A stored row with the same
    // `kid` but different material is a rotation done wrong, and it refuses rather than
    // silently invalidating every token in circulation (AC-17).
    app.signing_key_registrieren()
        .await
        .change_context(BackendServerError)?;
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
