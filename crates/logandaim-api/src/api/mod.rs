//! Typed facade between HTTP and use cases.
//!
//! Everything here is a thin translation: request → use case → schema. No business rules.
//! Use-case errors pass through unchanged; `HttpStatusMapping` in `errors/http_status.rs`
//! turns them into status codes.
//!
//! Two responsibilities are genuinely this layer's and must stay here:
//!
//! - **The access token's envelope.** Core signs bytes; this facade serialises the claims,
//!   base64url-encodes them, and asks core for the signature (`auth::jwt`). The reverse on
//!   the way in.
//! - **Turning a bearer header into an [`AuthContext`].** A token that verifies is then
//!   handed to `SitzungPrüfen`, which is what makes a *revoked* session fail even though
//!   its signature is still valid (AC-17). Only that use case's answer is injected into the
//!   use case that follows.

use std::sync::Arc;

use error_stack::Report;
use logandaim_core::{
    App, ResultReport,
    application::{AccessError, AuthContext},
    ports::ClientKey,
    service::{
        Abmeldung, AbmeldungFehler, Anmeldung, AnmeldungFehler, BenutzerAnzeigen,
        BenutzerAnzeigenFehler, BenutzerLoeschenFehler, BenutzerLöschen, Erneuerung,
        ErneuerungFehler, PasswortVergessen, PasswortVergessenFehler, PasswortWiederherstellen,
        PasswortWiederherstellenFehler, RegistrierungAbschliessenFehler, RegistrierungAbschließen,
        RegistrierungStarten, RegistrierungStartenFehler, SchussRegistrieren,
        SchussRegistrierenFehler, SitzungAusgestellt, SitzungPrüfen, WertungBerechnen,
        WertungFehler,
    },
};

pub mod extractors;
pub mod requests;
pub mod routes;
pub mod session;

use crate::{
    auth::jwt,
    schema::{
        AnmeldeAntwort, BenutzerAntwort, SchemaBenutzer, Schuss, Wertung,
        schema_schuss_from_domain, schema_wertung_from_domain,
    },
};

/// One issued session: the body the client receives, and the refresh token that must
/// leave as a cookie instead.
///
/// The two are deliberately separate values. `antwort` holds the access token only, so no
/// code path can accidentally serialise a refresh token into a JSON body or a log line.
pub struct AusgestellteSitzung {
    pub antwort: AnmeldeAntwort,
    pub refresh_token: String,
}

impl AusgestellteSitzung {
    fn aus(
        ausgestellt: SitzungAusgestellt,
        signing_key: &dyn logandaim_core::ports::Signing,
    ) -> Self {
        let access_token =
            jwt::encode(&ausgestellt.claims, signing_key).expect("the JOSE header serialises");
        Self {
            antwort: AnmeldeAntwort {
                access_token: access_token.as_str().to_owned(),
                benutzer: SchemaBenutzer::from(ausgestellt.benutzer),
            },
            refresh_token: ausgestellt.refresh_token.base64url().as_str().to_owned(),
        }
    }
}

#[derive(Clone)]
pub struct LogandaimAppApi {
    app: Arc<App>,
}

impl LogandaimAppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }

    /// The configured signing key, for the edge that assembles a JOSE envelope.
    ///
    /// Verification does **not** come from here: a token is checked against the public key
    /// stored on its session row, inside `SitzungPrüfen`.
    pub fn signing_key(&self) -> &dyn logandaim_core::ports::Signing {
        self.app.signing()
    }

    // ---------------------------------------------------------------- Schuss slice

    pub async fn schuss_registrieren(
        &self,
        erstellung: requests::SchussErstellung,
    ) -> ResultReport<Schuss, SchussRegistrierenFehler> {
        let schuss = self
            .app
            .execute(AuthContext::Anonymous, SchussRegistrieren::from(erstellung))
            .await?;
        Ok(schema_schuss_from_domain(schuss))
    }

    pub async fn wertung_berechnen(
        &self,
        anfrage: requests::WertungsAnfrage,
    ) -> ResultReport<Wertung, WertungFehler> {
        let ergebnis = self
            .app
            .execute(AuthContext::Anonymous, WertungBerechnen::from(anfrage))
            .await?;
        Ok(schema_wertung_from_domain(ergebnis))
    }

    // ---------------------------------------------------------------- Auth context

    /// Turn an optional `Authorization` header into the identity to execute as.
    ///
    /// Absent header → [`AuthContext::Anonymous`]. A present but unverifiable one is a
    /// refusal, not anonymity: silently downgrading a broken token to anonymous would make
    /// a protected route answer "not logged in" instead of "your token is bad".
    pub async fn auth_context(
        &self,
        authorization: Option<&str>,
    ) -> ResultReport<AuthContext, AccessError> {
        let Some(wert) = authorization else {
            return Ok(AuthContext::Anonymous);
        };
        let token =
            jwt::bearer_token(wert).ok_or_else(|| Report::new(AccessError::NotAuthenticated))?;

        let gelesen = jwt::decode(token)?;
        self.app
            .execute(
                AuthContext::Anonymous,
                SitzungPrüfen {
                    claims: gelesen.claims,
                    signed_message: gelesen.signed_message,
                    signature: gelesen.signature,
                },
            )
            .await
            .map_err(|_| Report::new(AccessError::NotAuthenticated))
    }

    // ---------------------------------------------------------------- Registration

    /// Step 1. Always answers `Ok`: a taken address is not disclosed (AC-2).
    pub async fn registrierung_starten(
        &self,
        anfrage: requests::RegistrierungStartAnfrage,
        client_key: ClientKey,
    ) -> ResultReport<(), RegistrierungStartenFehler> {
        let use_case: RegistrierungStarten = anfrage.in_use_case(client_key)?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    /// Step 2. Creates the Benutzer, its Konto, and the first session.
    pub async fn registrierung_abschließen(
        &self,
        anfrage: requests::RegistrierungAbschlussAnfrage,
        client_key: ClientKey,
    ) -> ResultReport<AusgestellteSitzung, RegistrierungAbschliessenFehler> {
        let use_case: RegistrierungAbschließen = anfrage.in_use_case(client_key)?;
        let ausgestellt = self.app.execute(AuthContext::Anonymous, use_case).await?;
        Ok(AusgestellteSitzung::aus(ausgestellt, self.signing_key()))
    }

    // ---------------------------------------------------------------- Login

    pub async fn anmelden(
        &self,
        anfrage: requests::AnmeldeAnfrage,
        client_key: ClientKey,
    ) -> ResultReport<AusgestellteSitzung, AnmeldungFehler> {
        let use_case: Anmeldung = anfrage.in_use_case(client_key)?;
        let ausgestellt = self.app.execute(AuthContext::Anonymous, use_case).await?;
        Ok(AusgestellteSitzung::aus(ausgestellt, self.signing_key()))
    }

    /// Rotate the refresh token. The presented one is spent; a replay ends the family.
    pub async fn erneuerung(
        &self,
        refresh_token: String,
        client_key: ClientKey,
    ) -> ResultReport<AusgestellteSitzung, ErneuerungFehler> {
        let refresh_token = logandaim_core::domain::konto::RefreshToken::try_from(refresh_token)
            .map_err(|_| Report::new(ErneuerungFehler::SitzungUngültig))?;

        let ausgestellt = self
            .app
            .execute(
                AuthContext::Anonymous,
                Erneuerung {
                    refresh_token,
                    client_key,
                },
            )
            .await?;
        let access_token = jwt::encode(&ausgestellt.claims, self.signing_key())
            .expect("the JOSE header serialises");
        Ok(AusgestellteSitzung {
            antwort: AnmeldeAntwort {
                access_token: access_token.as_str().to_owned(),
                // A renewal answers no Benutzer: the client already has it, and asking
                // for it would make renewal depend on a read it does not need.
                benutzer: SchemaBenutzer {
                    id: ausgestellt.claims.sub.0,
                    rolle: crate::schema::SchemaRolle::Benutzer,
                },
            },
            refresh_token: ausgestellt.refresh_token.base64url().as_str().to_owned(),
        })
    }

    pub async fn abmelden(&self, auth: AuthContext) -> ResultReport<(), AbmeldungFehler> {
        self.app.execute(auth, Abmeldung).await
    }

    // ---------------------------------------------------------------- Passwort

    pub async fn passwort_vergessen(
        &self,
        anfrage: requests::PasswortVergessenAnfrage,
        client_key: ClientKey,
    ) -> ResultReport<(), PasswortVergessenFehler> {
        let use_case: PasswortVergessen = anfrage.in_use_case(client_key)?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    /// Single use, and ends every session of the Benutzer.
    pub async fn passwort_wiederherstellen(
        &self,
        anfrage: requests::PasswortWiederherstellenAnfrage,
    ) -> ResultReport<(), PasswortWiederherstellenFehler> {
        let use_case: PasswortWiederherstellen = anfrage.in_use_case()?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    // ---------------------------------------------------------------- Benutzer

    /// The caller's own Benutzer, or another one when a Systemadministrator asks.
    pub async fn benutzer_anzeigen(
        &self,
        auth: AuthContext,
        benutzer_id: Option<uuid::Uuid>,
    ) -> ResultReport<BenutzerAntwort, BenutzerAnzeigenFehler> {
        let ansicht = self
            .app
            .execute(
                auth,
                BenutzerAnzeigen {
                    benutzer_id: benutzer_id.map(logandaim_core::domain::konto::BenutzerId),
                },
            )
            .await?;

        Ok(BenutzerAntwort {
            id: ansicht.id().0,
            e_mail_adresse: ansicht.email.as_ref().to_owned(),
            rolle: ansicht.benutzer.rolle().into(),
        })
    }

    pub async fn benutzer_löschen(
        &self,
        auth: AuthContext,
        anfrage: requests::BenutzerLöschungAnfrage,
    ) -> ResultReport<(), BenutzerLoeschenFehler> {
        let use_case: BenutzerLöschen = anfrage.in_use_case()?;
        self.app.execute(auth, use_case).await
    }
}
