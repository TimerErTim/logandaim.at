#!/usr/bin/env bash
set -Eeuo pipefail

# Local half of `deploy:vps`. Validates the request, then streams the remote half
# into the VPS over SSH stdin.
#
# Running the remote half from stdin is what keeps this workable: the checkout on
# the VPS rewrites the worktree, and a script read from the filesystem there could
# be executed as a mix of two revisions. A heredoc-style stdin script is already in
# memory, so the old re-exec guard is unnecessary.

image="${usage_image:?missing <image>}"
revision="${usage_revision:?missing <revision>}"
host="${usage_host:?set --host or VPS_HOST}"
user="${usage_user:?set --user or VPS_USER}"
api_domain="${usage_api_domain:?set --api-domain or API_DOMAIN}"
# `~` is expanded by the remote shell, not here: the runner's home is not the VPS's.
# The VPS layout keeps the checkout at `/opt/logandaim/repo` with `data/` and `secrets/`
# beside it, so the checkout and the host root are two different directories.
deploy_dir="${usage_dir:-/opt/logandaim/repo}"
identity="${usage_identity:-$HOME/.ssh/id_ed25519}"

fail() {
  echo "deploy:vps: $1" >&2
  exit 2
}

# One reference format, everywhere: the commit tag. `:main` is pushed for humans
# browsing the registry and is never a valid deployment input, and a digest would
# name an image whose revision the VPS cannot check out.
[[ "$image" =~ ^ghcr\.io/[a-z0-9._-]+/logandaim-server:sha-[0-9a-f]{40}$ ]] ||
  fail "'$image' is not ghcr.io/<owner>/logandaim-server:sha-<commit>"
[[ "$revision" =~ ^[0-9a-f]{40}$ ]] || fail "'$revision' is not a full commit sha"

# The tag and the revision must describe the same build: checking out one revision
# and running an image from another would deploy a composition that does not belong
# to the binary.
[[ "${image##*:sha-}" == "$revision" ]] ||
  fail "image tag '${image##*:sha-}' does not match revision '$revision'"

[[ "$host" =~ ^[A-Za-z0-9.-]+$ ]] || fail "'$host' is not a hostname or IPv4 address"
[[ "$user" =~ ^[a-z_][a-z0-9_-]*$ ]] || fail "'$user' is not a Unix account name"
[[ "$api_domain" =~ ^[A-Za-z0-9.-]+$ ]] || fail "'$api_domain' is not a DNS hostname"
[[ -f "$identity" ]] || fail "no SSH key at '$identity'"
# The values are embedded in the remote command words below, so the path must not be
# able to contribute shell syntax. Only path characters are accepted.
[[ "$deploy_dir" =~ ^[A-Za-z0-9_./~-]+$ ]] || fail "unsafe --dir '$deploy_dir'"

remote_script="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/deploy-vps-remote.sh"
[[ -f "$remote_script" ]] || fail "missing $remote_script"

# The remote arguments travel as separate `bash -s --` words, so the script needs no
# shell quoting of its own and a value can never be interpreted as code.
ssh -i "$identity" -o IdentitiesOnly=yes "$user@$host" \
  "bash -s -- '$image' '$revision' '$api_domain' '$deploy_dir'" \
  < "$remote_script"
