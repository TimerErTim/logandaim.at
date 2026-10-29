# Backend

The backend is implemented using [libsql](https://libsql.org) (SQLite) with a hexagonal architecture in Rust.

## Architecture

The backend follows the Hexagonal Architecture (Ports and Adapters) pattern:

- **Core Domain**: Pure Rust business logic and repository traits.
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

mise sets environment variables for the server suitable for a dev environment automatically. `SUBPATH` (default `/`) sets the API mount prefix; routes are served below `<SUBPATH>/api`.

| Source | Keys |
|--------|------|
| File (camelCase JSON) | `bindAddress`, `bindPort`, `subpath`, `databaseUrl`, `objectStoreDir`, `logDir`, `signingKeyPath`, `tokenIssuer`, `tokenAudience`, `frontendBasisUrl`, `smtpHost`, `smtpPort`, `smtpVerschlüsselung`, `smtpBenutzername`, `smtpPasswort`, `smtpAbsender` |
| Env / CLI | `LOGANDAIM_BIND_ADDRESS` / `--bind-address`, `LOGANDAIM_BIND_PORT` / `--bind-port`, `SUBPATH` / `--subpath`, `LOGANDAIM_DATABASE_URL` / `--database-url`, `LOGANDAIM_OBJECT_STORE_DIR` / `--object-store-dir`, `LOGANDAIM_LOG_DIR` / `--log-dir`, `LOGANDAIM_SIGNING_KEY_PATH` / `--signing-key-path`, `LOGANDAIM_TOKEN_ISSUER` / `--token-issuer`, `LOGANDAIM_TOKEN_AUDIENCE` / `--token-audience`, `LOGANDAIM_FRONTEND_BASIS_URL` / `--frontend-basis-url`, `LOGANDAIM_SMTP_HOST` / `--smtp-host`, `LOGANDAIM_SMTP_PORT` / `--smtp-port`, `LOGANDAIM_SMTP_VERSCHLUESSELUNG` / `--smtp-verschlüsselung`, `LOGANDAIM_SMTP_BENUTZERNAME` / `--smtp-benutzername`, `LOGANDAIM_SMTP_PASSWORT` / `--smtp-passwort`, `LOGANDAIM_SMTP_ABSENDER` / `--smtp-absender` |
| Accounts (env only) | `LOGANDAIM_SUPERUSER_EMAIL` — the address promoted to `Systemadministrator` at startup |
| CORS (env only) | `LOGANDAIM_CORS_ORIGIN` — optional comma-separated extra allowed origins; the configured frontend origin and documented loopback dev origins are allowed by default |
| Tracing | `LOGANDAIM_LOG_LEVEL` (default `info`) |

Notes on the account keys:

- `signingKeyPath` is optional. Absent, the server generates a throwaway Ed25519 key per
  process — convenient locally, and it invalidates every issued token on restart. A long-lived
  deployment sets a PKCS#8 PEM path instead. There is no key id: each session row stores the
  public key it was issued under, so verification needs no key table (AC-17).
- `tokenIssuer` and `tokenAudience` are the `iss` and `aud` claims, verified on every request.
- `frontendBasisUrl` is where the browser app is served (`http://localhost:3001` by default). Mailed
  links are this origin plus the product's own route — `/registrierung` and `/passwort` — so a
  deployment never configures two full URLs that can drift apart from the routes they name.
- `smtpHost` absent means "no relay": mails fail and are logged, never failing the request (NFR-5).
- `LOGANDAIM_SUPERUSER_EMAIL` is the only way a `Systemadministrator` is created — there is no
  self-service elevation (rule 9). Promotion is idempotent and tolerates an address that has not
  registered yet; it is retried on the next start.

### Transactional mail in development

Development and tests use [Mailpit](https://mailpit.axllent.org), a local SMTP catcher with a web
UI, so no real address receives anything:

```bash
mise daemons start mailpit   # SMTP 127.0.0.1:1025, web UI http://127.0.0.1:8025
```

The e2e suite does the same through the mise `mailpit` daemon. Confirmation and recovery mails open
in the web UI, and the link in them is `frontendBasisUrl` plus `/registrierung` or `/passwort`.

## Docker Compose deployment

The VPS Compose deployment, environment template, Caddy configuration, and operator runbook are in [`docs/deployment.md`](../docs/deployment.md). Docker and Compose run on the VPS outside `mise`; the backend image installs and runs `mise` only during its build.

## Spec Export

To export the OpenAPI spec:

```bash
mise run sync:openapi
```
