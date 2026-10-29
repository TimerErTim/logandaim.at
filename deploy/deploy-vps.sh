#!/usr/bin/env bash
set -Eeuo pipefail

revision="${1:?usage: deploy-vps.sh <revision> [api-domain]}"
api_domain="${2:?usage: deploy-vps.sh <revision> <api-domain>}"
[[ "$revision" =~ ^[0-9a-f]{40}$ ]] || { echo "Invalid revision" >&2; exit 2; }
[[ "$api_domain" =~ ^[A-Za-z0-9.-]+$ ]] || { echo "Invalid API domain" >&2; exit 2; }

repo="$(git rev-parse --show-toplevel)"
cd "$repo"
mkdir -p .deploy
known_good=.deploy/known-good
previous=""
[[ -s "$known_good" ]] && previous="$(<"$known_good")"
[[ -z "$previous" || "$previous" =~ ^[0-9a-f]{40}$ ]] || { echo "Invalid rollback revision" >&2; exit 2; }

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

rollback() {
  local status=$?
  trap - ERR
  if [[ -z "$previous" ]]; then
    echo "Deployment failed; no previous healthy revision is recorded for rollback." >&2
    exit "$status"
  fi
  echo "Deployment failed; rolling back to ${previous}." >&2
  git checkout --detach "$previous"
  docker compose build
  docker compose up -d --remove-orphans
  if ! health_check; then
    echo "Rollback to ${previous} also failed health check." >&2
    exit 1
  fi
  echo "Rollback healthy at ${previous}." >&2
  exit "$status"
}
trap rollback ERR

test -f .env
test -s secrets/signing-key.pem
test -d data/sqlite
test -d data/objects
export API_DOMAIN="$api_domain"
curl --version >/dev/null
git fetch --no-tags origin main
git cat-file -e "${revision}^{commit}"
if ! git merge-base --is-ancestor "$revision" origin/main; then
  echo "Requested deployment revision is not reachable from origin/main." >&2
  exit 2
fi
git checkout --detach "$revision"
docker compose build
docker compose up -d --remove-orphans
health_check
git rev-parse HEAD > "$known_good.tmp"
mv "$known_good.tmp" "$known_good"
echo "Deployed healthy revision $(git rev-parse HEAD)."
