use error_stack::Report;
use logandaim_core::{
    ResultReport,
    application::{AccessError, AuthContext},
    service::SessionPrüfen,
};

use crate::{LogandaimApiSpec, http::jwt};

/// Cookie name. Browsers require a leading `__Host-` name to be set with `Secure`,
/// `Path=/`, and without `Domain`.
pub const REFRESH_COOKIE: &str = "__Host-logandaim_refresh";

/// Split one `Authorization` header value into the bearer token.
///
/// `None` when the scheme is not `Bearer` or the value is empty. The header is split once,
/// so a token containing a space cannot smuggle a second scheme.
pub fn bearer_token(header: &str) -> Option<&str> {
    let (schema, wert) = header.split_once(' ')?;
    if !schema.eq_ignore_ascii_case("Bearer") {
        return None;
    }
    let wert = wert.trim();
    (!wert.is_empty()).then_some(wert)
}

impl LogandaimApiSpec {
    /// Turn an optional `Authorization` header into the identity to execute as.
    ///
    /// Absent header → [`AuthContext::Anonymous`]. A present but unverifiable one is a
    /// refusal, not anonymity: silently downgrading a broken token to anonymous would make
    /// a protected route answer "not logged in" instead of "your token is bad".
    pub async fn auth_context(
        &self,
        authorization_header: Option<&str>,
    ) -> ResultReport<AuthContext, AccessError> {
        let Some(header) = authorization_header else {
            return Ok(AuthContext::Anonymous);
        };
        let token =
            bearer_token(header).ok_or_else(|| Report::new(AccessError::NotAuthenticated))?;

        let gelesen = jwt::decode(token)?;
        self.app_api
            .inner_app()
            .execute(
                AuthContext::Anonymous,
                SessionPrüfen {
                    claims: gelesen.claims,
                    signed_message: gelesen.signed_message,
                    signature: gelesen.signature,
                },
            )
            .await
            .map_err(|_| Report::new(AccessError::NotAuthenticated))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn bearer_parsing() {
        assert_eq!(bearer_token("Bearer abc"), Some("abc"));
        assert_eq!(bearer_token("bearer abc"), Some("abc"));
        assert_eq!(bearer_token("Basic abc"), None);
        assert_eq!(bearer_token("Bearer "), None);
        assert_eq!(bearer_token("abc"), None);
        // Only one split: a second scheme cannot be smuggled in.
        assert_eq!(bearer_token("Bearer a b"), Some("a b"));
    }
}
