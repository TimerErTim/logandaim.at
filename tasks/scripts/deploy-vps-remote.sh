#!/usr/bin/env bash
set -Eeuo pipefail

# Remote half of `deploy:vps`, fed to `bash -s` over SSH stdin by the local half.
#
# Nothing here compiles. The image was built and pushed by CI around the release
# binary `pr/base` tested, so the VPS only pulls it and replaces the container.
# `.deploy/known-good-image` records the reference that last passed a health check;
# a failed deployment re-applies it and verifies health before returning failure.
#
# The signing key, `.env`, and the data directories must already exist: the server
# needs them to start, and a deployment must not invent production secrets.
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

# Compose reads `.env` on every invocation, including the one inside `rollback`, so
# the operator-owned state is checked before the trap can fire.
test -f .env || { echo "Missing .env: copy .env.example and fill it in." >&2; exit 2; }
grep -Eq '^FRONTEND_ORIGIN=.+' .env || { echo "Missing FRONTEND_ORIGIN in .env." >&2; exit 2; }
grep -Eq '^MAIL_HOSTNAME=.+' .env || { echo "Missing MAIL_HOSTNAME in .env." >&2; exit 2; }
grep -Eq '^MAIL_ABSENDER=.+' .env || { echo "Missing MAIL_ABSENDER in .env." >&2; exit 2; }
# Mail is part of the MVP: a stack without working relay credentials is not deployable,
# so this fails here rather than at the first registration that never sends.
test -f secrets/mail.env || { echo "Missing secrets/mail.env with the SMTP credentials." >&2; exit 2; }
grep -Eq '^LOGANDAIM_SMTP_BENUTZERNAME=.+' secrets/mail.env ||
  { echo "Missing LOGANDAIM_SMTP_BENUTZERNAME in secrets/mail.env." >&2; exit 2; }
grep -Eq '^LOGANDAIM_SMTP_PASSWORT=.+' secrets/mail.env ||
  { echo "Missing LOGANDAIM_SMTP_PASSWORT in secrets/mail.env." >&2; exit 2; }
test -s secrets/signing-key.pem || { echo "Missing secrets/signing-key.pem." >&2; exit 2; }
test -d data/sqlite || { echo "Missing data/sqlite." >&2; exit 2; }
test -d data/objects || { echo "Missing data/objects." >&2; exit 2; }
export API_DOMAIN="$api_domain"
# Exported rather than written to `.env`: the requested reference must win over any
# value an operator left behind, and compose reads the environment first.
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
apply() {
  docker compose pull backend
  docker compose up -d --remove-orphans
}

# `--since` scopes the read to this boot: a verdict from an earlier one would satisfy
# the check for a container that never reached the relay.
mail_check() {
  local container="" started_at="" _
  local seit=()
  container="$(docker compose ps -q backend 2>/dev/null || true)"
  if [[ -n "$container" ]]; then
    started_at="$(docker inspect --format '{{.State.StartedAt}}' "$container" 2>/dev/null || true)"
  fi
  [[ -n "$started_at" ]] && seit=(--since "$started_at")
  for _ in $(seq 1 20); do
    if docker compose logs --no-color "${seit[@]}" backend 2>/dev/null |
      grep -q "Mailversand ist verfügbar"; then
      echo "Mailversand verfügbar (SMTP-Relay erreichbar)."
      return 0
    fi
    sleep 5
  done
  echo "WARNUNG: Mailversand nicht bestätigt; siehe 'docker compose logs backend'." >&2
  return 1
}

# The digest the registry served for this tag. It is recorded beside the tag so a
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
record_known_good
echo "Deployed healthy revision ${revision} (${image})."
mail_check || true

# Old images accumulate on a small disk: every deploy pulls a new immutable tag.
# `-a` also removes tags that no container runs any more, so an old image is not
# kept forever just because it has a name. Rollback re-pulls its reference, so
# removing it here costs a pull, not the rollback itself. Only after a passing
# health check: a failed deployment keeps its image so the failure can be inspected.
docker image prune --all --force --filter until=168h >/dev/null
