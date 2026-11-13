//! Inbound requests of master data.
//!
//! Every conversion performs the domain validation exactly once, at the edge, and folds a
//! failure into the use case's own `UngültigeAnfrage` variant — so no route invents a second
//! error vocabulary. `AnlageAdresse`, `NonEmptyString`, `Farbe`, `Abstand`, and
//! `WebsiteUrl` are validated newtypes: the DTOs never re-check them.
//!
//! A refused parse attaches what it was parsing, so a report names the field and the index
//! that failed rather than only the top-level request.

use std::num::NonZeroU32;

use error_stack::{Report, ResultExt};
use uuid::Uuid;

use logandaim_core::{
    domain::{
        self, AnlageId, AnlageZielId, NonEmptyString, ParkourId, ParkourPflockId,
        anlage::WebsiteUrl, konto::BenutzerId,
    },
    service::{
        AnlageBeschreibendeDatenÄndern, AnlageErstellen, AnlageVerwaltungGewähren,
        AnlageVerwaltungÄndern, ParkourHinzufügen, ParkourSetupÄndern, ParkourUmbenennen,
        StammdatenFehler, ZielErsetzen, ZielHinzufügen, ZielÄndern,
    },
};

use crate::schema::{
    AnlageAdresse, AnlageKontakt, AnlageVerwaltungRolle, Farbe, Laendercode, ParkourPflock,
    ZielPfloecke, Zielplatzierung,
};

fn name(wert: String) -> Result<NonEmptyString, Report<StammdatenFehler>> {
    NonEmptyString::neu(wert)
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Anlage: Name")
}

fn adresse(wert: AnlageAdresse) -> Result<domain::AnlageAdresse, Report<StammdatenFehler>> {
    let laendercode = match wert.laendercode {
        Laendercode::AT => domain::Ländercode::AT,
        Laendercode::DE => domain::Ländercode::DE,
        Laendercode::CH => domain::Ländercode::CH,
    };
    domain::AnlageAdresse::neu(wert.strasse, wert.postleitzahl, wert.ort, laendercode)
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Anlage: Adresse")
}

fn kontakt(wert: AnlageKontakt) -> Result<domain::AnlageKontakt, Report<StammdatenFehler>> {
    let email = wert
        .email
        .map(TryInto::try_into)
        .transpose()
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Anlage: Kontakt-E-Mail-Adresse")?;
    let telefon = wert
        .telefon
        .map(domain::Mobilnummer::new)
        .transpose()
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Anlage: Kontakt-Telefonnummer")?;
    let website = wert
        .website
        .map(WebsiteUrl::neu)
        .transpose()
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Anlage: Kontakt-Website")?;
    Ok(domain::AnlageKontakt::neu(
        wert.name, email, telefon, website,
    ))
}

fn tiername(wert: String) -> Result<NonEmptyString, Report<StammdatenFehler>> {
    NonEmptyString::neu(wert)
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Ziel: Tiername")
}

fn parkour_name(wert: String) -> Result<NonEmptyString, Report<StammdatenFehler>> {
    NonEmptyString::neu(wert)
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Parkour: Name")
}

fn farbe(wert: Farbe) -> Result<domain::Farbe, Report<StammdatenFehler>> {
    wert.try_into()
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("Pflock: Farbe")
}

fn pflock(
    wert: crate::schema::ParkourPflock,
) -> Result<domain::ParkourPflock, Report<StammdatenFehler>> {
    Ok(domain::ParkourPflock::neu(
        ParkourPflockId(wert.id),
        farbe(wert.farbe)?,
    ))
}

impl TryFrom<ZielPfloecke> for domain::ZielPflöcke {
    type Error = Report<StammdatenFehler>;

    fn try_from(wert: ZielPfloecke) -> Result<Self, Self::Error> {
        match wert {
            ZielPfloecke::InheritFromParkour(_) => Ok(Self::InheritFromParkour),
            ZielPfloecke::Explicit(explicit) => Self::überschreibe(pflöcke(explicit.pfloecke)?)
                .change_context(StammdatenFehler::UngültigeAnfrage)
                .attach("Ziel: eigene Pflöcke"),
        }
    }
}

impl TryFrom<Zielplatzierung> for domain::Zielplatzierung {
    type Error = Report<StammdatenFehler>;

    fn try_from(wert: Zielplatzierung) -> Result<Self, Self::Error> {
        // An absent distance is simply not configured for that Pflock.
        let abstände = wert
            .abstaende
            .into_iter()
            .filter_map(|abstand| abstand.meter.map(|meter| (abstand.pflock_id, meter)))
            .map(|(pflock_id, meter)| {
                Ok((
                    ParkourPflockId(pflock_id),
                    domain::Abstand::neu(meter)
                        .change_context(StammdatenFehler::UngültigeAnfrage)
                        .attach(format!("Ziel: Abstand für Pflock {pflock_id:?}"))?,
                ))
            })
            .collect::<Result<Vec<_>, Report<StammdatenFehler>>>()?;
        let pflöcke = wert.pfloecke.try_into()?;
        domain::Zielplatzierung::neu_mit_abständen_und_pflöcken(
            AnlageZielId(wert.ziel_id),
            pflöcke,
            abstände,
            &[],
        )
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach(format!("Ziel {}", wert.ziel_id))
    }
}

fn anlage_kontext(anlage_id: AnlageId) -> String {
    format!("Anlage {:?}", anlage_id.0)
}

fn pflöcke(
    werte: Vec<ParkourPflock>,
) -> Result<Vec<domain::ParkourPflock>, Report<StammdatenFehler>> {
    werte
        .into_iter()
        .enumerate()
        .map(|(index, wert)| pflock(wert).attach(format!("Parkour: Pflock {index}")))
        .collect()
}

fn platzierungen(
    werte: Vec<Zielplatzierung>,
) -> Result<Vec<domain::Zielplatzierung>, Report<StammdatenFehler>> {
    werte
        .into_iter()
        .enumerate()
        .map(|(index, wert)| {
            domain::Zielplatzierung::try_from(wert).attach(format!("Parkour: Platzierung {index}"))
        })
        .collect()
}

// ------------------------------------------------------------------ Anlage

/// Create an Anlage. Systemadministrator only.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageErstellungRequest {
    /// Shown name of the new Anlage.
    pub name: String,
    /// Its structured address; every address field is required.
    pub adresse: AnlageAdresse,
    /// Optional contact details.
    #[serde(default)]
    pub kontakt: AnlageKontakt,
}

impl AnlageErstellungRequest {
    pub fn in_use_case(self) -> Result<AnlageErstellen, Report<StammdatenFehler>> {
        Ok(AnlageErstellen {
            name: name(self.name)?,
            adresse: adresse(self.adresse)?,
            kontakt: kontakt(self.kontakt)?,
        })
    }
}

/// Rewrite the Anlage's descriptive fields.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageAenderungRequest {
    /// Shown name of the Anlage.
    pub name: String,
    /// Its structured address; every address field is required.
    pub adresse: AnlageAdresse,
    /// Optional contact details.
    #[serde(default)]
    pub kontakt: AnlageKontakt,
}

impl AnlageAenderungRequest {
    pub fn in_use_case(
        self,
        anlage_id: AnlageId,
    ) -> Result<AnlageBeschreibendeDatenÄndern, Report<StammdatenFehler>> {
        Ok(AnlageBeschreibendeDatenÄndern {
            anlage_id,
            name: name(self.name)?,
            adresse: adresse(self.adresse)?,
            kontakt: kontakt(self.kontakt)?,
        })
    }
}

/// Grant or change one role in an Anlage.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageVerwaltungRequest {
    /// The Benutzer that should hold the role.
    pub benutzer_id: Uuid,
    /// The role to assert.
    pub rolle: AnlageVerwaltungRolle,
}

fn rolle(
    wert: AnlageVerwaltungRolle,
) -> Result<domain::AnlageVerwaltungRolle, Report<StammdatenFehler>> {
    wert.try_into()
        .change_context(StammdatenFehler::UngültigeAnfrage)
        .attach("AnlageVerwaltung: Rolle")
}

impl AnlageVerwaltungRequest {
    pub fn als_gewähren(
        self,
        anlage_id: AnlageId,
    ) -> Result<AnlageVerwaltungGewähren, Report<StammdatenFehler>> {
        Ok(AnlageVerwaltungGewähren {
            anlage_id,
            benutzer_id: BenutzerId(self.benutzer_id),
            rolle: rolle(self.rolle)?,
        })
    }

    /// Changing a role names its target in the path, so only the role is read from the body.
    pub fn als_ändern(
        self,
        anlage_id: AnlageId,
        benutzer_id: BenutzerId,
    ) -> Result<AnlageVerwaltungÄndern, Report<StammdatenFehler>> {
        Ok(AnlageVerwaltungÄndern {
            anlage_id,
            benutzer_id,
            rolle: rolle(self.rolle)?,
        })
    }
}

// ------------------------------------------------------------------ Ziel

/// Add or edit a Ziel.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZielRequest {
    /// Animal name shown to the shooter.
    pub tiername: String,
    /// 1-based ordinal that tells duplicate animal names apart ("Reh 1", "Reh 2").
    ///
    /// Omitted on creation, where the Anlage derives the next free ordinal from the
    /// Tiername. Editing refuses a request without one, because correcting the ordinal is
    /// the user's own act.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zielnummer: Option<u32>,
}

fn zielnummer(wert: Option<u32>) -> Result<NonZeroU32, Report<StammdatenFehler>> {
    wert.and_then(NonZeroU32::new)
        .ok_or_else(|| Report::new(StammdatenFehler::UngültigeAnfrage))
        .attach("Ziel: Zielnummer muss 1-basiert sein")
}

impl ZielRequest {
    pub fn als_hinzufügen(
        self,
        anlage_id: AnlageId,
    ) -> Result<ZielHinzufügen, Report<StammdatenFehler>> {
        Ok(ZielHinzufügen {
            anlage_id,
            tiername: tiername(self.tiername)?,
        })
    }

    pub fn als_ändern(
        self,
        anlage_id: AnlageId,
        ziel_id: AnlageZielId,
    ) -> Result<ZielÄndern, Report<StammdatenFehler>> {
        Ok(ZielÄndern {
            anlage_id,
            ziel_id,
            tiername: tiername(self.tiername)?,
            zielnummer: zielnummer(self.zielnummer)?,
        })
    }

    pub fn als_ersetzen(
        self,
        anlage_id: AnlageId,
        ziel_id: AnlageZielId,
    ) -> Result<ZielErsetzen, Report<StammdatenFehler>> {
        Ok(ZielErsetzen {
            anlage_id,
            ziel_id,
            tiername: tiername(self.tiername)?,
        })
    }
}

// ------------------------------------------------------------------ Parkour

/// Create a Parkour with its Pflock setup and its route.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParkourErstellungRequest {
    /// Shown name of the new Parkour.
    pub name: String,
    /// The Pflöcke, far to near.
    pub pfloecke: Vec<ParkourPflock>,
    /// The Ziele on the route, in walking order. May be empty.
    #[serde(default)]
    pub platzierungen: Vec<Zielplatzierung>,
}

impl ParkourErstellungRequest {
    pub fn in_use_case(
        self,
        anlage_id: AnlageId,
    ) -> Result<ParkourHinzufügen, Report<StammdatenFehler>> {
        Ok(ParkourHinzufügen {
            anlage_id,
            name: parkour_name(self.name).attach(anlage_kontext(anlage_id))?,
            pflöcke: pflöcke(self.pfloecke).attach(anlage_kontext(anlage_id))?,
            platzierungen: platzierungen(self.platzierungen).attach(anlage_kontext(anlage_id))?,
        })
    }
}

/// Replace a Parkour' whole shooting setup.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParkourSetupRequest {
    /// The Pflöcke, far to near.
    pub pfloecke: Vec<ParkourPflock>,
    /// The Ziele on the route, in walking order. May be empty.
    #[serde(default)]
    pub platzierungen: Vec<Zielplatzierung>,
}

impl ParkourSetupRequest {
    pub fn in_use_case(
        self,
        anlage_id: AnlageId,
        parkour_id: ParkourId,
    ) -> Result<ParkourSetupÄndern, Report<StammdatenFehler>> {
        Ok(ParkourSetupÄndern {
            anlage_id,
            parkour_id,
            pflöcke: pflöcke(self.pfloecke)
                .attach(anlage_kontext(anlage_id))
                .attach(format!("Parkour {parkour_id:?}"))?,
            platzierungen: platzierungen(self.platzierungen)
                .attach(anlage_kontext(anlage_id))
                .attach(format!("Parkour {parkour_id:?}"))?,
        })
    }
}

/// Rename a Parkour. Existing snapshots keep the name they captured.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParkourUmbenennenRequest {
    /// The new shown name.
    pub name: String,
}

impl ParkourUmbenennenRequest {
    pub fn in_use_case(
        self,
        anlage_id: AnlageId,
        parkour_id: ParkourId,
    ) -> Result<ParkourUmbenennen, Report<StammdatenFehler>> {
        Ok(ParkourUmbenennen {
            anlage_id,
            parkour_id,
            name: parkour_name(self.name)
                .attach(anlage_kontext(anlage_id))
                .attach(format!("Parkour {parkour_id:?}"))?,
        })
    }
}
