#!/usr/bin/env bash
set -Eeuo pipefail

# Deploy a revision whose binary `pr/base` already built and tested.
#
# Nothing here compiles. The binary arrives as `.deploy/incoming/logandaim-server`
# and `docker compose build` only wraps it in a runtime image, so what runs is the
# artefact the gate tested. `.deploy/known-good-server` keeps the last binary that
# passed a health check; a failed deployment promotes it back and verifies it.
# Rollback therefore needs no source tree and no rebuild.
#
# The signing key, `.env`, and the data directories must already exist: the server
# needs them to start, and a deployment must not invent production secrets.

# This script checks out a revision, which rewrites the file bash is reading. Bash
# reads a script incrementally, so a checkout mid-run can make it execute a mix of
# two versions. Re-exec from a private copy first, and the checkout cannot touch
# the bytes being run.
if [[ "${LOGANDAIM_DEPLOY_REEXEC:-}" != "1" ]]; then
  # Absolute: the checkout below changes what a relative path resolves to, and a
  # relative `BASH_SOURCE` would copy the wrong file back into the private copy.
  self_source="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
  self="$(mktemp)"
  cp "$self_source" "$self"
  chmod 0700 "$self"
  LOGANDAIM_DEPLOY_REEXEC=1 exec "$self" "$@"
fi

revision="${1:?usage: deploy-vps.sh <revision> <api-domain> <binary>}"
api_domain="${2:?usage: deploy-vps.sh <revision> <api-domain> <binary>}"
incoming="${3:?usage: deploy-vps.sh <revision> <api-domain> <binary>}"
[[ "$revision" =~ ^[0-9a-f]{40}$ ]] || { echo "Invalid revision" >&2; exit 2; }
[[ "$api_domain" =~ ^[A-Za-z0-9.-]+$ ]] || { echo "Invalid API domain" >&2; exit 2; }
[[ -f "$incoming" && -s "$incoming" ]] || { echo "Missing incoming binary: $incoming" >&2; exit 2; }

repo="$(git rev-parse --show-toplevel)"
cd "$repo"
mkdir -p .deploy/incoming
server=.deploy/incoming/logandaim-server
known_good_server=.deploy/known-good-server
known_good_revision=.deploy/known-good-revision

has_previous=false
[[ -s "$known_good_server" ]] && has_previous=true

# Compose reads `.env` on every invocation, including the one inside `rollback`, so
# the operator-owned state is checked before the trap can fire.
test -f .env || { echo "Missing .env: copy .env.example and fill it in." >&2; exit 2; }
grep -Eq '^FRONTEND_ORIGIN=.+' .env || { echo "Missing FRONTEND_ORIGIN in .env." >&2; exit 2; }
test -s secrets/signing-key.pem || { echo "Missing secrets/signing-key.pem." >&2; exit 2; }
test -d data/sqlite || { echo "Missing data/sqlite." >&2; exit 2; }
test -d data/objects || { echo "Missing data/objects." >&2; exit 2; }
export API_DOMAIN="$api_domain"
curl --version >/dev/null

health_check() {
  local attempt
  for attempt in $(seq 1 30); do
    if curl --fail --silent --show-error --connect-timeout 3 \
      --resolve "${api_domain}:443:127.0.0.1" "https://${api_domain}/api/health" | grep -qx '"OK"'; then
      return 0
    fi
    sleep 5
  done
  return 1
}

# Wrap whatever binary sits at `$server` in the runtime image and apply it.
apply() {
  BUILDKIT_PROGRESS=plain docker compose build
  docker compose up -d --remove-orphans
}

rollback() {
  local status=$?
  trap - ERR
  if [[ "$has_previous" != true ]]; then
    echo "Deployment failed; no previously healthy binary is recorded for rollback." >&2
    exit "$status"
  fi
  echo "Deployment failed; starting the last known-good binary." >&2
  install -m 0755 "$known_good_server" "$server"
  apply
  if ! health_check; then
    echo "Rollback also failed health check." >&2
    exit 1
  fi
  echo "Rollback healthy." >&2
  exit "$status"
}
trap rollback ERR

# The revision must belong to `main`. The workflow can only ask for one it built,
# but an operator can call this script too and must not deploy a stray commit.
git fetch --no-tags origin main
git cat-file -e "${revision}^{commit}"
if ! git merge-base --is-ancestor "$revision" origin/main; then
  echo "Requested deployment revision is not reachable from origin/main." >&2
  exit 2
fi

# The composition comes from the revision being deployed, while this script keeps
# running from the bytes the shell already read: check out before the binary is
# staged, so `compose.yaml` and the Caddyfile match the image being started.
git checkout --detach "$revision"

# `scp` and the artifact archive preserve no executable bit; the image copy reads
# the mode from the file, so restore it rather than trusting the transport.
if [[ "$incoming" != "$server" ]]; then
  install -m 0755 "$incoming" "$server"
fi
chmod 0755 "$server"

apply
health_check
# Promote only after the health check: a failed deployment leaves the recorded
# binary untouched, so the next failure still returns to what served traffic.
install -m 0755 "$server" "$known_good_server"
printf '%s\n' "$revision" > "$known_good_revision.tmp"
mv "$known_good_revision.tmp" "$known_good_revision"
echo "Deployed healthy revision ${revision}."
