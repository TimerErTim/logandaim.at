//! Typed facade between HTTP and use cases.
//!
//! Everything here is a thin translation: request → use case → schema. Reads are composed
//! inline from the repositories they need, writes run as use cases. No business rules.
//! Use-case errors pass through unchanged; `errors/http_status.rs` maps them to status codes.
//! Nothing here touches `poem`: the HTTP edge lives in `http/mod.rs` behind the `openapi`
//! feature, so the library compiles without it. Requests are grouped in `api/requests/`; the
//! HTTP edge owns JWT encoding and bearer verification (`http/jwt.rs`, `http/auth.rs`).
//!
//! Authorisation is never decided here either. A read that runs inline calls the same
//! helpers in `logandaim_core::service` that the use cases do; a write hands the caller's
//! [`AuthContext`] straight to the use case, which loads the aggregates and answers 401/403
//! itself (SPEC-konto rule 17).

use std::sync::Arc;

use error_stack::{IntoReport, Report, ResultExt};
use logandaim_core::{
    App, ResultReport,
    application::AuthContext,
    domain::{AnlageId, EmailAdresse, ParkourId, RundeId, ZielId, konto::BenutzerId},
    ports::{ClientKey, ThrottleAction},
    service::{
        Abmeldung, AbmeldungFehler, AnlageAnsicht, AnlageBeschreibendeDatenÄndern,
        AnlageDeaktivieren, AnlageVerwaltungEntziehen, AnlageVerwaltungGewähren,
        AnlageVerwaltungÄndern, AnlageZugriff as AnlageZugriffAnsicht,
        AnlageÖffentlich as AnlageÖffentlichAnsicht, Anmeldung, AnmeldungFehler, BenutzerLöschen,
        BenutzerLöschenFehler, Erneuerung, ErneuerungFehler, ParkourEntfernen, ParkourHinzufügen,
        ParkourKurz as ParkourKurzAnsicht, ParkourSetupÄndern, ParkourUmbenennen,
        ParkourÖffentlich as ParkourÖffentlichAnsicht, PasswortVergessen, PasswortVergessenFehler,
        PasswortWiederherstellen, PasswortWiederherstellenFehler, RegistrierungAbschliessenFehler,
        RegistrierungAbschließen, RegistrierungStarten, RegistrierungStartenFehler,
        RundeAbschliessen, RundeAbschliessenFehler, RundeAnsicht, RundeAnzeigenFehler,
        RundeStarten, RundeStartenFehler, RundenAnzeigenFehler, SchussRegistrieren,
        SchussRegistrierenFehler, SessionAusgestellt, StammdatenFehler, ZielEntfernen,
        ZielErsetzen, ZielHinzufügen, ZielÄndern, anlage_laden, anmeldung, benutzer_laden,
        darf_anlage_lesen, darf_anlage_verwalten, darf_masterdaten_pflegen, ist_nicht_gefunden,
        runden_neueste_zuerst,
    },
};
use tracing::warn;

pub mod requests;

use crate::schema::{
    Anlage, AnlageZugriff, AnlageÖffentlich, Benutzer, ParkourÖffentlich, Runde, RundeBasic, Schuss,
};

#[derive(Clone)]
pub struct LogandaimAppApi {
    app: Arc<App>,
}

impl LogandaimAppApi {
    pub fn new(app: App) -> Self {
        Self::from_arc(Arc::new(app))
    }

    /// An app the caller already holds behind an `Arc`, for a harness that must also reach
    /// the use cases directly (the e2e suite's superuser bootstrap).
    pub fn from_arc(app: Arc<App>) -> Self {
        Self { app }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }

    // ---------------------------------------------------------------- Anlage

    /// Create an Anlage. Only a Systemadministrator may (AC-1).
    pub async fn anlage_erstellen(
        &self,
        auth: AuthContext,
        anfrage: requests::AnlageErstellungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = anfrage.in_use_case()?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// The Anlagen the caller may administer.
    ///
    /// A `Systemadministrator` sees every active Anlage; everybody else sees the Anlagen their
    /// `AnlageVerwaltung` names. Archived Anlagen are not listed: only a
    /// `Systemadministrator` may still read one, through its detail route.
    pub async fn anlagen_anzeigen(
        &self,
        auth: AuthContext,
    ) -> ResultReport<Vec<AnlageZugriff>, StammdatenFehler> {
        self.app
            .execute_fn(auth, async |ctx| {
                let uow = ctx
                    .enter()
                    .await
                    .change_context(StammdatenFehler::AnlageLaden)?;

                let ergebnis = async {
                    let benutzer_id = anmeldung(&ctx)?;
                    let benutzer = benutzer_laden(&uow, benutzer_id).await?;
                    let ist_systemadministrator = benutzer.ist_systemadministrator();

                    let anlagen = if ist_systemadministrator {
                        uow.anlagen()
                            .find_aktiv()
                            .await
                            .map_err(IntoReport::into_report)
                            .change_context(StammdatenFehler::AnlageLaden)?
                    } else {
                        uow.anlagen()
                            .find_by_verwaltung(benutzer_id)
                            .await
                            .map_err(IntoReport::into_report)
                            .change_context(StammdatenFehler::AnlageLaden)?
                            .into_iter()
                            .filter(|anlage| darf_anlage_lesen(&benutzer, anlage))
                            .collect()
                    };

                    Ok(anlagen
                        .into_iter()
                        .map(|anlage| AnlageZugriffAnsicht {
                            rolle: anlage.rolle_von(benutzer_id),
                            ist_systemadministrator,
                            anlage: anlage.into_data(),
                        })
                        .map(Into::into)
                        .collect())
                }
                .await;

                uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
            })
            .await
    }

    /// One Anlage with its master data. An archived Anlage answers only to a
    /// `Systemadministrator`; everyone else gets 403 (AC-11).
    pub async fn anlage_anzeigen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        self.app
            .execute_fn(auth, async |ctx| {
                let uow = ctx
                    .enter()
                    .await
                    .change_context(StammdatenFehler::AnlageLaden)?;

                let ergebnis = async {
                    let benutzer_id = anmeldung(&ctx)?;
                    let benutzer = benutzer_laden(&uow, benutzer_id).await?;
                    let anlage = anlage_laden(&uow, anlage_id).await?;
                    if !darf_masterdaten_pflegen(&benutzer, &anlage)
                        || !darf_anlage_lesen(&benutzer, &anlage)
                    {
                        return Err(Report::new(StammdatenFehler::NichtErlaubt));
                    }
                    Ok(AnlageAnsicht::neu(anlage.into_data()))
                }
                .await;

                let ansicht = uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await?;
                Ok(ansicht.into())
            })
            .await
    }

    pub async fn anlage_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::AnlageÄnderungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: AnlageBeschreibendeDatenÄndern = anfrage.in_use_case(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Irreversible: no reactivation exists (AC-10).
    pub async fn anlage_deaktivieren(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let anlage = self
            .app
            .execute(auth, AnlageDeaktivieren { anlage_id })
            .await?;
        Ok(anlage.into())
    }

    // ---------------------------------------------------------------- AnlageVerwaltung

    pub async fn verwaltung_gewähren(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::AnlageVerwaltungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: AnlageVerwaltungGewähren = anfrage.als_gewähren(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn verwaltung_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        benutzer_id: BenutzerId,
        anfrage: requests::AnlageVerwaltungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: AnlageVerwaltungÄndern = anfrage.als_ändern(anlage_id, benutzer_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn verwaltung_entziehen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        benutzer_id: BenutzerId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = AnlageVerwaltungEntziehen {
            anlage_id,
            benutzer_id,
        };
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    // ---------------------------------------------------------------- Ziel

    pub async fn ziel_hinzufügen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::ZielRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ZielHinzufügen = anfrage.als_hinzufügen(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Edit details; the identity and its history stay (AC-5).
    pub async fn ziel_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        ziel_id: ZielId,
        anfrage: requests::ZielRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ZielÄndern = anfrage.als_ändern(anlage_id, ziel_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Replace the physical target: a new identity, never the old history (AC-5).
    pub async fn ziel_ersetzen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        ziel_id: ZielId,
        anfrage: requests::ZielRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ZielErsetzen = anfrage.als_ersetzen(anlage_id, ziel_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Archive when history references it, delete when nothing does (AC-11, AC-12).
    pub async fn ziel_entfernen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        ziel_id: ZielId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = ZielEntfernen { anlage_id, ziel_id };
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    // ---------------------------------------------------------------- Parkour

    pub async fn parkour_hinzufügen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        anfrage: requests::ParkourErstellungRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ParkourHinzufügen = anfrage.in_use_case(anlage_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn parkour_setup_ändern(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        parkour_id: ParkourId,
        anfrage: requests::ParkourSetupRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ParkourSetupÄndern = anfrage.in_use_case(anlage_id, parkour_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    pub async fn parkour_umbenennen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        parkour_id: ParkourId,
        anfrage: requests::ParkourUmbenennenRequest,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case: ParkourUmbenennen = anfrage.in_use_case(anlage_id, parkour_id)?;
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Archive when a Runde captured it, delete when none did (AC-11, AC-12).
    pub async fn parkour_entfernen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        parkour_id: ParkourId,
    ) -> ResultReport<Anlage, StammdatenFehler> {
        let use_case = ParkourEntfernen {
            anlage_id,
            parkour_id,
        };
        let anlage = self.app.execute(auth, use_case).await?;
        Ok(anlage.into())
    }

    /// Every active Parkour of every active Anlage. Needs no identity (AC-13).
    pub async fn parkour_anzeigen(
        &self,
        auth: AuthContext,
    ) -> ResultReport<Vec<ParkourÖffentlich>, StammdatenFehler> {
        self.app
            .execute_fn(auth, async |ctx| {
                let uow = ctx
                    .enter()
                    .await
                    .change_context(StammdatenFehler::AnlageLaden)?;

                let ergebnis = async {
                    let anlagen = uow
                        .anlagen()
                        .find_aktiv()
                        .await
                        .map_err(IntoReport::into_report)
                        .change_context(StammdatenFehler::AnlageLaden)?;

                    let mut offen = Vec::new();
                    for anlage in anlagen {
                        for parkour in anlage.parkour() {
                            if parkour.ist_archiviert() {
                                continue;
                            }
                            offen.push(ParkourÖffentlichAnsicht {
                                anlage_id: anlage.id(),
                                anlage_name: anlage.name().clone(),
                                parkour: parkour.clone(),
                            });
                        }
                    }
                    Ok(offen)
                }
                .await;

                let offen = uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await?;
                Ok(offen.into_iter().map(Into::into).collect())
            })
            .await
    }

    /// The public Anlagen directory: active Anlagen with a short Parkour list.
    ///
    /// Needs no identity, and carries no contact data (see [`AnlageÖffentlich`]).
    pub async fn anlagen_verzeichnis(
        &self,
        auth: AuthContext,
    ) -> ResultReport<Vec<AnlageÖffentlich>, StammdatenFehler> {
        self.app
            .execute_fn(auth, async |ctx| {
                let uow = ctx
                    .enter()
                    .await
                    .change_context(StammdatenFehler::AnlageLaden)?;

                let ergebnis = async {
                    let anlagen = uow
                        .anlagen()
                        .find_aktiv()
                        .await
                        .map_err(IntoReport::into_report)
                        .change_context(StammdatenFehler::AnlageLaden)?;

                    Ok(anlagen
                        .into_iter()
                        .map(|anlage| {
                            let parkour = anlage
                                .parkour()
                                .iter()
                                .filter(|parkour| !parkour.ist_archiviert())
                                .map(|parkour| ParkourKurzAnsicht {
                                    id: parkour.id(),
                                    name: parkour.name().clone(),
                                    anzahl_pflöcke: parkour.pflöcke().len(),
                                    anzahl_ziele: parkour.platzierungen().len(),
                                })
                                .collect();
                            AnlageÖffentlichAnsicht {
                                anlage_id: anlage.id(),
                                name: anlage.name().clone(),
                                ort: anlage.adresse().ort().to_owned(),
                                ländercode: anlage.adresse().ländercode(),
                                parkour,
                            }
                            .into()
                        })
                        .collect())
                }
                .await;

                uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
            })
            .await
    }

    // ---------------------------------------------------------------- Runde

    /// Start a Runde; the snapshot is captured in the same transaction (AC-7).
    pub async fn runde_starten(
        &self,
        auth: AuthContext,
        anfrage: requests::RundeStartenRequest,
    ) -> ResultReport<Runde, RundeStartenFehler> {
        let runde = self.app.execute(auth, RundeStarten::from(anfrage)).await?;
        Ok(runde.into())
    }

    /// One Runde with both Wertungen; owner or Systemadministrator (AC-9, AC-13).
    pub async fn runde_anzeigen(
        &self,
        auth: AuthContext,
        runde_id: RundeId,
    ) -> ResultReport<Runde, RundeAnzeigenFehler> {
        self.app
            .execute_fn(auth, async |ctx| {
                let benutzer_id = ctx
                    .auth()
                    .require_benutzer_id()
                    .change_context(RundeAnzeigenFehler::NichtAngemeldet)?;

                let uow = ctx
                    .enter()
                    .await
                    .change_context(RundeAnzeigenFehler::RundeLaden)?;

                let ergebnis = async {
                    let runde = match uow.runden().find_by_id(runde_id).await {
                        Ok(runde) => runde,
                        Err(fehler) if ist_nicht_gefunden(&fehler) => {
                            return Err(Report::new(RundeAnzeigenFehler::NichtGefunden));
                        }
                        Err(fehler) => {
                            return Err(fehler).change_context(RundeAnzeigenFehler::RundeLaden);
                        }
                    };

                    // Owner or Systemadministrator. An Anlage's staff never reads another
                    // Benutzer's Runde (AC-13), which is why the AnlageVerwaltung grants
                    // nothing here.
                    if !runde.gehört_zu(benutzer_id) {
                        let benutzer = uow
                            .benutzer()
                            .find_by_id(benutzer_id)
                            .await
                            .map_err(IntoReport::into_report)
                            .change_context(RundeAnzeigenFehler::BenutzerLaden)?;
                        if !benutzer.ist_systemadministrator() {
                            return Err(Report::new(RundeAnzeigenFehler::NichtErlaubt));
                        }
                    }

                    let schüsse: Vec<logandaim_core::domain::Schuss> = uow
                        .schüsse()
                        .find_by_runde(runde_id)
                        .await
                        .map_err(IntoReport::into_report)
                        .change_context(RundeAnzeigenFehler::SchüsseLaden)?
                        .into_iter()
                        .map(|schuss| schuss.into_data())
                        .collect();

                    Ok(RundeAnsicht::neu(runde.into_data(), schüsse))
                }
                .await;

                let ansicht = uow
                    .finish(ergebnis, RundeAnzeigenFehler::RundeLaden)
                    .await?;
                Ok(ansicht.into())
            })
            .await
    }

    /// The caller's own Runden, newest first.
    pub async fn runden_anzeigen(
        &self,
        auth: AuthContext,
    ) -> ResultReport<Vec<RundeBasic>, RundenAnzeigenFehler> {
        self.app
            .execute_fn(auth, async |ctx| {
                let benutzer_id = ctx
                    .auth()
                    .require_benutzer_id()
                    .change_context(RundenAnzeigenFehler::NichtAngemeldet)?;

                let uow = ctx
                    .enter()
                    .await
                    .change_context(RundenAnzeigenFehler::Laden)?;

                let ergebnis = async {
                    let runden = uow
                        .runden()
                        .find_by_benutzer(benutzer_id)
                        .await
                        .map_err(IntoReport::into_report)
                        .change_context(RundenAnzeigenFehler::Laden)?
                        .into_iter()
                        .map(|runde| runde.into_data())
                        .collect();
                    Ok(runden_neueste_zuerst(runden))
                }
                .await;

                let runden = uow.finish(ergebnis, RundenAnzeigenFehler::Laden).await?;
                Ok(runden.into_iter().map(Into::into).collect())
            })
            .await
    }

    pub async fn runde_abschliessen(
        &self,
        auth: AuthContext,
        runde_id: RundeId,
    ) -> ResultReport<Runde, RundeAbschliessenFehler> {
        let runde = self
            .app
            .execute(auth, RundeAbschliessen { runde_id })
            .await?;
        Ok(runde.into())
    }

    // ---------------------------------------------------------------- Schuss

    /// Log one arrow. The owner comes from the token, never from the body (AC-9).
    pub async fn schuss_registrieren(
        &self,
        auth: AuthContext,
        runde_id: RundeId,
        anfrage: requests::SchussErstellungRequest,
    ) -> ResultReport<Schuss, SchussRegistrierenFehler> {
        let use_case: SchussRegistrieren = anfrage.in_use_case(runde_id)?;
        let schuss = self.app.execute(auth, use_case).await?;
        Ok(schuss.into())
    }

    // ---------------------------------------------------------------- Registration

    /// Step 1. Always answers `Ok`: a taken address is not disclosed (AC-2).
    pub async fn registrierung_starten(
        &self,
        anfrage: requests::RegistrierungStartRequest,
        client_key: ClientKey,
    ) -> ResultReport<(), RegistrierungStartenFehler> {
        let use_case: RegistrierungStarten = anfrage.in_use_case(client_key)?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    /// Step 2. Creates the Benutzer, its Konto, and the first session.
    pub async fn registrierung_abschließen(
        &self,
        anfrage: requests::RegistrierungAbschlussRequest,
        client_key: ClientKey,
    ) -> ResultReport<SessionAusgestellt, RegistrierungAbschliessenFehler> {
        let use_case: RegistrierungAbschließen = anfrage.in_use_case(client_key)?;
        let ausgestellt = self.app.execute(AuthContext::Anonymous, use_case).await?;
        Ok(ausgestellt)
    }

    // ---------------------------------------------------------------- Login

    pub async fn anmelden(
        &self,
        anfrage: requests::AnmeldungRequest,
        client_key: ClientKey,
    ) -> ResultReport<SessionAusgestellt, AnmeldungFehler> {
        let use_case: Anmeldung = anfrage.in_use_case(client_key)?;
        let ausgestellt = self.app.execute(AuthContext::Anonymous, use_case).await?;
        Ok(ausgestellt)
    }

    // TODO: serializable dto for session ausgestellt (as per api standard)
    /// Rotate the refresh token. The presented one is spent; a replay ends the family.
    pub async fn session_erneuern(
        &self,
        refresh_token: String,
        client_key: ClientKey,
    ) -> ResultReport<SessionAusgestellt, ErneuerungFehler> {
        let jetzt = self.app.clock.now();
        self.app
            .throttle
            .check(&client_key, ThrottleAction::Erneuerung, jetzt)
            .await
            .change_context(ErneuerungFehler::Gedrosselt)?;

        let refresh_token = match logandaim_core::domain::konto::RefreshToken::try_from(
            refresh_token,
        ) {
            Ok(token) => token,
            Err(_) => {
                if let Err(fehler) = self
                    .app
                    .throttle
                    .increment(&client_key, ThrottleAction::Erneuerung, jetzt)
                    .await
                {
                    warn!(fehler = %fehler, "der Erneuerungsversuch konnte nicht gezählt werden");
                }
                return Err(Report::new(ErneuerungFehler::SessionUngültig));
            }
        };

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
        Ok(ausgestellt)
    }

    pub async fn abmelden(&self, auth: AuthContext) -> ResultReport<(), AbmeldungFehler> {
        self.app.execute(auth, Abmeldung).await
    }

    // ---------------------------------------------------------------- Passwort

    pub async fn passwort_vergessen(
        &self,
        anfrage: requests::PasswortVergessenRequest,
        client_key: ClientKey,
    ) -> ResultReport<(), PasswortVergessenFehler> {
        let use_case: PasswortVergessen = anfrage.in_use_case(client_key)?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    /// Single use, and ends every session of the Benutzer.
    pub async fn passwort_wiederherstellen(
        &self,
        anfrage: requests::PasswortWiederherstellenRequest,
    ) -> ResultReport<(), PasswortWiederherstellenFehler> {
        let use_case: PasswortWiederherstellen = anfrage.in_use_case()?;
        self.app.execute(AuthContext::Anonymous, use_case).await
    }

    // ---------------------------------------------------------------- Benutzer

    /// Resolve one account by exact email for assigning AnlageVerwaltung. A Systemadministrator
    /// or the active Anlage's Administrator may search; no partial/bulk enumeration is exposed.
    pub async fn benutzer_nach_email_suchen(
        &self,
        auth: AuthContext,
        anlage_id: AnlageId,
        e_mail_adresse: String,
    ) -> ResultReport<Vec<Benutzer>, StammdatenFehler> {
        self.app
            .execute_fn(auth, async |ctx| {
                let e_mail_adresse = EmailAdresse::normalisiert(e_mail_adresse)
                    .map_err(|_| Report::new(StammdatenFehler::UngültigeAnfrage))?;
                let benutzer_id = ctx
                    .auth()
                    .require_benutzer_id()
                    .change_context(StammdatenFehler::NichtAngemeldet)?;
                let uow = ctx
                    .enter()
                    .await
                    .change_context(StammdatenFehler::AnlageLaden)?;

                let ergebnis = async {
                    let benutzer = uow
                        .benutzer()
                        .find_by_id(benutzer_id)
                        .await
                        .map_err(error_stack::IntoReport::into_report)
                        .change_context(StammdatenFehler::BenutzerLaden)?;
                    let anlage = match uow.anlagen().find_by_id(anlage_id).await {
                        Ok(anlage) => anlage,
                        Err(fehler)
                            if fehler
                                .downcast_ref::<logandaim_core::ports::RepositoryError>()
                                .is_some_and(|error| {
                                    matches!(
                                        error,
                                        logandaim_core::ports::RepositoryError::NotFound
                                    )
                                }) =>
                        {
                            return Err(error_stack::Report::new(StammdatenFehler::NichtGefunden));
                        }
                        Err(fehler) => {
                            return Err(fehler.change_context(StammdatenFehler::AnlageLaden));
                        }
                    };
                    if !darf_anlage_verwalten(&benutzer, &anlage) {
                        return Err(error_stack::Report::new(StammdatenFehler::NichtErlaubt));
                    }
                    if anlage.ist_archiviert() {
                        return Err(error_stack::Report::new(StammdatenFehler::Archiviert));
                    }

                    let email = &e_mail_adresse;
                    match uow.konten().find_by_email(email).await {
                        Ok(konto) => {
                            if anlage
                                .verwaltungen()
                                .iter()
                                .any(|verwaltung| verwaltung.benutzer_id() == konto.benutzer_id())
                            {
                                return Ok(Vec::new());
                            }
                            let benutzer = uow
                                .benutzer()
                                .find_by_id(konto.benutzer_id())
                                .await
                                .map_err(error_stack::IntoReport::into_report)
                                .change_context(StammdatenFehler::BenutzerLaden)?;
                            Ok(vec![(benutzer.into_data(), email.clone()).into()])
                        }
                        Err(fehler)
                            if fehler
                                .downcast_ref::<logandaim_core::ports::RepositoryError>()
                                .is_some_and(|error| {
                                    matches!(
                                        error,
                                        logandaim_core::ports::RepositoryError::NotFound
                                    )
                                }) =>
                        {
                            Ok(Vec::new())
                        }
                        Err(fehler) => Err(fehler.change_context(StammdatenFehler::BenutzerLaden)),
                    }
                }
                .await;

                uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
            })
            .await
    }

    pub async fn benutzer_löschen(
        &self,
        auth: AuthContext,
        anfrage: requests::BenutzerLöschungRequest,
    ) -> ResultReport<(), BenutzerLöschenFehler> {
        let use_case: BenutzerLöschen = anfrage.in_use_case()?;
        self.app.execute(auth, use_case).await
    }
}
