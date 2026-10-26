# Backend

The backend is implemented using [libsql](https://libsql.org) (SQLite) with a hexagonal architecture in Rust.

## Architecture

The backend follows the Hexagonal Architecture (Ports and Adapters) pattern:

- **Core Domain**: Pure Rust business logic and repository traits. Accounts (`Benutzer`, `Konto`,
  `Session`, credentials) are a **supporting subdomain** under `crates/logandaim-core/src/domain/konto`,
  including the cryptographic decisions: argon2id password hashing, SHA-256 token hashing, and
  Ed25519 signing. Mail delivery and throttling are ports because they touch the outside world.
- **Persistence Adapter**: Implementation of repository traits using `libsql`.
- **Standalone Server**: REST API using `poem-openapi`.

Every use case runs through `App::execute`, which takes an `AuthContext` naming the acting identity
(`Anonymous` or a logged-in `Benutzer`). The API layer extracts it from the bearer token; the core
decides authorisation inside the use case body.

## Standalone Server

To run the standalone webserver:

```bash
mise run dev:server
```

### Configuration

Resolution order: **config path → file → env → CLI**.

Default file: `logandaim-server.json` in the current working directory. Override with `LOGANDAIM_CONFIG_PATH` or `--config-path`. Missing default file falls back to built-in defaults. An explicit path that is missing or malformed is an error.

mise sets environment variables for the server suitable for a dev environment automatically.

| Source | Keys |
|--------|------|
| File (camelCase JSON) | `bindAddress`, `bindPort`, `subpath`, `databaseUrl`, `objectStoreDir`, `logDir`, `signingKeyPath`, `signingKeyId`, `tokenIssuer`, `tokenAudience`, `bestaetigungLink`, `wiederherstellungLink`, `smtpHost`, `smtpPort`, `smtpVerschluesselung`, `smtpBenutzername`, `smtpPasswort`, `smtpAbsender` |
| Env / CLI | `LOGANDAIM_BIND_ADDRESS` / `--bind-address`, `LOGANDAIM_BIND_PORT` / `--bind-port`, `SUBPATH` / `--subpath`, `LOGANDAIM_DATABASE_URL` / `--database-url`, `LOGANDAIM_OBJECT_STORE_DIR` / `--object-store-dir`, `LOGANDAIM_LOG_DIR` / `--log-dir`, `LOGANDAIM_SIGNING_KEY_PATH` / `--signing-key-path`, `LOGANDAIM_SIGNING_KEY_ID` / `--signing-key-id`, `LOGANDAIM_TOKEN_ISSUER` / `--token-issuer`, `LOGANDAIM_TOKEN_AUDIENCE` / `--token-audience`, `LOGANDAIM_BESTAETIGUNG_LINK` / `--bestaetigung-link`, `LOGANDAIM_WIEDERHERSTELLUNG_LINK` / `--wiederherstellung-link`, `LOGANDAIM_SMTP_HOST` / `--smtp-host`, `LOGANDAIM_SMTP_PORT` / `--smtp-port`, `LOGANDAIM_SMTP_VERSCHLUESSELUNG` / `--smtp-verschluesselung`, `LOGANDAIM_SMTP_BENUTZERNAME` / `--smtp-benutzername`, `LOGANDAIM_SMTP_PASSWORT` / `--smtp-passwort`, `LOGANDAIM_SMTP_ABSENDER` / `--smtp-absender` |
| Accounts (env only) | `LOGANDAIM_SUPERUSER_EMAIL` — the address promoted to `Systemadministrator` at startup |
| Tracing | `LOGANDAIM_LOG_LEVEL` (default `info`) |

Notes on the account keys:

- `signingKeyPath` is a PKCS#8 PEM Ed25519 private key. There is no generated fallback: a key
  invented at startup would invalidate every session on restart. Generate one with
  `openssl genpkey -algorithm ed25519 -out signing-key.pem`.
- `signingKeyId` is the `kid` the key is published under. Rotating means running two keys side by
  side: add the new row, point the id at it, and retire the old one once its tokens expire (AC-17).
- `tokenIssuer` and `tokenAudience` are the `iss` and `aud` claims, verified on every request.
- `smtpHost` absent means "no relay": mails fail and are logged, never failing the request (NFR-5).
- `LOGANDAIM_SUPERUSER_EMAIL` is the only way a `Systemadministrator` is created — there is no
  self-service elevation (rule 9). Promotion is idempotent and tolerates an address that has not
  registered yet; it is retried on the next start.

### Transactional mail in development

Development and tests use [Mailpit](https://mailpit.axllent.org), a local SMTP catcher with a web
UI, so no real address receives anything:

```bash
mise run run:mailpit   # SMTP 127.0.0.1:1025, web UI http://127.0.0.1:8025
```

The e2e suite does the same through the mise `mailpit` daemon. Confirmation and recovery mails open
in the web UI, and the link in them points at `bestaetigungLink` / `wiederherstellungLink`.

## Spec Export

To export the OpenAPI spec:

```bash
mise run sync:openapi
```
