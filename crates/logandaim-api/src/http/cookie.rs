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

use std::time::Duration;

/// `SameSite` attribute value.
///
/// `None` is only honoured by browsers together with `Secure`, so a config that combines the
/// two is a cookie the browser drops; the builder does not silently fix that.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SameSite {
    Strict,
    #[default]
    Lax,
    None,
}

impl SameSite {
    /// The attribute value as it appears in the header.
    fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "Strict",
            Self::Lax => "Lax",
            Self::None => "None",
        }
    }
}

/// The attributes of one cookie, so setting and clearing cannot drift apart.
///
/// Every field is optional except the name; the defaults are the ones this API needs for the
/// refresh cookie (see the module docs). A corner that needs a different cookie overrides only
/// the field it disagrees with, and derives its clearing header from the same value — which is
/// the whole point: a `Set-Cookie` that omits `Path` or `Secure` targets a different cookie,
/// and the token stays in the jar.
///
/// `Domain` has no default on purpose: a `__Host-` name must not carry one, and any caller
/// that sets it opts out of that protection knowingly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieConfig {
    /// `Path` attribute. `None` omits it and scopes the cookie to the request path.
    pub path: Option<String>,
    /// `Domain` attribute. `None` is the default and is what `__Host-` requires.
    pub domain: Option<String>,
    /// `HttpOnly`: hides the cookie from page scripts.
    pub http_only: bool,
    /// `Secure`: sends the cookie on HTTPS only.
    pub secure: bool,
    /// `SameSite` attribute.
    pub same_site: SameSite,
    /// `Max-Age` attribute. `None` makes it a session cookie.
    pub max_age: Option<Duration>,
}

impl Default for CookieConfig {
    fn default() -> Self {
        Self {
            path: None,
            domain: None,
            http_only: false,
            secure: false,
            same_site: SameSite::Lax,
            max_age: None,
        }
    }
}

impl CookieConfig {
    /// The same attributes with `Max-Age` set to `lifetime`.
    #[must_use]
    pub fn with_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_age = Some(lifetime);
        self
    }

    /// The same attributes as a session cookie (no `Max-Age`).
    #[must_use]
    pub fn session(mut self) -> Self {
        self.max_age = None;
        self
    }

    /// The `Set-Cookie` header value that stores `value`.
    ///
    /// `value` and `name` are written verbatim: they must already be free of `;`, `,`, and
    /// control characters, which holds for a base64url token and a literal name.
    fn header(&self, key: &str, value: &str) -> String {
        let mut kopf = format!("{key}={value}");

        let attribute = |kopf: &mut String, attribut: &str| {
            kopf.push_str("; ");
            kopf.push_str(attribut);
        };

        if let Some(path) = &self.path {
            attribute(&mut kopf, &format!("Path={path}"));
        }
        if let Some(domain) = &self.domain {
            attribute(&mut kopf, &format!("Domain={domain}"));
        }
        if self.http_only {
            attribute(&mut kopf, "HttpOnly");
        }
        if self.secure {
            attribute(&mut kopf, "Secure");
        }
        attribute(&mut kopf, &format!("SameSite={}", self.same_site.as_str()));
        if let Some(max_age) = self.max_age {
            attribute(&mut kopf, &format!("Max-Age={}", max_age.as_secs()));
        }

        kopf
    }
}

pub fn set_cookie_header(key: &str, value: &str, config: CookieConfig) -> String {
    config.header(key, value)
}

pub fn clear_cookie_header(key: &str, mut config: CookieConfig) -> String {
    config.max_age = Some(Duration::ZERO);
    config.header(key, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The attribute set the refresh cookie is set and cleared with.
    const NAME: &str = "__Host-logandaim_refresh";
    const LEBENSDAUER: Duration = Duration::from_secs(2_592_000);

    fn refresh_cookie() -> CookieConfig {
        CookieConfig {
            path: Some("/".to_owned()),
            max_age: Some(LEBENSDAUER),
            http_only: true,
            secure: true,
            ..CookieConfig::default()
        }
    }

    /// The attributes of a header, without the name/value pair and without `Max-Age`.
    fn attribute(kopf: &str) -> Vec<String> {
        kopf.split(';')
            .skip(1)
            .map(|teil| teil.trim().to_owned())
            .filter(|teil| !teil.starts_with("Max-Age="))
            .collect()
    }

    #[test_log::test]
    fn die_attribute_sind_vollständig() {
        let kopf = set_cookie_header(NAME, "token", refresh_cookie());
        for erwartet in [
            "Path=/",
            "HttpOnly",
            "Secure",
            "SameSite=Lax",
            "Max-Age=2592000",
        ] {
            assert!(kopf.contains(erwartet), "{erwartet} fehlt in {kopf}");
        }
        assert!(kopf.starts_with(&format!("{NAME}=token")), "{kopf}");
        assert!(
            !kopf.contains("Domain"),
            "der __Host- Präfix verbietet Domain"
        );
    }

    #[test_log::test]
    fn das_löschen_trifft_dasselbe_cookie() {
        let gesetzt = set_cookie_header(NAME, "token", refresh_cookie());
        let gelöscht = clear_cookie_header(NAME, refresh_cookie());

        assert_eq!(
            attribute(&gesetzt),
            attribute(&gelöscht),
            "sonst löscht der Browser ein anderes Cookie"
        );
        assert!(gelöscht.starts_with(&format!("{NAME}=;")), "{gelöscht}");
        assert!(gelöscht.ends_with("Max-Age=0"), "{gelöscht}");
    }

    #[test_log::test]
    fn das_löschen_setzt_max_age_auch_ohne_lebensdauer() {
        let gelöscht = clear_cookie_header("sid", CookieConfig::default());
        assert!(gelöscht.ends_with("Max-Age=0"), "{gelöscht}");
        assert!(gelöscht.starts_with("sid=;"), "{gelöscht}");
    }

    #[test_log::test]
    fn die_vorgaben_schreiben_nur_das_nötigste() {
        let kopf = set_cookie_header("sid", "abc", CookieConfig::default());
        assert_eq!(kopf, "sid=abc; SameSite=Lax");
    }

    #[test_log::test]
    fn optionale_attribute_erscheinen_nur_wenn_gesetzt() {
        let config = CookieConfig {
            path: Some("/api".to_owned()),
            domain: Some("example.test".to_owned()),
            http_only: true,
            secure: true,
            same_site: SameSite::Strict,
            max_age: Some(Duration::from_secs(60)),
        };
        assert_eq!(
            set_cookie_header("sid", "abc", config),
            "sid=abc; Path=/api; Domain=example.test; HttpOnly; Secure; SameSite=Strict; Max-Age=60"
        );
    }

    #[test_log::test]
    fn die_lebensdauer_und_die_session_variante_setzen_max_age() {
        let sesshaft = CookieConfig::default().with_lifetime(Duration::from_secs(90));
        assert_eq!(
            set_cookie_header("sid", "abc", sesshaft.clone()),
            "sid=abc; SameSite=Lax; Max-Age=90"
        );

        let session = sesshaft.session();
        assert_eq!(
            set_cookie_header("sid", "abc", session),
            "sid=abc; SameSite=Lax"
        );
    }
}
