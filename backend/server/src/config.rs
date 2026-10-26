use std::path::{Path, PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const DEFAULT_CONFIG_FILE: &str = "logandaim-server.json";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file `{path}` does not exist")]
    NotFound { path: String },
    #[error("failed to read config file `{path}`: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("config file `{path}` is malformed: {message}")]
    Malformed { path: String, message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ServerConfig {
    pub bind_address: String,
    pub bind_port: u16,
    pub subpath: String,
    pub database_url: String,
    pub object_store_dir: PathBuf,
    pub log_dir: Option<PathBuf>,
    /// Path to the PKCS#8 PEM private key that signs access tokens.
    ///
    /// Absent means "generate a throwaway key for this process", which is what a local run
    /// wants. A production deployment sets one: a generated key invalidates every issued
    /// access token on the next restart.
    pub signing_key_path: Option<PathBuf>,
    /// Row id the key is published under, so a rotation can run two keys side by side
    /// while old tokens expire (AC-17).
    pub signing_key_id: String,
    /// Issuer claim of every access token.
    pub token_issuer: String,
    /// Audience claim of every access token.
    pub token_audience: String,
    /// Frontend origin the mailed links point at. The routes are the product's; this is
    /// only where the app is served.
    pub frontend_basis_url: String,
    /// SMTP relay host. Absent means "no relay": every send fails and is logged.
    pub smtp_host: Option<String>,
    pub smtp_port: u16,
    pub smtp_verschlüsselung: SmtpVerschlüsselung,
    pub smtp_benutzername: Option<String>,
    pub smtp_passwort: Option<String>,
    /// Envelope sender of every transactional mail.
    pub smtp_absender: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".into(),
            bind_port: 3000,
            subpath: "/".into(),
            database_url: "logandaim.db".into(),
            object_store_dir: PathBuf::from("storage/"),
            log_dir: None,
            signing_key_path: None,
            signing_key_id: "k1".into(),
            token_issuer: "logandaim.at".into(),
            token_audience: "logandaim-api".into(),
            frontend_basis_url: "http://localhost:3000".into(),
            smtp_host: None,
            smtp_port: 1025,
            smtp_verschlüsselung: SmtpVerschlüsselung::Klar,
            smtp_benutzername: None,
            smtp_passwort: None,
            smtp_absender: "noreply@logandaim.at".into(),
        }
    }
}

/// How the SMTP adapter talks to its relay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, clap::ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum SmtpVerschlüsselung {
    /// Plain SMTP: Mailpit on loopback in development and tests.
    #[default]
    Klar,
    /// `STARTTLS` upgrade, requiring a valid certificate.
    StartTls,
    /// TLS from the first byte (submission port 465).
    Implizit,
}

#[derive(Debug, Parser, Default, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the config file (`.json` or `.toml`)
    #[arg(long = "config-path", env = "LOGANDAIM_CONFIG_PATH")]
    pub config_path: Option<PathBuf>,

    /// IP address to bind to
    #[arg(long, env = "LOGANDAIM_BIND_ADDRESS")]
    pub bind_address: Option<String>,

    /// Port to bind to
    #[arg(long, env = "LOGANDAIM_BIND_PORT")]
    pub bind_port: Option<u16>,

    /// Subpath this service is hosted on
    #[arg(long, env = "SUBPATH")]
    pub subpath: Option<String>,

    /// Database URL
    #[arg(long, env = "LOGANDAIM_DATABASE_URL")]
    pub database_url: Option<String>,

    /// Directory for object store
    #[arg(long, env = "LOGANDAIM_OBJECT_STORE_DIR")]
    pub object_store_dir: Option<PathBuf>,

    /// Directory for rotated JSON log files (omit to disable logging)
    #[arg(long, env = "LOGANDAIM_LOG_DIR")]
    pub log_dir: Option<PathBuf>,

    /// PKCS#8 PEM private key that signs access tokens; omit to generate one per process
    #[arg(long, env = "LOGANDAIM_SIGNING_KEY_PATH")]
    pub signing_key_path: Option<PathBuf>,

    /// Row id the signing key is published under
    #[arg(long, env = "LOGANDAIM_SIGNING_KEY_ID")]
    pub signing_key_id: Option<String>,

    /// Issuer claim of every access token
    #[arg(long, env = "LOGANDAIM_TOKEN_ISSUER")]
    pub token_issuer: Option<String>,

    /// Audience claim of every access token
    #[arg(long, env = "LOGANDAIM_TOKEN_AUDIENCE")]
    pub token_audience: Option<String>,

    /// Frontend origin the mailed links point at
    #[arg(long, env = "LOGANDAIM_FRONTEND_BASIS_URL")]
    pub frontend_basis_url: Option<String>,

    /// SMTP relay host; omit to run without mail delivery
    #[arg(long, env = "LOGANDAIM_SMTP_HOST")]
    pub smtp_host: Option<String>,

    /// SMTP relay port
    #[arg(long, env = "LOGANDAIM_SMTP_PORT")]
    pub smtp_port: Option<u16>,

    /// SMTP transport: klar, starttls, or implizit
    #[arg(long, env = "LOGANDAIM_SMTP_VERSCHLUESSELUNG")]
    pub smtp_verschlüsselung: Option<SmtpVerschlüsselung>,

    /// SMTP username
    #[arg(long, env = "LOGANDAIM_SMTP_BENUTZERNAME")]
    pub smtp_benutzername: Option<String>,

    /// SMTP password
    #[arg(long, env = "LOGANDAIM_SMTP_PASSWORT")]
    pub smtp_passwort: Option<String>,

    /// Envelope sender of transactional mail
    #[arg(long, env = "LOGANDAIM_SMTP_ABSENDER")]
    pub smtp_absender: Option<String>,
}

pub fn load() -> Result<ServerConfig, ConfigError> {
    resolve(Cli::parse())
}

pub fn resolve(cli: Cli) -> Result<ServerConfig, ConfigError> {
    let explicit = cli.config_path.is_some();
    let path = cli
        .config_path
        .clone()
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_FILE));
    let file = load_file(&path, explicit)?;
    Ok(overlay(file, &cli))
}

fn load_file(path: &Path, explicit: bool) -> Result<ServerConfig, ConfigError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => match parse_config_file::<ServerConfig>(path, &contents) {
            Ok(config) => Ok(config),
            Err(message) => {
                if explicit {
                    Err(ConfigError::Malformed {
                        path: path.display().to_string(),
                        message,
                    })
                } else {
                    tracing::warn!(
                        path = %path.display(),
                        error = %message,
                        "config file is malformed; using defaults"
                    );
                    Ok(ServerConfig::default())
                }
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            if explicit {
                Err(ConfigError::NotFound {
                    path: path.display().to_string(),
                })
            } else {
                tracing::warn!(
                    path = %path.display(),
                    "config file does not exist; using defaults"
                );
                Ok(ServerConfig::default())
            }
        }
        Err(source) => Err(ConfigError::Io {
            path: path.display().to_string(),
            source,
        }),
    }
}

fn overlay(mut config: ServerConfig, cli: &Cli) -> ServerConfig {
    if let Some(bind_address) = &cli.bind_address {
        config.bind_address = bind_address.clone();
    }
    if let Some(bind_port) = cli.bind_port {
        config.bind_port = bind_port;
    }
    if let Some(subpath) = &cli.subpath {
        config.subpath = subpath.clone();
    }
    if let Some(database_url) = &cli.database_url {
        config.database_url = database_url.clone();
    }
    if let Some(object_store_dir) = &cli.object_store_dir {
        config.object_store_dir = object_store_dir.clone();
    }
    if let Some(log_dir) = &cli.log_dir {
        config.log_dir = Some(log_dir.clone());
    }
    if cli.signing_key_path.is_some() {
        config.signing_key_path = cli.signing_key_path.clone();
    }
    if let Some(signing_key_id) = &cli.signing_key_id {
        config.signing_key_id = signing_key_id.clone();
    }
    if let Some(token_issuer) = &cli.token_issuer {
        config.token_issuer = token_issuer.clone();
    }
    if let Some(token_audience) = &cli.token_audience {
        config.token_audience = token_audience.clone();
    }
    if let Some(frontend_basis_url) = &cli.frontend_basis_url {
        config.frontend_basis_url = frontend_basis_url.clone();
    }
    if let Some(smtp_host) = &cli.smtp_host {
        config.smtp_host = Some(smtp_host.clone());
    }
    if let Some(smtp_port) = cli.smtp_port {
        config.smtp_port = smtp_port;
    }
    if let Some(smtp_verschlüsselung) = cli.smtp_verschlüsselung {
        config.smtp_verschlüsselung = smtp_verschlüsselung;
    }
    if let Some(smtp_benutzername) = &cli.smtp_benutzername {
        config.smtp_benutzername = Some(smtp_benutzername.clone());
    }
    if let Some(smtp_passwort) = &cli.smtp_passwort {
        config.smtp_passwort = Some(smtp_passwort.clone());
    }
    if let Some(smtp_absender) = &cli.smtp_absender {
        config.smtp_absender = smtp_absender.clone();
    }
    config
}

fn parse_config_file<T: serde::de::DeserializeOwned>(
    path: &Path,
    contents: &str,
) -> Result<T, String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("toml") => toml::from_str(contents).map_err(|err| err.to_string()),
        Some("json") => serde_json::from_str(contents).map_err(|err| err.to_string()),
        Some(ext) => Err(format!(
            "unsupported config format `.{ext}`; use `.json` or `.toml`"
        )),
        None => serde_json::from_str(contents).map_err(|err| err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cli_overlay(cli: Cli) -> ServerConfig {
        overlay(ServerConfig::default(), &cli)
    }

    #[test_log::test]
    fn defaults_when_no_overlay() {
        assert_eq!(cli_overlay(Cli::default()), ServerConfig::default());
    }

    #[test_log::test]
    fn file_values_used_when_no_overlay() {
        let file = ServerConfig {
            database_url: "from-file.db".into(),
            bind_port: 4000,
            ..ServerConfig::default()
        };
        assert_eq!(overlay(file.clone(), &Cli::default()), file);
    }

    #[test_log::test]
    fn env_or_cli_overlay_beats_file() {
        let file = ServerConfig {
            database_url: "from-file.db".into(),
            bind_port: 4000,
            bind_address: "0.0.0.0".into(),
            ..ServerConfig::default()
        };
        let cli = Cli {
            database_url: Some("from-env.db".into()),
            bind_port: Some(5000),
            ..Cli::default()
        };
        let resolved = overlay(file, &cli);
        assert_eq!(resolved.database_url, "from-env.db");
        assert_eq!(resolved.bind_port, 5000);
        assert_eq!(resolved.bind_address, "0.0.0.0");
    }

    #[test_log::test]
    fn overlays_log_dir() {
        let cli = Cli {
            log_dir: Some(PathBuf::from("/var/log/logandaim.at")),
            ..Cli::default()
        };
        let resolved = cli_overlay(cli);
        assert_eq!(
            resolved.log_dir,
            Some(PathBuf::from("/var/log/logandaim.at"))
        );
    }

    #[test_log::test]
    fn loads_log_dir_from_json_file() {
        let dir = std::env::temp_dir().join(format!(
            "logandaim-server-config-log-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.json");
        std::fs::write(
            &path,
            r#"{
                "logDir": "server-logs/"
            }"#,
        )
        .unwrap();

        let loaded = load_file(&path, true).unwrap();
        assert_eq!(loaded.log_dir, Some(PathBuf::from("server-logs/")));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test_log::test]
    fn loads_log_dir_from_toml_file() {
        let dir = std::env::temp_dir().join(format!(
            "logandaim-server-config-log-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");
        std::fs::write(
            &path,
            r#"
logDir = "server-logs/"
"#,
        )
        .unwrap();

        let loaded = load_file(&path, true).unwrap();
        assert_eq!(loaded.log_dir, Some(PathBuf::from("server-logs/")));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test_log::test]
    fn missing_default_file_uses_defaults() {
        let path = PathBuf::from("definitely-missing-logandaim-server.json");
        let loaded = load_file(&path, false).unwrap();
        assert_eq!(loaded, ServerConfig::default());
    }

    #[test_log::test]
    fn missing_explicit_file_errors() {
        let path = PathBuf::from("definitely-missing-logandaim-server.json");
        let err = load_file(&path, true).unwrap_err();
        assert!(matches!(err, ConfigError::NotFound { .. }));
    }

    #[test_log::test]
    fn loads_json_file() {
        let dir = std::env::temp_dir().join(format!(
            "logandaim-server-config-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.json");
        std::fs::write(
            &path,
            r#"{
                "databaseUrl": "file.db",
                "objectStoreDir": "objects/",
                "bindAddress": "10.0.0.1",
                "bindPort": 1234,
                "subpath": "/logandaim.at/"
            }"#,
        )
        .unwrap();

        let loaded = load_file(&path, true).unwrap();
        assert_eq!(loaded.database_url, "file.db");
        assert_eq!(loaded.object_store_dir, PathBuf::from("objects/"));
        assert_eq!(loaded.bind_address, "10.0.0.1");
        assert_eq!(loaded.bind_port, 1234);
        assert_eq!(loaded.subpath, "/logandaim.at/");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test_log::test]
    fn malformed_explicit_file_errors() {
        let dir = std::env::temp_dir().join(format!(
            "logandaim-server-config-bad-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.json");
        std::fs::write(&path, "{ not json").unwrap();

        let err = load_file(&path, true).unwrap_err();
        assert!(matches!(err, ConfigError::Malformed { .. }));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test_log::test]
    fn clap_cli_wins_over_env_for_config_path() {
        let cli =
            Cli::try_parse_from(["logandaim-server", "--config-path", "/tmp/cli.json"]).unwrap();
        assert_eq!(cli.config_path, Some(PathBuf::from("/tmp/cli.json")));
    }
}
