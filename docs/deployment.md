# Docker Compose deployment

This runbook covers the Compose-managed API, Caddy HTTPS, Stalwart outbound mail, and the VPS deployment workflow. Docker Engine and the Docker Compose plugin are installed and run on the host independently of `mise`; the VPS needs neither `mise` nor a compiler. CI builds the release binary, wraps it in the runtime image, and pushes it to GitHub Container Registry. Deploying pulls that image by commit tag.

## Operator-owned prerequisites

- A VPS with Docker Engine and Docker Compose plugin v2.24 or newer, Git, `curl`, and outbound access to GitHub and `ghcr.io`. The account used for deployment must be able to access the checked-out repository and run Docker Compose. No distro-specific commands are required. The VPS runs no `mise` and no compiler; the deploy task runs on the machine that holds the SSH key.
- The `logandaim-server` package on GHCR must be set to public once, after the first `image` job publishes it. Public is what lets the VPS pull anonymously with no stored registry credentials.
- A full (not shallow) deploy checkout at `/opt/logandaim/repo` on the VPS, with `data/` and `secrets/` beside it at `/opt/logandaim`. It must have an `origin` remote the deploy account can fetch from and an initial known-good revision deployed before automatic deployments are enabled.
- Public DNS for the API hostname pointing to the VPS; inbound TCP 80/443 (and optional UDP 443 for HTTP/3) for Caddy. This is not an nginx deployment.
- The Pages origin, API hostname, superuser email if applicable, and authoritative DNS console access. These production values are operator-supplied and do not belong in Git. There is no signing key to manage: the server generates one per process.
- Configure the GitHub `production` environment with variables `VPS_HOST`, `VPS_USER`, and `API_DOMAIN`, plus secrets `VPS_SSH_PRIVATE_KEY` and `VPS_SSH_KNOWN_HOSTS`. The custom SSH private key must be restricted to the deployment account; the known-host entry must be obtained and verified out of band, not populated with `ssh-keyscan` during a workflow. Ensure the VPS deploy account can read the repository. Protect the environment with the desired approval gate before enabling the workflow.
- GitHub Actions must also have the existing `LOGANDAIM_API_URL` repository variable for Pages. Pull requests never deploy; a `main` push after `pr/base` and `release-check` succeed deploys, and Pages deploys that same export. Release publication depends on deployment success. `workflow_dispatch` runs the identical pipeline from any branch, deploys the same artefacts and skips only release validation and publication, so a pull request can be deployed and tried before it merges; `:main` is not moved by such a run.

## Manual installation and persistence (AC-1, AC-2, AC-3, AC-5)

From the repository checkout on the VPS:

1. Copy `.env.example` to `.env` and set `API_DOMAIN` to the public API hostname, `FRONTEND_ORIGIN` to the exact Pages/custom-domain origin (scheme included, no path), and `MAIL_ABSENDER` to the envelope sender. Optionally set `SUPERUSER_EMAIL`. `.env` is ignored by Git. It belongs at `/opt/logandaim/.env`, beside the checkout.
2. **No signing key is provisioned.** The server generates a fresh Ed25519 key per process and stores each session's public half on its own row, so verification never needs a configured key. Existing sessions keep verifying across a restart: each token is checked against the key its row carries. Nothing logs users out when the process restarts, and there is no key file to mount, rotate, or back up. The one consequence is that a session row's key no longer verifies after its row is deleted — a deleted row is already a 401 by design (AC-17).
3. Create `data/sqlite` and `data/objects` and ensure both are writable by container UID 10001. These bind mounts persist independently of container/image replacement. Create `secrets/mail.env` with the relay credentials. The expected layout is `/opt/logandaim/{data,secrets,repo}`: `data/` and `secrets/` sit beside the checkout, not inside it, and `deploy:vps` exports `LOGANDAIM_HOST_ROOT` so Compose resolves them. Stalwart configuration and state persist in the named `stalwart_etc` and `stalwart_data` volumes. Caddy's ACME account/certificates persist in the named `caddy_data` volume; keep both named Caddy volumes when operating the stack.
4. Start the stack from a published image. The image is public, so the VPS pulls anonymously and stores no registry credentials. From a checkout with `VPS_HOST`, `VPS_USER`, and `API_DOMAIN` set, run `mise run deploy:vps ghcr.io/<owner>/logandaim-server:sha-<commit> <commit>`; that is the same path an automated deployment takes, and it records `.deploy/known-good-image`, `.deploy/known-good-digest`, and `.deploy/known-good-revision` once the health check passes. For a first bootstrap without the task, on the VPS `export LOGANDAIM_IMAGE` to that reference and `export LOGANDAIM_HOST_ROOT=/opt/logandaim`, then from `/opt/logandaim/repo` run `docker compose --env-file /opt/logandaim/.env pull backend` and `docker compose --env-file /opt/logandaim/.env up -d`, then `curl --fail https://$API_DOMAIN/api/health`, and record the rollback target by hand from that reference and `git rev-parse HEAD`. Both flags matter for a manual run: without `LOGANDAIM_HOST_ROOT` Compose looks for `data/` and `secrets/` inside the checkout, and without `--env-file` it reads `.env` from the checkout rather than from `/opt/logandaim`. `deploy:vps` supplies both. Caddy obtains and renews the API certificate automatically through HTTP/TLS validation; keep ports 80 and 443 available to Caddy.

The API listens on the Compose network at `0.0.0.0:3000`; it is not published directly to the host. Caddy is the only public HTTP(S) entry point and forwards API traffic to the backend. CORS and mailed links use `FRONTEND_ORIGIN`; token issuer uses `API_DOMAIN`.

Hetzner-managed daily VPS backups are the selected backup mechanism (AC-3). Confirm the provider backup includes the VPS disk paths containing `data/sqlite` and `data/objects`, plus Compose named volumes `stalwart_etc`, `stalwart_data`, `caddy_data`, and `caddy_config` (typically under Docker's host data root). App paths here are bind-mounted on the VPS disk; Docker named volumes are also on that disk but must be included by the provider snapshot. No restic sidecar, S3 target, SQLite copy job, or backup retention/RPO is configured by this deployment. Provider snapshots are host-level recovery points, not an application-aware backup guarantee or a tested database/object-store restore. The separate application backup/restore drill and its recovery objectives remain unverified and out of scope here; perform and record a coordinated restore drill before relying on it for application recovery.

## Stalwart mail (AC-4, #24 AC-6)

Stalwart is part of the stack, not an optional profile: the MVP sends registration and password
recovery mail, so a deployment without a working relay is not a working deployment. `compose.yaml`
starts it unconditionally and fixes the topology of the hop — `LOGANDAIM_SMTP_HOST=stalwart`,
port `587`, plain SMTP — because a relay host or TLS mode left to the operator drifts from the
service it talks to and only fails at send time.

**Why the hop is plain SMTP.** The backend-to-Stalwart leg never leaves the Compose bridge network.
The submission listener is published on loopback only (`127.0.0.1:587`, reached through an SSH
tunnel), and the backend addresses the service as `stalwart`, a name no CA issues a certificate
for. STARTTLS on that leg would therefore require either a private CA installed in the image or
hostname verification turned off — both more work than the hop is worth. The credentials travel
only over a bridge that no host outside the stack can join. Dropping TLS here does **not** affect
Stalwart's outbound delivery to public mail servers: that TLS is Stalwart's own, configured in
Stalwart's MTA settings, and is unrelated to how the backend submits.

It is still not a public web or SMTP entry point. The only host-published port is its setup UI bound
to `127.0.0.1:8080` for an SSH-tunneled bootstrap; SMTP stays inside the Compose network. A relay
that is unconfigured therefore fails closed: nothing is exposed, and sends fail and are logged rather
than silently delivered over plaintext.

Before mail can be relied on, complete all steps below:

1. Select a CA and inspect its live ACME directory JSON from the VPS/network that will use it. For each CA directory you intend to use (staging and production are separate checks), run `curl --fail --silent "$ACME_DIRECTORY" | jq -e '.meta.accountHashPrefix != null and .meta.issuerDomainNames != null and (.meta.accountHashPrefix | length > 0) and (.meta.issuerDomainNames | length > 0)'`; confirm both properties are present and non-empty. Record the directory URL and observed metadata with the deployment evidence. Do not assume the Let’s Encrypt directory has these fields and do not claim certificate issuance is verified. If either is missing, the relay cannot obtain a certificate and mail delivery must be resolved with the PO before launch.
2. Bootstrap Stalwart privately: `docker compose up -d stalwart`, then tunnel the setup UI (`ssh -L 8080:127.0.0.1:8080`) and complete the wizard. The hostname you enter there is Stalwart's own name for outbound mail; it is not read by the backend and needs no public certificate for the bridge hop. Persist `/etc/stalwart` and `/var/lib/stalwart` as configured in Compose.
3. Configure Stalwart for outbound relay/transactional mail and ACME DNS-Persist-01. Create the ACME account, then use Stalwart to generate the `_validation-persist` TXT value. Manually add the exact value to the authoritative DNS provider console; do not calculate it or automate DNS provider credentials. Verify the authoritative record before requesting a certificate, and leave the static record in DNS for renewals. It delegates issuance to that ACME account; changing ACME accounts requires publishing the value generated for the new account before relying on renewal.
4. Create the submission account for the backend and put only its credentials in the ignored `secrets/mail.env`:

   ```
   LOGANDAIM_SMTP_BENUTZERNAME=<submission account>
   LOGANDAIM_SMTP_PASSWORT=<submission password>
   ```

   `MAIL_ABSENDER` belongs in `.env` beside `API_DOMAIN`. Do not set `LOGANDAIM_SMTP_HOST`, `LOGANDAIM_SMTP_PORT`, or `LOGANDAIM_SMTP_VERSCHLUESSELUNG` anywhere: Compose fixes them, and a stray value only makes the backend disagree with the service beside it. The accepted values are `klar`, `start-tls`, and `implizit`; Compose sets `klar`.
5. Verify SMTP submission end to end. `docker compose logs backend` reports `SMTP-Relay erreichbar; Mailversand ist verfügbar` once authentication on the bridge succeeds, and `deploy:vps` prints the same verdict after the health check. An error line there names the failing step (credentials or reachability) and must be resolved before launch.

**Outbound delivery is separate from submission.** A healthy backend verdict only proves the
backend reached Stalwart. Whether Stalwart can deliver to a public mailbox is a different hop and
shows up only in Stalwart's own logs. Two failures are common on a fresh VPS:

- `Network is unreachable (os error 101)` on an IPv6 target: the host has no working IPv6 route
  while the MX lookup returned an AAAA record. Either fix IPv6 on the host or make Stalwart prefer
  IPv4.
- A connect timeout on port 25 to a large provider's MX: many VPS providers block outbound TCP 25
  by default to limit spam. Ask the provider to unblock it, or relay outbound mail through a
  smarthost the provider allows. The backend's own submission port is unaffected.

**Stalwart is mandatory as of the MVP.** Reverting to a private pilot without mail means the relay stops being started, and Compose must then also stop declaring the SMTP variables, because the backend would otherwise point at a service that does not exist.

## Automated deployment (AC-1–AC-6)

On an eligible push to `main`, `pr/base` has already compiled the release profile for that revision and uploaded two artefacts: the server binary and the static export. `deploy/pages` deploys the export as built. `deploy/image` wraps that binary in the runtime image and pushes it to `ghcr.io/<owner>/logandaim-server`, tagged `sha-<full commit>` and, for humans browsing the registry, `main`. `deploy/backend` then calls `mise run deploy:vps <reference> <commit>`, the same task an operator runs. It validates that the reference is `ghcr.io/<owner>/logandaim-server:sha-<commit>` and that the tag matches the revision, streams the remote half over SSH, and there fetches every branch, checks out that revision, pulls the image, applies the services, and checks `https://API_DOMAIN/api/health`. Nothing compiles on the VPS or in the deploy step: what runs is the binary the gate tested, addressed by the commit it was built from. `deploy/release` is gated on both deployments. PR runs have no deployment job.

A `workflow_dispatch` run takes exactly this path from whatever branch it is dispatched on and skips only `release-check` and `deploy/release`, so a pull request can be deployed and exercised before it merges. Nothing restricts the revision to `main`; the tag-and-revision agreement is what keeps a deployment self-consistent. Only a `main` push moves the human-facing `:main` tag.

The only reference any consumer uses is the `sha-<commit>` tag. `:main` is pushed for humans and nothing pulls it; `deploy:vps` rejects it, as it rejects a digest reference and a short sha. A `sha-<commit>` tag is written once per commit and never re-pointed, so the recorded rollback target names an exact revision.

Before enabling mail for an automated deployment, the operator must have completed the same ACME metadata and static DNS-Persist-01 record gate described above. The workflow does not open mail ports. Stalwart starts with the rest of the stack, and the deploy output reports whether the hop works; a deployment whose relay is not configured yet still ships the image and health-checked API, but reports the missing mail verdict as a warning rather than passing silently.

## Recovery notes

- Application state is the SQLite bind mount plus object-store bind mount; restore them together from the same host-level recovery point and preserve ownership/permissions. Do not copy a live SQLite database file as an application backup and call it consistent.
- Mail state and Caddy certificate/account state live separately in their own persistent mount/volume. A fresh Caddy volume can trigger new ACME issuance and rate limits; retain it.
- The deploy task rolls back the image and the checked-out revision; it does not roll back database migrations or restore application data. Verify rollback health and inspect logs with `docker compose logs backend caddy`.
- Rolling back needs `.deploy/known-good-image`; a deployment that fails before one exists reports that no rollback target is recorded instead of starting an empty image. To roll back by hand, read that file and run `mise run deploy:vps "$(cat .deploy/known-good-image)" "$(cat .deploy/known-good-revision)"` with the production values set, or on the VPS set `LOGANDAIM_IMAGE` to the recorded reference and run `docker compose pull backend && docker compose up -d backend`.
- `.deploy/known-good-digest` records the registry digest the tag resolved to. It is a tripwire, not authentication: if a `sha-<commit>` tag were ever re-pushed, the digest would change and be visible there on the next deploy.
- A local stack builds from this checkout with the override: `mise run build:server`, then `docker compose -f compose.yaml -f compose.build.yaml up -d --build`. The deployment never passes `compose.build.yaml`, so a deploy can pull but cannot build.
- Never run `docker compose down -v` on production: it removes named Caddy state volumes.
