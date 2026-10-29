//! Inbound requests of master data.
//!
//! Every conversion performs the domain validation exactly once, at the edge, and folds a
//! failure into the use case's own `UngültigeAnfrage` variant — so no route invents a second
//! error vocabulary. `AnlageAdresse`, `Tiername`, `Zielgruppe`, `PflockBezeichnung`,
//! `PflockFarbe`, and `Abstand` are validated newtypes: the DTOs never re-check them.

use error_stack::Report;
use uuid::Uuid;

use logandaim_core::{
    domain::{
        Abstand, AnlageAdresse, AnlageId, AnlageKontakt, AnlageName, AnlageVerwaltungRolle,
        Ländercode, ParcoursId, ParcoursName, Pflock, PflockBezeichnung, PflockFarbe, PflockId,
        Tiername, ZielId, Zielgruppe, Zielplatzierung, anlage::Website, konto::BenutzerId,
    },
    service::{
        AnlageBeschreibendeDatenÄndern, AnlageErstellen, AnlageVerwaltungGewähren,
        AnlageVerwaltungÄndern, ParcoursHinzufügen, ParcoursSetupÄndern, ParcoursUmbenennen,
        StammdatenFehler, ZielErsetzen, ZielHinzufügen, ZielÄndern,
    },
};

use crate::schema::{
    AnlageAdresse as AnlageAdresseSchema, AnlageKontakt as AnlageKontaktSchema,
    AnlageVerwaltungRolle as RolleSchema, Pflock as PflockSchema,
    Zielplatzierung as ZielplatzierungSchema,
};

fn ungültig() -> Report<StammdatenFehler> {
    Report::new(StammdatenFehler::UngültigeAnfrage)
}

fn name(wert: String) -> Result<AnlageName, Report<StammdatenFehler>> {
    AnlageName::neu(wert).map_err(|_| ungültig())
}

fn adresse(wert: AnlageAdresseSchema) -> Result<AnlageAdresse, Report<StammdatenFehler>> {
    let ländercode = Ländercode::from_str(&wert.ländercode).map_err(|_| ungültig())?;
    AnlageAdresse::neu(wert.straße, wert.postleitzahl, wert.ort, ländercode).map_err(|_| ungültig())
}

fn kontakt(wert: AnlageKontaktSchema) -> Result<AnlageKontakt, Report<StammdatenFehler>> {
    let email = match wert.email {
        Some(email) => Some(email.try_into().map_err(|_| ungültig())?),
        None => None,
    };
    let telefon = match wert.telefon {
        Some(nummer) => {
            Some(logandaim_core::domain::Mobilnummer::new(nummer).map_err(|_| ungültig())?)
        }
        None => None,
    };
    let website = match wert.website {
        Some(website) => Some(Website::neu(website).map_err(|_| ungültig())?),
        None => None,
    };
    Ok(AnlageKontakt::neu(wert.name, email, telefon, website))
}

fn tiername(wert: String) -> Result<Tiername, Report<StammdatenFehler>> {
    Tiername::neu(wert).map_err(|_| ungültig())
}

fn zielgruppe(wert: u8) -> Result<Zielgruppe, Report<StammdatenFehler>> {
    Zielgruppe::neu(wert).map_err(|_| ungültig())
}

fn parcours_name(wert: String) -> Result<ParcoursName, Report<StammdatenFehler>> {
    ParcoursName::neu(wert).map_err(|_| ungültig())
}

fn pflock(wert: PflockSchema) -> Result<Pflock, Report<StammdatenFehler>> {
    Ok(Pflock::neu(
        PflockId(wert.id),
        PflockBezeichnung::neu(wert.bezeichnung).map_err(|_| ungültig())?,
        PflockFarbe::neu(wert.farbe).map_err(|_| ungültig())?,
    ))
}

fn platzierung(wert: ZielplatzierungSchema) -> Result<Zielplatzierung, Report<StammdatenFehler>> {
    let abstände = wert
        .abstände
        .into_iter()
        .map(|abstand| {
            Ok((
                PflockId(abstand.pflock_id),
                Abstand::neu(abstand.meter).map_err(|_| ungültig())?,
            ))
        })
        .collect::<Result<Vec<_>, Report<StammdatenFehler>>>()?;
    Zielplatzierung::neu(ZielId(wert.ziel_id), abstände).map_err(|_| ungültig())
}

// ------------------------------------------------------------------ Anlage

/// Create an Anlage (AC-1).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageErstellungRequest {
    pub name: String,
    pub adresse: AnlageAdresseSchema,
    #[serde(default)]
    pub kontakt: AnlageKontaktSchema,
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

/// Edit the Anlage's descriptive fields (AC-3).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageÄnderungRequest {
    pub name: String,
    pub adresse: AnlageAdresseSchema,
    #[serde(default)]
    pub kontakt: AnlageKontaktSchema,
}

impl AnlageÄnderungRequest {
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

/// Grant or change one AnlageVerwaltung (AC-2).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageVerwaltungRequest {
    pub benutzer_id: Uuid,
    pub rolle: RolleSchema,
}

fn rolle(wert: RolleSchema) -> Result<AnlageVerwaltungRolle, Report<StammdatenFehler>> {
    wert.try_into().map_err(|_| ungültig())
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

/// Add or edit a Ziel (AC-5).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZielRequest {
    pub tiername: String,
    pub zielgruppe: u8,
}

impl ZielRequest {
    pub fn als_hinzufügen(
        self,
        anlage_id: AnlageId,
    ) -> Result<ZielHinzufügen, Report<StammdatenFehler>> {
        Ok(ZielHinzufügen {
            anlage_id,
            tiername: tiername(self.tiername)?,
            zielgruppe: zielgruppe(self.zielgruppe)?,
        })
    }

    pub fn als_ändern(
        self,
        anlage_id: AnlageId,
        ziel_id: ZielId,
    ) -> Result<ZielÄndern, Report<StammdatenFehler>> {
        Ok(ZielÄndern {
            anlage_id,
            ziel_id,
            tiername: tiername(self.tiername)?,
            zielgruppe: zielgruppe(self.zielgruppe)?,
        })
    }

    pub fn als_ersetzen(
        self,
        anlage_id: AnlageId,
        ziel_id: ZielId,
    ) -> Result<ZielErsetzen, Report<StammdatenFehler>> {
        Ok(ZielErsetzen {
            anlage_id,
            ziel_id,
            tiername: tiername(self.tiername)?,
            zielgruppe: zielgruppe(self.zielgruppe)?,
        })
    }
}

// ------------------------------------------------------------------ Parcours

/// Create a Parcours with its Pflock setup and route (AC-4, AC-6).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParcoursErstellungRequest {
    pub name: String,
    pub pflöcke: Vec<PflockSchema>,
    #[serde(default)]
    pub platzierungen: Vec<ZielplatzierungSchema>,
}

impl ParcoursErstellungRequest {
    pub fn in_use_case(
        self,
        anlage_id: AnlageId,
    ) -> Result<ParcoursHinzufügen, Report<StammdatenFehler>> {
        Ok(ParcoursHinzufügen {
            anlage_id,
            name: parcours_name(self.name)?,
            pflöcke: self
                .pflöcke
                .into_iter()
                .map(pflock)
                .collect::<Result<Vec<_>, _>>()?,
            platzierungen: self
                .platzierungen
                .into_iter()
                .map(platzierung)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

/// Replace a Parcours' whole shooting setup (AC-6, AC-7).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParcoursSetupRequest {
    pub pflöcke: Vec<PflockSchema>,
    #[serde(default)]
    pub platzierungen: Vec<ZielplatzierungSchema>,
}

impl ParcoursSetupRequest {
    pub fn in_use_case(
        self,
        anlage_id: AnlageId,
        parcours_id: ParcoursId,
    ) -> Result<ParcoursSetupÄndern, Report<StammdatenFehler>> {
        Ok(ParcoursSetupÄndern {
            anlage_id,
            parcours_id,
            pflöcke: self
                .pflöcke
                .into_iter()
                .map(pflock)
                .collect::<Result<Vec<_>, _>>()?,
            platzierungen: self
                .platzierungen
                .into_iter()
                .map(platzierung)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

/// Rename a Parcours (AC-9: existing snapshots keep the old name).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParcoursUmbenennenRequest {
    pub name: String,
}

impl ParcoursUmbenennenRequest {
    pub fn in_use_case(
        self,
        anlage_id: AnlageId,
        parcours_id: ParcoursId,
    ) -> Result<ParcoursUmbenennen, Report<StammdatenFehler>> {
        Ok(ParcoursUmbenennen {
            anlage_id,
            parcours_id,
            name: parcours_name(self.name)?,
        })
    }
}
