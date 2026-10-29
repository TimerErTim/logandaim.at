//! Outbound schemas of master data: the Anlage with its Ziele and Parcours, the role
//! assignments, and the public Parcours list.
//!
//! The domain stays free of OpenAPI and serde; every struct here is an API mirror, built by
//! the `From` impls at the bottom so a handler never assembles one by hand.

use uuid::Uuid;

use logandaim_core::{
    domain,
    service::{
        AnlageAnsicht, AnlagenZeile as AnlagenZeileAnsicht,
        ParcoursÖffentlich as ParcoursÖffentlichAnsicht,
    },
};

use crate::schema::EmailAdresse;

// ------------------------------------------------------------------ Anlage

/// Country codes currently accepted for Anlage addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CountryCode {
    AT,
    DE,
    CH,
}

/// One structured address. Every field is required (SPEC-stammdaten AC-1).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageAdresse {
    pub straße: String,
    pub postleitzahl: String,
    pub ort: String,
    pub ländercode: CountryCode,
}

/// Optional contact details; each may be absent on its own (AC-1).
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageKontakt {
    pub name: Option<String>,
    pub email: Option<EmailAdresse>,
    pub telefon: Option<String>,
    pub website: Option<String>,
}

/// `Aktiv` or `Archiviert`. Archived reads answer `200` with this state; writes to them
/// answer `409` (AC-10, AC-11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AnlageZustand {
    Aktiv,
    Archiviert,
}

/// The Anlage's role inside one Anlage (AC-2, AC-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AnlageVerwaltungRolle {
    Administrator,
    Mitarbeiter,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageVerwaltung {
    pub benutzer_id: Uuid,
    pub rolle: AnlageVerwaltungRolle,
}

// ------------------------------------------------------------------ Ziel

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ziel {
    pub id: Uuid,
    pub tiername: String,
    /// 1–4 (AC-5).
    pub zielgruppe: u8,
    pub zustand: AnlageZustand,
    /// Where this physical Ziel currently lies, across this Anlage's active Parcours.
    pub verwendungen: Vec<Zielverwendung>,
}

/// One Parcours' use of one Ziel, with that Parcours' own distances (AC-8).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zielverwendung {
    pub parcours_id: Uuid,
    pub parcours_name: String,
    pub parcours_ordnung: usize,
    /// 0-based position in that Parcours' route.
    pub position: usize,
    /// Pflock label to metres, far to near.
    pub abstände: Vec<PflockAbstand>,
}

/// One Pflock colour with the distance captured for it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PflockAbstand {
    /// The Pflock's colour as `#rrggbb`.
    pub pflock: String,
    pub meter: u32,
}

// ------------------------------------------------------------------ Parcours

/// One shooting position: its colour (`#rrggbb`) identifies it, and no label exists.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pflock {
    pub id: Uuid,
    /// Caller data, never a design token (AC-6).
    pub farbe: String,
}

/// One Ziel's place in a Parcours, with its distances keyed by Pflock identity.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zielplatzierung {
    pub ziel_id: Uuid,
    /// Pflock id to metres. A missing entry means "no distance configured".
    pub abstände: Vec<ZielplatzierungAbstand>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZielplatzierungAbstand {
    pub pflock_id: Uuid,
    pub meter: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcours {
    pub id: Uuid,
    pub name: String,
    pub zustand: AnlageZustand,
    /// Far to near, never inferred from a colour (AC-6).
    pub pflöcke: Vec<Pflock>,
    /// In route order.
    pub platzierungen: Vec<Zielplatzierung>,
}

/// One active Parcours with the Anlage that holds it, for starting a Runde (AC-13).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParcoursÖffentlich {
    pub anlage_id: Uuid,
    pub anlage_name: String,
    pub parcours: Parcours,
}

/// One active Parcours as the public directory names it (counts only, no setup).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParcoursKurz {
    pub id: Uuid,
    pub name: String,
    pub anzahl_pflöcke: usize,
    pub anzahl_ziele: usize,
}

/// One active Anlage in the public directory.
///
/// Public, so it carries no contact data: an email or phone number is personal data and no
/// spec approved publishing it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageÖffentlich {
    pub anlage_id: Uuid,
    pub name: String,
    pub ort: String,
    pub ländercode: CountryCode,
    pub parcours: Vec<ParcoursKurz>,
}

// ------------------------------------------------------------------ Anlage views

/// `GET /anlagen/{id}`: the Anlage with its whole master data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anlage {
    pub id: Uuid,
    pub name: String,
    pub adresse: AnlageAdresse,
    pub kontakt: AnlageKontakt,
    pub zustand: AnlageZustand,
    pub verwaltungen: Vec<AnlageVerwaltung>,
    pub ziele: Vec<Ziel>,
    pub parcours: Vec<Parcours>,
}

/// One row of `GET /anlagen`, with the caller's role so the UI can gate its controls.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlagenZeile {
    pub anlage: Anlage,
    pub rolle: Option<AnlageVerwaltungRolle>,
    pub ist_systemadministrator: bool,
}

// ------------------------------------------------------------------ conversions

fn zustand(wert: domain::Zustand) -> AnlageZustand {
    match wert {
        domain::Zustand::Aktiv => AnlageZustand::Aktiv,
        domain::Zustand::Archiviert => AnlageZustand::Archiviert,
    }
}

impl From<domain::AnlageVerwaltungRolle> for AnlageVerwaltungRolle {
    fn from(rolle: domain::AnlageVerwaltungRolle) -> Self {
        match rolle {
            domain::AnlageVerwaltungRolle::Administrator => Self::Administrator,
            domain::AnlageVerwaltungRolle::Mitarbeiter => Self::Mitarbeiter,
        }
    }
}

impl TryFrom<AnlageVerwaltungRolle> for domain::AnlageVerwaltungRolle {
    type Error = crate::errors::ValidationError;

    fn try_from(rolle: AnlageVerwaltungRolle) -> Result<Self, Self::Error> {
        Ok(match rolle {
            AnlageVerwaltungRolle::Administrator => Self::Administrator,
            AnlageVerwaltungRolle::Mitarbeiter => Self::Mitarbeiter,
        })
    }
}

impl From<domain::Ländercode> for CountryCode {
    fn from(code: domain::Ländercode) -> Self {
        match code {
            domain::Ländercode::AT => Self::AT,
            domain::Ländercode::DE => Self::DE,
            domain::Ländercode::CH => Self::CH,
        }
    }
}

impl From<domain::AnlageAdresse> for AnlageAdresse {
    fn from(adresse: domain::AnlageAdresse) -> Self {
        Self {
            straße: adresse.straße().to_owned(),
            postleitzahl: adresse.postleitzahl().to_owned(),
            ort: adresse.ort().to_owned(),
            ländercode: adresse.ländercode().into(),
        }
    }
}

impl From<domain::AnlageKontakt> for AnlageKontakt {
    fn from(kontakt: domain::AnlageKontakt) -> Self {
        Self {
            name: kontakt.name().map(str::to_owned),
            email: kontakt.email().cloned().map(Into::into),
            telefon: kontakt.telefon().map(|nummer| nummer.as_ref().to_owned()),
            website: kontakt.website().map(|website| website.as_str().to_owned()),
        }
    }
}

impl From<domain::AnlageVerwaltung> for AnlageVerwaltung {
    fn from(verwaltung: domain::AnlageVerwaltung) -> Self {
        Self {
            benutzer_id: verwaltung.benutzer_id().0,
            rolle: verwaltung.rolle().into(),
        }
    }
}

impl From<domain::Pflock> for Pflock {
    fn from(pflock: domain::Pflock) -> Self {
        Self {
            id: pflock.id().0,
            farbe: pflock.farbe().as_str().to_owned(),
        }
    }
}

impl From<domain::Zielplatzierung> for Zielplatzierung {
    fn from(platzierung: domain::Zielplatzierung) -> Self {
        Self {
            ziel_id: platzierung.ziel_id().0,
            abstände: platzierung
                .abstände()
                .iter()
                .map(|(pflock_id, abstand)| ZielplatzierungAbstand {
                    pflock_id: pflock_id.0,
                    meter: abstand.meter(),
                })
                .collect(),
        }
    }
}

impl From<domain::Parcours> for Parcours {
    fn from(parcours: domain::Parcours) -> Self {
        Self {
            id: parcours.id().0,
            name: parcours.name().as_str().to_owned(),
            zustand: zustand(parcours.zustand()),
            pflöcke: parcours.pflöcke().iter().cloned().map(Into::into).collect(),
            platzierungen: parcours
                .platzierungen()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<domain::Zielverwendung> for Zielverwendung {
    fn from(verwendung: domain::Zielverwendung) -> Self {
        Self {
            parcours_id: verwendung.parcours_id.0,
            parcours_name: verwendung.parcours_name,
            parcours_ordnung: verwendung.parcours_ordnung,
            position: verwendung.position,
            abstände: verwendung
                .abstände
                .into_iter()
                .map(|(pflock, meter)| PflockAbstand { pflock, meter })
                .collect(),
        }
    }
}

impl From<domain::Ziel> for Ziel {
    fn from(ziel: domain::Ziel) -> Self {
        Self {
            id: ziel.id().0,
            tiername: ziel.tiername().as_str().to_owned(),
            zielgruppe: ziel.zielgruppe().value(),
            zustand: zustand(ziel.zustand()),
            verwendungen: Vec::new(),
        }
    }
}

impl From<domain::Anlage> for Anlage {
    fn from(anlage: domain::Anlage) -> Self {
        let zustand = if anlage.ist_archiviert() {
            AnlageZustand::Archiviert
        } else {
            AnlageZustand::Aktiv
        };
        Self {
            id: anlage.id().0,
            name: anlage.name().as_str().to_owned(),
            adresse: anlage.adresse().clone().into(),
            kontakt: anlage.kontakt().clone().into(),
            zustand,
            verwaltungen: anlage
                .verwaltungen()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            ziele: anlage.ziele().iter().cloned().map(Into::into).collect(),
            parcours: anlage.parcours().iter().cloned().map(Into::into).collect(),
        }
    }
}

impl From<AnlageAnsicht> for Anlage {
    fn from(ansicht: AnlageAnsicht) -> Self {
        // The placement aggregation the admin page renders, attached to each Ziel.
        let verwendungen: std::collections::HashMap<_, _> =
            ansicht.zielverwendungen.into_iter().collect();
        let mut anlage: Anlage = ansicht.anlage.into();
        for ziel in &mut anlage.ziele {
            ziel.verwendungen = verwendungen
                .get(&domain::ZielId(ziel.id))
                .map(|liste| liste.iter().cloned().map(Into::into).collect())
                .unwrap_or_default();
        }
        anlage
    }
}

impl From<AnlagenZeileAnsicht> for AnlagenZeile {
    fn from(zeile: AnlagenZeileAnsicht) -> Self {
        Self {
            anlage: zeile.anlage.into(),
            rolle: zeile.rolle.map(Into::into),
            ist_systemadministrator: zeile.ist_systemadministrator,
        }
    }
}

impl From<ParcoursÖffentlichAnsicht> for ParcoursÖffentlich {
    fn from(parcours: ParcoursÖffentlichAnsicht) -> Self {
        Self {
            anlage_id: parcours.anlage_id.0,
            anlage_name: parcours.anlage_name.as_str().to_owned(),
            parcours: parcours.parcours.into(),
        }
    }
}

impl From<logandaim_core::service::ParcoursKurz> for ParcoursKurz {
    fn from(kurz: logandaim_core::service::ParcoursKurz) -> Self {
        Self {
            id: kurz.id.0,
            name: kurz.name.as_str().to_owned(),
            anzahl_pflöcke: kurz.anzahl_pflöcke,
            anzahl_ziele: kurz.anzahl_ziele,
        }
    }
}

impl From<logandaim_core::service::AnlageÖffentlich> for AnlageÖffentlich {
    fn from(anlage: logandaim_core::service::AnlageÖffentlich) -> Self {
        Self {
            anlage_id: anlage.anlage_id.0,
            name: anlage.name.as_str().to_owned(),
            ort: anlage.ort,
            ländercode: anlage.ländercode.into(),
            parcours: anlage.parcours.into_iter().map(Into::into).collect(),
        }
    }
}
