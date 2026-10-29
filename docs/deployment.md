# Docker Compose deployment

This runbook covers the Compose-managed API, Caddy HTTPS, optional Stalwart outbound mail, and the VPS deployment workflow. Docker Engine and the Docker Compose plugin are installed and run on the host independently of `mise`; `mise` is installed and invoked only inside the backend image build stage.

## Operator-owned prerequisites

- A VPS with Docker Engine and Docker Compose plugin v2.24 or newer, Git, `curl`, and outbound access to GitHub and image registries. The account used for deployment must be able to access the checked-out repository and run Docker Compose. No distro-specific commands are required.
- A full (not shallow) deploy checkout at `~/logandaim.at` on the VPS. It must have an `origin` remote the deploy account can fetch from and an initial known-good revision deployed before automatic deployments are enabled.
- Public DNS for the API hostname pointing to the VPS; inbound TCP 80/443 (and optional UDP 443 for HTTP/3) for Caddy. This is not an nginx deployment.
- The Pages origin, API hostname, superuser email if applicable, a stable backend signing key, and authoritative DNS console access. These production values are operator-supplied and do not belong in Git.
- Configure the GitHub `production` environment with variables `VPS_HOST`, `VPS_USER`, and `API_DOMAIN`, plus secrets `VPS_SSH_PRIVATE_KEY` and `VPS_SSH_KNOWN_HOSTS`. The custom SSH private key must be restricted to the deployment account; the known-host entry must be obtained and verified out of band, not populated with `ssh-keyscan` during a workflow. Ensure the VPS deploy account can read the repository. Protect the environment with the desired approval gate before enabling the workflow.
- GitHub Actions must also have the existing `LOGANDAIM_API_URL` repository variable for Pages. Pull requests never deploy; only a `main` push after `ci`, `release-check`, and Pages succeeds runs the VPS deployment. Release publication depends on deployment success.

## Manual installation and persistence (AC-1, AC-2, AC-3, AC-5)

From the repository checkout on the VPS:

1. Copy `.env.example` to `.env` and set `API_DOMAIN` to the public API hostname and `FRONTEND_ORIGIN` to the exact Pages/custom-domain origin (scheme included, no path). Optionally set `SUPERUSER_EMAIL`. `.env` is ignored by Git.
2. Create `secrets/signing-key.pem` as a PKCS#8 Ed25519 private key, readable by container UID 10001, and restrict it to the operator and the container runtime. Never commit it. Reusing this file preserves token verification across restarts and redeployments.
3. Create `data/sqlite` and `data/objects` and ensure both are writable by container UID 10001. These bind mounts persist independently of container/image replacement. Create `secrets/` with the signing key; optional SMTP settings go in an ignored `secrets/mail.env` containing only the documented `LOGANDAIM_SMTP_*` variables, which Compose reads if present. Stalwart configuration and state persist in the named `stalwart_etc` and `stalwart_data` volumes if the mail profile is enabled. Caddy's ACME account/certificates persist in the named `caddy_data` volume; keep both named Caddy volumes when operating the stack.
4. Validate and start the app/proxy: `docker compose config`, `docker compose build`, `docker compose up -d`, then `curl --fail https://$API_DOMAIN/api/health`. After this manual deployment is confirmed healthy, initialize the rollback target with `mkdir -p .deploy && git rev-parse HEAD > .deploy/known-good`. Caddy obtains and renews the API certificate automatically through HTTP/TLS validation. Keep ports 80 and 443 available to Caddy.

The API listens on the Compose network at `0.0.0.0:3000`; it is not published directly to the host. Caddy is the only public HTTP(S) entry point and forwards API traffic to the backend. CORS and mailed links use `FRONTEND_ORIGIN`; token issuer uses `API_DOMAIN`.

Hetzner-managed daily VPS backups are the selected backup mechanism (AC-3). Confirm the provider backup includes the VPS disk paths containing `data/sqlite` and `data/objects`, plus Compose named volumes `stalwart_etc`, `stalwart_data`, `caddy_data`, and `caddy_config` (typically under Docker's host data root). App paths here are bind-mounted on the VPS disk; Docker named volumes are also on that disk but must be included by the provider snapshot. No restic sidecar, S3 target, SQLite copy job, or backup retention/RPO is configured by this deployment. Provider snapshots are host-level recovery points, not an application-aware backup guarantee or a tested database/object-store restore. The separate application backup/restore drill and its recovery objectives remain unverified and out of scope here; perform and record a coordinated restore drill before relying on it for application recovery.

## Stalwart mail bootstrap gate (AC-4, #24 AC-6)

Stalwart is for outbound transactional SMTP, not a second public web entry point. Its Compose profile is disabled by default; the only host-published port is its setup UI bound to `127.0.0.1:8080` for an SSH-tunneled bootstrap. SMTP ports are never published to the host. Do not expose or rely on mail until all steps below are complete:

1. Select a CA and inspect its live ACME directory JSON from the VPS/network that will use it. For each CA directory you intend to use (staging and production are separate checks), run `curl --fail --silent "$ACME_DIRECTORY" | jq -e '.meta.accountHashPrefix != null and .meta.issuerDomainNames != null and (.meta.accountHashPrefix | length > 0) and (.meta.issuerDomainNames | length > 0)'`; confirm both properties are present and non-empty. Record the directory URL and observed metadata with the deployment evidence. Do not assume the Let’s Encrypt directory has these fields and do not claim certificate issuance is verified. If either is missing, keep the mail service disabled and unexposed until a compatible CA is confirmed or the PO approves another approach.
2. Only after confirming metadata, bootstrap Stalwart privately: `docker compose --profile mail up -d stalwart`. Keep it isolated to the Compose network; use an SSH tunnel to its setup UI if required. Persist `/etc/stalwart` and `/var/lib/stalwart` as configured in Compose.
3. Configure Stalwart for outbound relay/transactional mail and ACME DNS-Persist-01. Create the ACME account, then use Stalwart to generate the `_validation-persist` TXT value. Manually add the exact value to the authoritative DNS provider console; do not calculate it or automate DNS provider credentials. Verify the authoritative record before requesting a certificate, and leave the static record in DNS for renewals. It delegates issuance to that ACME account; changing ACME accounts requires publishing the value generated for the new account before relying on renewal.
4. Verify certificate issuance and SMTP submission end-to-end, separately from the CA metadata check. Only then set `COMPOSE_PROFILES=mail` in the ignored `.env`, provide `LOGANDAIM_SMTP_HOST=stalwart`, `LOGANDAIM_SMTP_PORT=587`, `LOGANDAIM_SMTP_VERSCHLUESSELUNG=startTls`, sender, username, and password through ignored `secrets/mail.env`, and recreate the backend. Use the actual submission settings chosen in Stalwart. SMTP remains private on the Compose network; this repository does not publish Stalwart mail ports or configure SMTP credentials.

## Automated deployment (AC-1–AC-6)

On an eligible push to `main`, GitHub Actions waits for CI, release checks, and Pages, then connects to the configured VPS over SSH using the production environment's custom private key. The VPS script fetches and checks out the requested full revision, runs `docker compose build` remotely from that source, updates Compose services and checks `https://API_DOMAIN/api/health`. A healthy revision is recorded as `.deploy/known-good`; on failure the script restores the prior known-good revision, rebuilds/starts it, and verifies health before returning failure. The release job is gated on this deployment. PR runs have no deployment job.

Before enabling mail for an automated deployment, the operator must have completed the same ACME metadata and static DNS-Persist-01 record gate described above. The workflow does not open mail ports or enable the Stalwart profile by default. The operator must explicitly enable `COMPOSE_PROFILES=mail` only after satisfying the documented metadata, DNS record, certificate, and SMTP verification steps; subsequent deployments then preserve that operator choice.

## Recovery notes

- Application state is the SQLite bind mount plus object-store bind mount; restore them together from the same host-level recovery point and preserve ownership/permissions. Do not copy a live SQLite database file as an application backup and call it consistent.
- Mail state and Caddy certificate/account state live separately in their own persistent mount/volume. A fresh Caddy volume can trigger new ACME issuance and rate limits; retain it.
- The deployment script rolls back code revision and rebuilds from source; it does not roll back database migrations or restore application data. Verify rollback health and inspect logs with `docker compose logs backend caddy`.
- Never run `docker compose down -v` on production: it removes named Caddy state volumes.
