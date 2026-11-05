#!/usr/bin/env bash
#
# Bootstrap the mail server on the VPS: one submission account, and a DKIM key per domain.
#
# Run this on the VPS as the deploy account, from anywhere:
#
#   bash tasks/scripts/mail-bootstrap.sh
#
# It is an operator script, never a CI step: creating a mailbox and a signing key is a
# one-time decision, and running it again must not rotate a DKIM key that is already in
# DNS (that would break signatures until the new record propagates).
#
# What it does NOT do, deliberately: it does not touch DNS. It prints the exact records to
# add, because the authoritative record is added by hand — a script that rewrote public
# DNS on every run is one mistake away from a domain-wide outage.
set -Eeuo pipefail

# The checkout holds `compose.yaml`; `data/` and `secrets/` sit beside it.
deploy_dir="${VPS_DEPLOY_DIR:-/opt/logandaim/repo}"
host_root="$(cd "$deploy_dir/.." && pwd)"
env_file="$host_root/.env"
mail_env="$host_root/secrets/mail.env"

[[ -f "$env_file" ]] || { echo "Missing $env_file" >&2; exit 2; }
[[ -f "$mail_env" ]] || { echo "Missing $mail_env" >&2; exit 2; }

# Read a single value without executing the file, so an operator-local shell syntax in it
# cannot run here either.
read_value() {
  local key="$1" file="$2"
  sed -n "s/^${key}=//p" "$file" | tail -n 1
}

api_domain="$(read_value API_DOMAIN "$env_file")"
[[ -n "$api_domain" ]] || { echo "API_DOMAIN is empty in $env_file" >&2; exit 2; }

# The mail domain is the FQDN with its first label dropped, exactly how docker-mailserver
# derives it. Keeping one hostname only works because both agree on this.
mail_domain="${api_domain#*.}"
sender="${MAIL_ABSENDER:-$(read_value MAIL_ABSENDER "$env_file")}"
user="$(read_value LOGANDAIM_SMTP_BENUTZERNAME "$mail_env")"
password="$(read_value LOGANDAIM_SMTP_PASSWORT "$mail_env")"
[[ -n "$user" ]] || { echo "Missing LOGANDAIM_SMTP_BENUTZERNAME in $mail_env" >&2; exit 2; }
[[ -n "$password" ]] || { echo "Missing LOGANDAIM_SMTP_PASSWORT in $mail_env" >&2; exit 2; }

cd "$deploy_dir"
# `--env-file` only feeds interpolation; the backend reads `mail.env` through its own
# `env_file`, which resolves against the host root.
export LOGANDAIM_HOST_ROOT="$host_root"
compose=(docker compose --env-file "$env_file")

echo "== Hostnames"
echo "   mail server FQDN : ${api_domain}"
echo "   mail domain      : ${mail_domain}"
echo "   submission user  : ${user}"
echo "   envelope sender  : ${sender:-<unset>}"
echo

# The account must exist before a key can be generated, and the container must be running
# for either. Starting it here is safe: Caddy has already obtained the certificate, because
# the mail server depends on the certificate copy sidecar.
"${compose[@]}" up -d mailserver

echo "== Submission account"
if "${compose[@]}" exec -T mailserver setup email list 2>/dev/null | grep -Fxq "* ${user}"; then
  echo "   ${user} already exists; left untouched (its password is not read back)."
  echo "   To change it: docker compose exec mailserver setup email update ${user} <new-password>"
else
  "${compose[@]}" exec -T mailserver setup email add "$user" "$password" >/dev/null
  echo "   created ${user}"
fi

echo
echo "== DKIM key"
# An existing key is reused, never regenerated: its public half is already in DNS, and a
# rotation has to be a deliberate act with a new selector.
key_file="/tmp/docker-mailserver/opendkim/keys/${mail_domain}/mail.txt"
if "${compose[@]}" exec -T mailserver test -s "$key_file" 2>/dev/null; then
  echo "   key for ${mail_domain} already exists; left untouched."
else
  "${compose[@]}" exec -T mailserver setup config dkim domain "$mail_domain"
  echo "   generated a key for ${mail_domain} (selector: mail)"
  echo "   restart the container so outgoing mail is signed with it:"
  echo "     ${compose[*]} restart mailserver"
fi

echo
echo "== DNS records to publish (add by hand; this script never touches DNS)"
echo
echo "1. DKIM — TXT on mail._domainkey.${mail_domain}"
echo "   Value (one line, quotes and whitespace between parts removed):"
"${compose[@]}" exec -T mailserver cat "$key_file" 2>/dev/null |
  sed -n 's/^[^(]*(\(.*\))$/\1/p' |
  tr -d '"' | tr -d '\n' | tr -s ' ' | sed 's/^ *//; s/ *$//' |
  sed 's/^/     /'
echo
echo "2. SPF — TXT on ${mail_domain}"
echo "     v=spf1 mx ~all"
echo "   Start with ~all so a misconfiguration is tagged rather than rejected, then tighten"
echo "   to -all once delivery to a public mailbox is confirmed."
echo
echo "3. DMARC — TXT on _dmarc.${mail_domain}"
echo "     v=DMARC1; p=none; rua=mailto:dmarc@${mail_domain}"
echo "   p=none reports without acting; move to quarantine once the reports are clean."
echo
echo "4. MX — A/AAAA on ${api_domain} must already resolve to the VPS."
echo "   MX record for ${mail_domain}:"
echo "     ${mail_domain}. IN MX 10 ${api_domain}."
echo
echo "5. PTR — set in the provider's network console, not in the DNS zone:"
echo "     the VPS IP must resolve back to ${api_domain}"
echo "   Without it, large providers reject or heavily penalise this relay."
echo
echo "== Ports 25, 465, and 587 are published on the host. Verify reachability from outside"
echo "   the VPS, and check the provider is not blocking outbound 25 (delivery needs it)."
