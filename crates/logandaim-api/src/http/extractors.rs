use std::net::{IpAddr, Ipv4Addr};

use logandaim_core::ports::ClientKey;
use poem::{Request, RequestBody, web::RealIp};
use poem_openapi::{
    ApiExtractor, ApiExtractorType, ExtractParamOptions,
    registry::{MetaParamIn, MetaSchemaRef, MetaSecurityScheme, Registry},
    types::Type,
};

use sha2::{Digest, Sha256};

/// The name the `Authorization` scheme is published under.
pub const BEARER_SCHEME: &str = "bearerAuth";

fn register_bearer(registry: &mut Registry) {
    registry.create_security_scheme(
        BEARER_SCHEME,
        MetaSecurityScheme {
            ty: "http",
            description: Some(
                "Access token from registration, sign-in, or session renewal, sent as \
                 `Authorization: Bearer <token>`.",
            ),
            name: None,
            key_in: None,
            scheme: Some("bearer"),
            bearer_format: Some("JWT"),
            flows: None,
            openid_connect_url: None,
        },
    );
}

async fn bearer_from_request(request: &Request, name: &str) -> Option<String> {
    request
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

macro_rules! bearer_extractor {
    ($(#[$meta:meta])* $name:ident, fallback = $fallback:expr) => {
        $(#[$meta])*
        pub struct $name(pub Option<String>);

        impl<'a> ApiExtractor<'a> for $name {
            const TYPES: &'static [ApiExtractorType] = &[
                ApiExtractorType::Parameter,
                ApiExtractorType::SecurityScheme,
            ];

            type ParamType = Option<String>;
            type ParamRawType = String;

            fn register(registry: &mut Registry) {
                register_bearer(registry);
            }

            fn security_schemes() -> Vec<&'static str> {
                vec![BEARER_SCHEME]
            }

            fn has_security_fallback() -> bool {
                $fallback
            }

            fn param_in() -> Option<MetaParamIn> {
                Some(MetaParamIn::Header)
            }

            fn param_schema_ref() -> Option<MetaSchemaRef> {
                Some(<Option<String> as Type>::schema_ref())
            }

            fn param_raw_type(&self) -> Option<&Self::ParamRawType> {
                self.0.as_ref()
            }

            async fn from_request(
                request: &'a Request,
                _body: &mut RequestBody,
                param_opts: ExtractParamOptions<Self::ParamType>,
            ) -> poem::Result<Self> {
                Ok(Self(bearer_from_request(request, &param_opts.name).await))
            }
        }
    };
}

bearer_extractor!(
    /// The optional `Authorization` header of a route that also serves anonymous callers.
    ///
    /// It is deliberately both a parameter and a security scheme: the scheme says what the
    /// header carries, while the parameter stays optional so "no header means anonymous" is
    /// expressible. Extraction never fails — an unusable token is the handler's refusal, not
    /// a parse error, so the `401` keeps its own error envelope.
    BearerAuthorization,
    fallback = true
);

bearer_extractor!(
    /// The required `Authorization` header of a route that has no anonymous answer.
    ///
    /// It publishes the same scheme as [`BearerAuthorization`] but without the anonymous
    /// alternative, so a reader sees which operations are callable without a token. It still
    /// extracts a missing header as `None`: the handler answers `401` in its own error
    /// envelope rather than poem's parameter error.
    RequiredBearerAuthorization,
    fallback = false
);

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
