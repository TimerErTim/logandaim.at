//! Reading the caller out of one request.
//!
//! Two functions, both on [`LogandaimAppApi`], because the request is the only place these
//! facts live:
//!
//! - [`LogandaimAppApi::require_authenticated`] turns the `Authorization` header into an
//!   [`AuthContext`] and **refuses with 401** rather than handing `Anonymous` to a
//!   protected use case (SPEC-konto AC-13). A route that wants the anonymous case as well
//!   uses [`LogandaimAppApi::auth_context_from`] instead.
//! - [`LogandaimAppApi::client_key`] builds the throttle key from the real IP and a header
//!   fingerprint — never from the body, so a caller cannot choose its own budget.
//!
//! Core checks the identity *again* in each use case's body. That is deliberate: the edge
//! can be misconfigured by a future route, the use case cannot.

use std::net::{IpAddr, Ipv4Addr};

use error_stack::Report;
use logandaim_core::{
    ResultReport,
    application::{AccessError, AuthContext},
    ports::ClientKey,
};
use poem::{FromRequest, Request};

use crate::auth::kunde;

use super::LogandaimAppApi;

impl LogandaimAppApi {
    /// The identity of the caller, or [`AuthContext::Anonymous`] when no header was sent.
    pub async fn auth_context_from(
        &self,
        request: &Request,
    ) -> ResultReport<AuthContext, AccessError> {
        let header = request
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok());
        self.auth_context(header).await
    }

    /// The identity of the caller, or a refusal. This is what a protected route calls.
    pub async fn require_authenticated(
        &self,
        request: &Request,
    ) -> ResultReport<AuthContext, AccessError> {
        let auth = self.auth_context_from(request).await?;
        if auth.is_authenticated() {
            Ok(auth)
        } else {
            Err(Report::new(AccessError::NotAuthenticated))
        }
    }

    /// The throttle key of the client behind this request.
    pub async fn client_key(&self, request: &Request) -> ClientKey {
        let mut leib = poem::RequestBody::default();
        let real_ip = poem::web::RealIp::from_request(request, &mut leib)
            .await
            .ok()
            .and_then(|real_ip| real_ip.0)
            // No socket address at all (a test client, a unix socket): loopback is the
            // honest placeholder, and it keeps the budget shared rather than unlimited.
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));

        let header = |name: &str| {
            request
                .headers()
                .get(name)
                .and_then(|value| value.to_str().ok())
        };

        kunde::client_key(real_ip, header("user-agent"), header("accept-language"))
    }
}
