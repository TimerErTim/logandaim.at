use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::{AuthContext, ExecutionContext},
    domain::{
        EmailAdresse,
        konto::{
            AccessTokenClaims, KlartextPasswort, NeueSession, RefreshToken, SecretToken,
            dummy_verify, hash_token, verify_password,
        },
    },
    ports::{DrosselAktion, KundeSchlüssel},
    service::UseCase,
};

use super::{
    KontoDienstFehler, SitzungAusgestellt, claims_fuer, drossle, signing_key_id, speicherfehler,
};

/// Log in with address and Passwort.
///
/// Unknown address and wrong Passwort answer identically, and the unknown-address path
/// still pays for one argon2 verification (`dummy_verify`), so the response time does not
/// disclose existence (rule 4, AC-6).
#[derive(Debug, Clone)]
pub struct Anmeldung {
    pub email: String,
    pub passwort: String,
    /// Throttle key. Never the address: that would let anybody lock an owner out.
    pub kunde: KundeSchlüssel,
}

#[async_trait]
impl UseCase<SitzungAusgestellt> for Anmeldung {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SitzungAusgestellt, Self::Error> {
        let email = EmailAdresse::normalisiert(&self.email)
            .change_context(KontoDienstFehler::UngültigeAnfrage)?;
        let passwort = KlartextPasswort::neu(self.passwort)
            .change_context(KontoDienstFehler::UngültigeAnfrage)?;
        let jetzt = ctx.clock().now();

        drossle(&ctx, &self.kunde, DrosselAktion::Anmeldung, jetzt).await?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let konto = uow
                .konten()
                .find_by_email(&email)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let Some(konto) = konto else {
                // Same work as the wrong-Passwort path (rule 4).
                dummy_verify(&passwort).change_context(KontoDienstFehler::Speicher)?;
                return Err(Report::new(KontoDienstFehler::ZugangsdatenFalsch));
            };

            let stimmt = verify_password(&passwort, konto.password_hash())
                .change_context(KontoDienstFehler::Speicher)?;
            if !stimmt {
                return Err(Report::new(KontoDienstFehler::ZugangsdatenFalsch));
            }

            let benutzer = uow
                .benutzer()
                .find_by_id(konto.benutzer_id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::ZugangsdatenFalsch)?;

            let refresh_token = RefreshToken::neu().change_context(KontoDienstFehler::Speicher)?;
            let session = uow
                .sessions()
                .create(NeueSession::neu(
                    benutzer.id(),
                    refresh_token.hash(),
                    signing_key_id(&ctx),
                    jetzt,
                ))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let claims = claims_fuer(&ctx, benutzer.id(), session.id(), jetzt);
            Ok::<_, Report<KontoDienstFehler>>(SitzungAusgestellt {
                benutzer,
                session_id: session.id(),
                refresh_token,
                claims,
            })
        }
        .await;

        let ausgestellt = uow.finish_with(ergebnis, speicherfehler).await?;

        // A successful login clears the client's failure budget, so one mistyped
        // Passwort is not held against it for the rest of the window (rule 5).
        let _ = ctx
            .drossel()
            .zurücksetzen(&self.kunde, DrosselAktion::Anmeldung)
            .await;

        tracing::info!(benutzer_id = ?ausgestellt.benutzer.id(), "angemeldet");
        Ok(ausgestellt)
    }
}

/// A renewed token pair: the API encodes `claims`, the client gets `refresh_token`.
#[derive(Debug, Clone)]
pub struct ErneuerungAusgestellt {
    pub refresh_token: RefreshToken,
    pub claims: AccessTokenClaims,
}

/// Rotate the refresh token of one Anmelde-Sitzung.
///
/// Every use spends the presented token and mints a successor session. A second use of an
/// already spent token is treated as theft and ends the session family (rule 12).
#[derive(Debug, Clone)]
pub struct Erneuerung {
    pub refresh_token: String,
    pub kunde: KundeSchlüssel,
}

#[async_trait]
impl UseCase<ErneuerungAusgestellt> for Erneuerung {
    type Error = Report<KontoDienstFehler>;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> Result<ErneuerungAusgestellt, Self::Error> {
        let jetzt = ctx.clock().now();
        drossle(&ctx, &self.kunde, DrosselAktion::Erneuerung, jetzt).await?;

        let token = SecretToken::parse(self.refresh_token)
            .change_context(KontoDienstFehler::SitzungUngültig)?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let session = uow
                .sessions()
                .find_by_refresh_hash(&hash_token(&token))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let Some(session) = session else {
                return Err(Report::new(KontoDienstFehler::SitzungUngültig));
            };

            if session.ist_verbraucht() {
                // Replay of a spent token: end the family rather than warning (rule 12).
                // The revocation must *commit*, so it is done here and reported as an
                // outcome, not as an error that would roll the whole unit of work back.
                uow.sessions()
                    .widerrufe_alle_fuer_benutzer(session.benutzer_id(), jetzt)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(KontoDienstFehler::Speicher)?;
                return Ok::<_, Report<KontoDienstFehler>>(Erneuerungsausgang::Wiederholung);
            }

            if !session.ist_gültig(jetzt) {
                return Err(Report::new(KontoDienstFehler::SitzungUngültig));
            }

            let folge_refresh = RefreshToken::neu().change_context(KontoDienstFehler::Speicher)?;
            let folge = uow
                .sessions()
                .create(session.erneuern(folge_refresh.hash(), jetzt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            uow.sessions()
                .markiere_verbraucht(session.id(), jetzt)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let claims = claims_fuer(&ctx, session.benutzer_id(), folge.id(), jetzt);
            Ok(Erneuerungsausgang::Neu(ErneuerungAusgestellt {
                refresh_token: folge_refresh,
                claims,
            }))
        }
        .await;

        match uow.finish_with(ergebnis, speicherfehler).await? {
            Erneuerungsausgang::Neu(ausgestellt) => Ok(ausgestellt),
            Erneuerungsausgang::Wiederholung => {
                Err(Report::new(KontoDienstFehler::SitzungUngültig))
            }
        }
    }
}

/// Whether a renewal produced a successor session or detected a replay.
enum Erneuerungsausgang {
    Neu(ErneuerungAusgestellt),
    Wiederholung,
}

/// End the current Anmelde-Sitzung. Idempotent: a missing session is already logged out.
#[derive(Debug, Clone, Copy)]
pub struct Abmeldung;

#[async_trait]
impl UseCase<()> for Abmeldung {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let (_, session_id) = ctx
            .auth()
            .require_authenticated()
            .change_context(KontoDienstFehler::NichtAngemeldet)?;
        let jetzt = ctx.clock().now();

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;
        let ergebnis = uow
            .sessions()
            .widerrufe(session_id, jetzt)
            .await
            .map_err(IntoReport::into_report)
            .change_context(KontoDienstFehler::Speicher);

        match uow.finish(ergebnis, KontoDienstFehler::Speicher).await {
            Ok(()) => Ok(()),
            // Already gone: the caller's intent is satisfied.
            Err(fehler) if matches!(fehler.current_context(), KontoDienstFehler::Speicher) => {
                tracing::debug!("abmeldung: sitzung war bereits beendet");
                Ok(())
            }
            Err(fehler) => Err(fehler),
        }
    }
}

/// Verify an access token that the API already parsed and signature-checked.
///
/// This is where an unexpired token still gets rejected: the `kid` row must be active and
/// the `sid` row must be unrevoked, both of which are database rows (rule 15, AC-17).
#[derive(Debug, Clone)]
pub struct SitzungPrüfen {
    pub claims: AccessTokenClaims,
}

#[async_trait]
impl UseCase<AuthContext> for SitzungPrüfen {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<AuthContext, Self::Error> {
        let jetzt = ctx.clock().now();
        let konfiguration = ctx.konto_konfiguration();

        self.claims
            .prüfe_herkunft(&konfiguration.issuer, &konfiguration.audience)
            .change_context(KontoDienstFehler::NichtAngemeldet)?;
        self.claims
            .prüfe_zeitfenster(jetzt)
            .change_context(KontoDienstFehler::NichtAngemeldet)?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let schlüssel = uow
                .signing_keys()
                .find_by_kid(&self.claims.kid)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::NichtAngemeldet)?;

            let Some(schlüssel) = schlüssel else {
                return Err(Report::new(KontoDienstFehler::NichtAngemeldet));
            };
            if !schlüssel.ist_aktiv() {
                return Err(Report::new(KontoDienstFehler::NichtAngemeldet));
            }

            let session = uow
                .sessions()
                .find_by_id(self.claims.sid)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::NichtAngemeldet)?;

            if session.benutzer_id() != self.claims.sub || !session.ist_annehmbar(jetzt) {
                return Err(Report::new(KontoDienstFehler::NichtAngemeldet));
            }

            // The Benutzer must still exist: a deleted account leaves no live identity.
            uow.benutzer()
                .find_by_id(self.claims.sub)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::NichtAngemeldet)?;

            Ok::<_, Report<KontoDienstFehler>>(AuthContext::Authenticated {
                benutzer_id: self.claims.sub,
                session_id: self.claims.sid,
            })
        }
        .await;

        uow.finish_with(ergebnis, speicherfehler).await
    }
}
