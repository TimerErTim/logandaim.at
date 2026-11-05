#!/usr/bin/env bash
set -Eeuo pipefail

# Remote half of `deploy:vps`, fed to `bash -s` over SSH stdin by the local half.
#
# Nothing here compiles. The image was built and pushed by CI around the release
# binary `pr/base` tested, so the VPS only pulls it and replaces the container.
# `.deploy/known-good-image` records the reference that last passed a health check;
# a failed deployment re-applies it and verifies health before returning failure.
#
# `.env`, the mail credentials, and the data directories must already exist: the server
# needs them to start, and a deployment must not invent production secrets. There is no
# signing key file to check: the server generates its Ed25519 key per process.
#
# Arguments arrive as `bash -s --` words, never interpolated into a command:
#   1 image reference   2 revision   3 api domain   4 repository directory

image="${1:?usage: deploy-vps-remote.sh <image> <revision> <api-domain> <dir>}"
revision="${2:?usage: deploy-vps-remote.sh <image> <revision> <api-domain> <dir>}"
api_domain="${3:?usage: deploy-vps-remote.sh <image> <revision> <api-domain> <dir>}"
deploy_dir="${4:?usage: deploy-vps-remote.sh <image> <revision> <api-domain> <dir>}"

[[ "$image" =~ ^ghcr\.io/[a-z0-9._-]+/logandaim-server:sha-[0-9a-f]{40}$ ]] ||
  { echo "Invalid image reference" >&2; exit 2; }
[[ "$revision" =~ ^[0-9a-f]{40}$ ]] || { echo "Invalid revision" >&2; exit 2; }
[[ "${image##*:sha-}" == "$revision" ]] ||
  { echo "Image tag and revision disagree" >&2; exit 2; }
[[ "$api_domain" =~ ^[A-Za-z0-9.-]+$ ]] || { echo "Invalid API domain" >&2; exit 2; }

# `~` from the caller is a local shell convenience, not a remote one.
deploy_dir="${deploy_dir/#\~/$HOME}"
[[ "$deploy_dir" = /* ]] || { echo "Deploy directory must be absolute" >&2; exit 2; }
cd "$deploy_dir" || { echo "Missing deploy checkout at $deploy_dir" >&2; exit 2; }

repo="$(git rev-parse --show-toplevel)"
cd "$repo"
mkdir -p .deploy
known_good_image=.deploy/known-good-image
known_good_digest=.deploy/known-good-digest
known_good_revision=.deploy/known-good-revision

has_previous=false
[[ -s "$known_good_image" ]] && has_previous=true

# Persistent state and operator secrets sit beside the checkout, not inside it:
# `/opt/logandaim/data`, `/opt/logandaim/secrets`, checkout at `/opt/logandaim/repo`.
# Compose resolves relative bind mounts against the compose file's directory (the repo),
# so the sources are addressed through this explicit host root instead.
host_root="$(cd "$deploy_dir/.." && pwd)"

# Compose reads `.env` from the directory it runs in — the checkout — on every
# invocation, including the one inside `rollback`, so the operator-owned state is checked
# before the trap can fire. The VPS keeps its secrets beside the checkout, so prefer
# `$host_root/.env`; a checkout that still carries its own `.env` keeps working.
if [[ -f "$host_root/.env" ]]; then
  env_file="$host_root/.env"
else
  env_file=.env
fi
test -f "$env_file" || { echo "Missing .env: copy .env.example and fill it in." >&2; exit 2; }
grep -Eq '^FRONTEND_ORIGIN=.+' "$env_file" ||
  { echo "Missing FRONTEND_ORIGIN in $env_file." >&2; exit 2; }
# MAIL_ABSENDER stays required: every transactional mail carries it. There is no
# MAIL_HOSTNAME: the mail server's own name is API_DOMAIN, so a second hostname in `.env`
# would be one more value that can disagree with the certificate.
grep -Eq '^MAIL_ABSENDER=.+' "$env_file" ||
  { echo "Missing MAIL_ABSENDER in $env_file." >&2; exit 2; }
# Mail is part of the MVP: a stack without working relay credentials is not deployable,
# so this fails here rather than at the first registration that never sends.
mail_env="$host_root/secrets/mail.env"
test -f "$mail_env" || { echo "Missing $mail_env with the SMTP credentials." >&2; exit 2; }
grep -Eq '^LOGANDAIM_SMTP_BENUTZERNAME=.+' "$mail_env" ||
  { echo "Missing LOGANDAIM_SMTP_BENUTZERNAME in $mail_env." >&2; exit 2; }
grep -Eq '^LOGANDAIM_SMTP_PASSWORT=.+' "$mail_env" ||
  { echo "Missing LOGANDAIM_SMTP_PASSWORT in $mail_env." >&2; exit 2; }
test -d "$host_root/data/sqlite" || { echo "Missing $host_root/data/sqlite." >&2; exit 2; }
test -d "$host_root/data/objects" || { echo "Missing $host_root/data/objects." >&2; exit 2; }
export LOGANDAIM_HOST_ROOT="$host_root"
export API_DOMAIN="$api_domain"
# Exported so the requested reference wins over the value `.env` holds from the last
# deployment; `persist_env` writes it back once it is running.
export LOGANDAIM_IMAGE="$image"
curl --version >/dev/null

health_check() {
  local _
  for _ in $(seq 1 30); do
    if curl --fail --silent --show-error --connect-timeout 3 \
      --resolve "${api_domain}:443:127.0.0.1" "https://${api_domain}/api/health" | grep -qx '"OK"'; then
      return 0
    fi
    sleep 5
  done
  return 1
}

# `up` never builds: this checkout has no binary in `.deploy/incoming`.
#
# `--env-file` is explicit rather than left to Compose's default lookup: when `.env` lives
# beside the checkout, Compose would not see it and every required variable would render
# as unset. It reads the same file the checks above validated.
apply() {
  docker compose --env-file "$env_file" pull backend
  docker compose --env-file "$env_file" up -d --remove-orphans
  persist_env
}

# A plain `docker compose up -d` in the checkout must start what is running now. Compose
# reads `.env` from the project directory, so the checkout's `.env` links to the operator's
# file, and that file records the started image and the host root. Rollback calls `apply`
# too, so the record always names the image that was started last.
persist_env() {
  local tmp="$env_file.tmp"
  grep -Ev '^(LOGANDAIM_IMAGE|LOGANDAIM_HOST_ROOT)=' "$env_file" > "$tmp" || true
  printf 'LOGANDAIM_IMAGE=%s\nLOGANDAIM_HOST_ROOT=%s\n' "$LOGANDAIM_IMAGE" "$host_root" >> "$tmp"
  chmod --reference="$env_file" "$tmp"
  mv "$tmp" "$env_file"
  if [[ "$env_file" != .env ]]; then
    ln -sfn "$env_file" .env
  fi
}

# Ask the API for its mail verdict, the one the container healthcheck reads. The probe logs
# in on the submission listener, so a wrong password or a broken certificate shows up here.
#
# Only `Ok` passes. `Unknown` and `Failed` are polled again: each read of a non-`Ok` verdict
# starts a fresh probe, so a relay that is still starting gets the whole window.
mail_check() {
  local body="" _
  for _ in $(seq 1 30); do
    if ! body="$(curl --fail --silent --show-error --connect-timeout 3 \
      --resolve "${api_domain}:443:127.0.0.1" "https://${api_domain}/api/health/mail" 2>/dev/null)"; then
      sleep 5
      continue
    fi
    if printf '%s' "$body" | grep -q '"status":"Ok"'; then
      echo "Mailversand verfügbar (SMTP-Relay erreichbar)."
      return 0
    fi
    sleep 5
  done
  echo "Mailversand nicht bestätigt: ${body:-keine Antwort}" >&2
  return 1
}
# tag that was re-pushed out of band is visible in the recorded state, and so a
# rollback reference can be compared against the local image store.
pulled_digest() {
  docker image inspect --format '{{index .RepoDigests 0}}' "$image" 2>/dev/null || true
}

record_known_good() {
  local digest
  digest="$(pulled_digest)"
  [[ -n "$digest" ]] || { echo "Cannot read the digest of $image" >&2; return 1; }
  printf '%s\n' "$image" > "$known_good_image.tmp"
  printf '%s\n' "$digest" > "$known_good_digest.tmp"
  printf '%s\n' "$revision" > "$known_good_revision.tmp"
  mv "$known_good_image.tmp" "$known_good_image"
  mv "$known_good_digest.tmp" "$known_good_digest"
  mv "$known_good_revision.tmp" "$known_good_revision"
}

rollback() {
  local status=$?
  trap - ERR
  if [[ "$has_previous" != true ]]; then
    echo "Deployment failed; no previously healthy image is recorded for rollback." >&2
    exit "$status"
  fi
  local previous
  previous="$(cat "$known_good_image")"
  echo "Deployment failed; starting the last known-good image ${previous}." >&2
  export LOGANDAIM_IMAGE="$previous"
  apply
  if ! health_check; then
    echo "Rollback also failed health check." >&2
    exit 1
  fi
  echo "Rollback healthy." >&2
  exit "$status"
}
trap rollback ERR

# Any published revision is deployable, including one from a branch, so a pull request
# can be tried before it merges. Fetch all refs first: the deploy account's checkout may
# not have the branch yet. The reference itself is the authority — an unknown commit
# fails here rather than as a confusing compose error later.
git fetch --no-tags origin '+refs/heads/*:refs/remotes/origin/*' ||
  { echo "Cannot fetch from origin." >&2; exit 2; }
git cat-file -e "${revision}^{commit}" 2>/dev/null || {
  echo "Deployment revision ${revision} is not a commit known to this checkout." >&2
  exit 2
}

# The composition comes from the revision being deployed: check out before the
# image is pulled, so `compose.yaml` and the Caddyfile match the image being started.
git checkout --detach "$revision"

apply
health_check
# Before recording: an unverified relay must not be written down as known-good, because
# the rollback target would then be a revision whose mail never worked. The `ERR` trap
# turns a failure here into the normal rollback path.
mail_check
record_known_good
echo "Deployed healthy revision ${revision} (${image})."

# Old images accumulate on a small disk: every deploy pulls a new immutable tag.
# `-a` also removes tags that no container runs any more, so an old image is not
# kept forever just because it has a name. Rollback re-pulls its reference, so
# removing it here costs a pull, not the rollback itself. Only after a passing
# health check: a failed deployment keeps its image so the failure can be inspected.
docker image prune --all --force --filter until=168h >/dev/null
