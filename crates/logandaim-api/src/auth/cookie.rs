//! The refresh cookie.
//!
//! One builder, so every response that touches the cookie emits the *same* attribute set:
//! a `Set-Cookie` that changes one attribute between setting and clearing is a cookie the
//! browser will not clear, and the refresh token outlives the logout.
//!
//! `__Host-` prefix, `HttpOnly`, `Secure`, `SameSite=Lax`, `Path=/`, and **no** `Domain`:
//! the prefix is only honoured by browsers when those hold, and it is what stops a
//! subdomain from overwriting the cookie. `SameSite=Lax` (not `Strict`) is deliberate: it
//! still blocks cross-site POSTs, which is the CSRF surface, while keeping the cookie on a
//! top-level navigation back to the app.

use logandaim_core::domain::konto::{REFRESH_TOKEN_LIFETIME, RefreshToken};

/// Cookie name. Browsers require a leading `__Host-` name to be set with `Secure`,
/// `Path=/`, and without `Domain`.
pub const REFRESH_COOKIE: &str = "__Host-logandaim_refresh";

/// `Set-Cookie` value that stores a fresh refresh token.
pub fn setzen(token: &RefreshToken) -> String {
    setzen_raw(token.as_str())
}

/// The same builder for a token that arrived as a string.
///
/// The facade keeps the refresh token as a `String` between the use case and the response,
/// so that value has to go through the identical attribute set — a second format string
/// here is exactly how a cookie stops being clearable.
pub fn setzen_raw(token: &str) -> String {
    format!(
        "{REFRESH_COOKIE}={token}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
        REFRESH_TOKEN_LIFETIME.num_seconds()
    )
}

/// `Set-Cookie` value that deletes the cookie.
///
/// Identical attributes with `Max-Age=0`: a clearing header that omits `Path` or `Secure`
/// targets a different cookie, and the token stays in the jar.
pub fn loeschen() -> String {
    format!("{REFRESH_COOKIE}=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token() -> RefreshToken {
        RefreshToken::neu().expect("the CSPRNG works")
    }

    #[test_log::test]
    fn die_attribute_sind_vollstaendig() {
        let kopf = setzen(&token());
        for attribut in [
            "Path=/",
            "HttpOnly",
            "Secure",
            "SameSite=Lax",
            "Max-Age=2592000",
        ] {
            assert!(kopf.contains(attribut), "{attribut} fehlt in {kopf}");
        }
        assert!(
            !kopf.contains("Domain"),
            "der __Host- Präfix verbietet Domain"
        );
    }

    #[test_log::test]
    fn das_loeschen_trifft_dasselbe_cookie() {
        let gesetzt = setzen(&token());
        let geloescht = loeschen();

        let attribute = |kopf: &str| -> Vec<String> {
            kopf.split(';')
                .skip(1)
                .map(|teil| teil.trim().to_owned())
                .filter(|teil| !teil.starts_with("Max-Age="))
                .collect()
        };
        assert_eq!(
            attribute(&gesetzt),
            attribute(&geloescht),
            "sonst löscht der Browser ein anderes Cookie"
        );
        assert!(geloescht.contains("Max-Age=0"));
    }

    #[test_log::test]
    fn das_token_steht_nur_im_cookie_und_hat_256_bit() {
        let token = token();
        assert_eq!(token.as_str().len(), 43);
        assert!(setzen(&token).starts_with(&format!("{REFRESH_COOKIE}={}", token.as_str())));
    }
}
