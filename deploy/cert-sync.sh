#!/bin/bash
# Publishes Caddy's current certificate for the API hostname at a stable path the mail
# server can read.
#
# Caddy keeps certificates inside its data volume under a path that names the ACME
# directory (`.../acme-v02.api.letsencrypt.org-directory/<host>/<host>.crt`). That path is
# an implementation detail: it moves with the Caddy version and changes outright when
# staging is used. This sidecar therefore discovers the certificate by hostname instead of
# hardcoding it, and writes a copy to a fixed path.
#
# The copy is what lets the mail server's TLS setup stay indifferent to Caddy's storage
# layout. docker-mailserver's changedetector polls `SSL_CERT_PATH` and `SSL_KEY_PATH`
# (2 s by default), re-copies them to its internal `/etc/dms/tls/`, and reloads Postfix
# and Dovecot. So a renewal reaches the listeners without either container being
# restarted, and no hook or restart is needed here.
#
# Publishing is a rename into a directory that is mounted as a whole, never a write to a
# file that is bind-mounted on its own: a single-file bind mount would keep pointing at
# the old inode after the rename, and the mail server would never see the new pair.
set -eu

caddy_data="${CADDY_DATA_DIR:-/caddy-data}"
target_dir="${CERT_TARGET_DIR:-/certs}"
domain="${API_DOMAIN:?API_DOMAIN must be set}"

cert_target="$target_dir/active.crt"
key_target="$target_dir/active.key"

# Caddy writes the pair together, but guard anyway: a half-written directory would
# otherwise publish a key that does not match the certificate.
# `find` runs against Caddy's data volume, which may not exist yet on the very first boot;
# a missing directory is not an error, it is "no certificate yet".
find_cert() {
  if [ ! -d "$caddy_data" ]; then
    return 0
  fi
  find "$caddy_data" -type f -name '*.crt' -path "*/${domain}/*" 2>/dev/null | head -n 1
}

publish() {
  local cert key
  cert="$(find_cert)"
  if [ -z "$cert" ]; then
    # Deliberately keeps the last published pair. Caddy owning no certificate is a
    # transient state (a removed storage volume, a re-issuance in flight), and retracting
    # would take the mail server's listeners down for it. The published pair keeps working
    # until it expires, which is long enough for the operator to notice.
    echo "cert-sync: no certificate for ${domain} under ${caddy_data} yet" >&2
    return 1
  fi

  key="${cert%.crt}.key"
  if [ ! -s "$key" ]; then
    echo "cert-sync: certificate ${cert} has no matching key at ${key}" >&2
    return 1
  fi

  # Nothing to do when the published pair already matches; this keeps the mail server
  # from reloading on every pass.
  if [ -s "$cert_target" ] && [ -s "$key_target" ] &&
    cmp -s "$cert" "$cert_target" && cmp -s "$key" "$key_target"; then
    return 0
  fi

  # Copy beside the target and rename, so a reader never sees a partial file. The key is
  # published with a restrictive mode but must stay readable by the mail server, which
  # runs as root.
  cp "$cert" "$cert_target.tmp"
  cp "$key" "$key_target.tmp"
  chmod 0644 "$cert_target.tmp"
  chmod 0600 "$key_target.tmp"
  mv "$cert_target.tmp" "$cert_target"
  mv "$key_target.tmp" "$key_target"
  echo "cert-sync: published ${cert} as ${cert_target}"
}

mkdir -p "$target_dir"

# Publish once immediately so `depends_on: service_healthy` can release the mail server,
# then keep watching. Five minutes is cheap (one `find`) and keeps the window between a
# renewal and the mail server reloading short; six hours is far inside Caddy's renewal
# cycle but would leave a stale certificate for most of a day after a renewal.
while :; do
  publish || true
  sleep 300
done
