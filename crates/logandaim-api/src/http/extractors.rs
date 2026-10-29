use std::net::{IpAddr, Ipv4Addr};

use logandaim_core::ports::ClientKey;
use poem::web::RealIp;

use sha2::{Digest, Sha256};

/// The throttle key of the client behind one request: real IP plus a header fingerprint,
/// never anything from the body, so a caller cannot choose its own budget.
pub fn client_key(
    real_ip: RealIp,
    user_agent: Option<&str>,
    accept_language: Option<&str>,
) -> ClientKey {
    // No socket address at all (a test client, a unix socket): loopback is the honest
    // placeholder, and it keeps the budget shared rather than unlimited.
    let ip = real_ip.0.unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));

    let mut hasher = Sha256::new();
    hasher.update(user_agent.unwrap_or("").as_bytes());
    hasher.update(b"\x1f");
    hasher.update(accept_language.unwrap_or("").as_bytes());
    // First 16 hex chars: enough to tell clients apart, short enough to read in a log.
    let fingerprint = hex::encode(hasher.finalize())[..16].to_owned();

    ClientKey::new(ip, fingerprint)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip() -> RealIp {
        RealIp(Some("203.0.113.7".parse().expect("static address")))
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

        let andere_ip = client_key(
            RealIp(Some("198.51.100.1".parse().unwrap())),
            Some("Mozilla/5.0"),
            None,
        );
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
