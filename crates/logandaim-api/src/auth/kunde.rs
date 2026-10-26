//! Client identity for the throttles.
//!
//! A throttle key is an IP plus a fingerprint, never an E-Mail-Adresse: a per-address
//! limit would let anybody lock an owner out by guessing their address (SPEC-konto rule 5).

use std::net::IpAddr;

use logandaim_core::ports::ClientKey;

/// Build the key of the client behind one request.
///
/// The fingerprint is a hash of the `User-Agent` (and the `Accept-Language` header), so a
/// NAT'd IP shared by a household still gets separate budgets, while a script that varies
/// only its UA does not get a fresh one per request — the IP is in the key either way.
///
/// `real_ip` is what a reverse proxy forwards; the caller supplies it, so this module
/// never guesses at `X-Forwarded-For` on its own.
pub fn client_key(
    real_ip: IpAddr,
    user_agent: Option<&str>,
    accept_language: Option<&str>,
) -> ClientKey {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(user_agent.unwrap_or("").as_bytes());
    hasher.update(b"\x1f");
    hasher.update(accept_language.unwrap_or("").as_bytes());
    // First 16 hex chars: enough to tell clients apart, short enough to read in a log.
    let fingerprint = hex::encode(hasher.finalize())[..16].to_owned();

    ClientKey::new(real_ip, fingerprint)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip() -> IpAddr {
        "203.0.113.7".parse().expect("static address")
    }

    #[test_log::test]
    fn gleiche_kennung_ergibt_gleichen_schlüssel() {
        let a = client_key(ip(), Some("Mozilla/5.0"), Some("de-AT"));
        let b = client_key(ip(), Some("Mozilla/5.0"), Some("de-AT"));
        assert_eq!(a, b);
    }

    #[test_log::test]
    fn andere_ip_oder_anderer_client_ergeben_andere_schlüssel() {
        let basis = client_key(ip(), Some("Mozilla/5.0"), None);

        let andere_ip = client_key("198.51.100.1".parse().unwrap(), Some("Mozilla/5.0"), None);
        assert_ne!(basis, andere_ip);

        let anderer_client = client_key(ip(), Some("curl/8"), None);
        assert_ne!(basis, anderer_client);
    }

    #[test_log::test]
    fn fehlende_header_sind_kein_fehler() {
        let a = client_key(ip(), None, None);
        assert_eq!(a.fingerprint.len(), 16);
    }
}
