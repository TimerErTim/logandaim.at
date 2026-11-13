//! Outbound schemas of master data: the Anlage with its Ziele and Parkour, the role
//! assignments, and the public Parkour list.
//!
//! The domain stays free of OpenAPI and serde; every struct here is an API mirror, built by
//! the `From` impls at the bottom so a handler never assembles one by hand.

use uuid::Uuid;

use logandaim_core::domain;

use crate::schema::EmailAdresse;

// ------------------------------------------------------------------ Anlage

/// Country codes currently accepted for Anlage addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "openapi", oai(rename = "Laendercode"))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Ländercode {
    AT,
    DE,
    CH,
}

/// A Pflock colour as validated `#rrggbb`.
///
/// Caller data, not a design token, so it cannot come from the design system's palette.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Farbe(String);

impl Farbe {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<domain::Farbe> for Farbe {
    fn from(farbe: domain::Farbe) -> Self {
        Self(farbe.to_hex())
    }
}

impl TryFrom<Farbe> for domain::Farbe {
    type Error = domain::common::FarbeFehler;

    fn try_from(farbe: Farbe) -> Result<Self, Self::Error> {
        domain::Farbe::neu(farbe.0)
    }
}

/// One structured address. Every field is required.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageAdresse {
    /// Street and house number.
    #[cfg_attr(feature = "openapi", oai(rename = "strasse"))]
    #[serde(rename = "strasse")]
    pub straße: String,
    /// Postal code of the place.
    pub postleitzahl: String,
    /// The place itself.
    pub ort: String,
    /// Two-letter country code; only `AT`, `DE`, and `CH` are accepted.
    #[cfg_attr(feature = "openapi", oai(rename = "laendercode"))]
    #[serde(rename = "laendercode")]
    pub ländercode: Ländercode,
}

/// Optional contact details of an Anlage; each may be absent on its own.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageKontakt {
    /// Name of the person to reach.
    pub name: Option<String>,
    /// E-mail address of the person to reach.
    pub email: Option<EmailAdresse>,
    /// Phone number of the person to reach.
    pub telefon: Option<String>,
    /// Website of the Anlage.
    pub website: Option<String>,
}

/// `Aktiv` or `Archiviert`.
///
/// An archived Anlage stays readable; every write to one answers `409`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AnlageZustand {
    Aktiv,
    Archiviert,
}

/// `Administrator` or `Mitarbeiter`: the Anlage's role inside one Anlage.
///
/// An Administrator may change the Anlage, its roles, and its master data; a Mitarbeiter
/// may maintain Ziele and Parkour but not the Anlage's own fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AnlageVerwaltungRolle {
    Administrator,
    Mitarbeiter,
}

/// One role assignment: which Benutzer holds which role in this Anlage.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageVerwaltung {
    /// The Benutzer that holds the role.
    pub benutzer_id: Uuid,
    /// The role it holds.
    pub rolle: AnlageVerwaltungRolle,
}

// ------------------------------------------------------------------ Ziel

/// One physical Ziel of the Anlage.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageZiel {
    /// Stable identity of this physical target.
    pub id: Uuid,
    /// Animal name shown to the shooter.
    pub tiername: String,
    /// 1-based ordinal that tells duplicate animal names apart ("Reh 1", "Reh 2").
    pub zielnummer: u32,
    /// Whether the Ziel is in use or retired.
    pub zustand: AnlageZustand,
    /// Where this physical Ziel currently lies, across this Anlage's active Parkour.
    pub verwendungen: Vec<Zielverwendung>,
}

/// One Parkour' use of one Ziel, with that Parkour' own distances.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zielverwendung {
    /// The Parkour that places this Ziel.
    pub parkour_id: Uuid,
    /// 0-based position in that Parkour' route.
    pub position: usize,
    #[cfg_attr(feature = "openapi", oai(rename = "pfloecke"))]
    #[serde(rename = "pfloecke")]
    pub pflöcke: ZielPflöcke,
    /// Pflock identity to metres, near to far.
    #[cfg_attr(feature = "openapi", oai(rename = "abstaende"))]
    #[serde(rename = "abstaende")]
    pub abstände: Vec<PflockAbstand>,
}

/// One distance from a Pflock to a Ziel.
///
/// `meter` is absent when no distance is configured for that Pflock.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PflockAbstand {
    /// The Pflock the distance is measured from.
    pub pflock_id: Uuid,
    /// Distance from that Pflock to the Ziel, in metres; absent when unconfigured.
    pub meter: Option<u32>,
}

// ------------------------------------------------------------------ Parkour

/// One shooting position. Its colour identifies it; no label exists.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParkourPflock {
    /// Identity of the Pflock inside its Parkour.
    pub id: Uuid,
    /// The Pflock's colour. It is what distinguishes one Pflock from another.
    pub farbe: Farbe,
}

/// Whether a Ziel uses the Parkour-wide Pflock order or its own.
///
/// `InheritFromParkour` selects the Parkour's order. `Explicit` provides a non-empty
/// near-to-far order that replaces it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Union))]
#[cfg_attr(
    feature = "openapi",
    oai(rename = "ZielPfloecke", discriminator_name = "art")
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "art")]
pub enum ZielPflöcke {
    #[cfg_attr(feature = "openapi", oai(mapping = "InheritFromParkour"))]
    #[serde(rename = "InheritFromParkour")]
    InheritFromParkour(InheritFromParkourPflöcke),
    #[cfg_attr(feature = "openapi", oai(mapping = "Explicit"))]
    #[serde(rename = "Explicit")]
    Explicit(ExplicitPflöcke),
}

/// The Parkour-wide order applies to this Ziel. Carries nothing; the discriminator is the
/// whole message.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "openapi",
    oai(rename = "InheritFromParkourPfloecke", rename_all = "camelCase")
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritFromParkourPflöcke {}

/// This Ziel's own near-to-far Pflock order, which replaces the Parkour-wide one. Never
/// empty.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "openapi",
    oai(rename = "ExplicitPfloecke", rename_all = "camelCase")
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplicitPflöcke {
    /// The own Pflöcke, near to far.
    #[cfg_attr(feature = "openapi", oai(rename = "pfloecke"))]
    #[serde(rename = "pfloecke")]
    pub pflöcke: Vec<ParkourPflock>,
}

impl Default for ZielPflöcke {
    fn default() -> Self {
        Self::InheritFromParkour(InheritFromParkourPflöcke {})
    }
}

/// One Ziel's place in a Parkour, with its distances keyed by Pflock identity.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zielplatzierung {
    /// The Ziel that stands here.
    pub ziel_id: Uuid,
    #[cfg_attr(feature = "openapi", oai(rename = "pfloecke"))]
    #[serde(rename = "pfloecke")]
    pub pflöcke: ZielPflöcke,
    /// Distances by Pflock identity; a missing entry means no distance is configured.
    #[cfg_attr(feature = "openapi", oai(rename = "abstaende"))]
    #[serde(rename = "abstaende")]
    pub abstände: Vec<PflockAbstand>,
}

/// One Parkour of the Anlage: its name, its Pflöcke, and the route they form.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parkour {
    /// Identity of the Parkour.
    pub id: Uuid,
    /// Shown name. Renaming keeps the identity and every captured snapshot.
    pub name: String,
    /// Whether the Parkour is in use or retired.
    pub zustand: AnlageZustand,
    /// Far to near, in the order the shooter walks them. Never inferred from a colour.
    #[cfg_attr(feature = "openapi", oai(rename = "pfloecke"))]
    #[serde(rename = "pfloecke")]
    pub pflöcke: Vec<ParkourPflock>,
    /// In route order.
    pub platzierungen: Vec<Zielplatzierung>,
}

/// One active Parkour with the Anlage that holds it, for starting a Runde.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "openapi",
    oai(rename = "ParkourOeffentlich", rename_all = "camelCase")
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParkourÖffentlich {
    /// The Anlage that offers this Parkour.
    pub anlage_id: Uuid,
    /// That Anlage's name.
    pub anlage_name: String,
    /// The Parkour with its whole setup.
    pub parkour: Parkour,
}

/// One active Parkour as the public directory names it: counts only, no setup.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParkourKurz {
    /// Identity of the Parkour.
    pub id: Uuid,
    /// Shown name of the Parkour.
    pub name: String,
    /// How many Pflöcke the route uses.
    #[cfg_attr(feature = "openapi", oai(rename = "anzahlPfloecke"))]
    #[serde(rename = "anzahlPfloecke")]
    pub anzahl_pflöcke: usize,
    /// How many Ziele the route places.
    pub anzahl_ziele: usize,
}

/// One active Anlage in the public directory.
///
/// Public, so it carries no contact data: an email address or phone number is personal data
/// and is not published.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(
    feature = "openapi",
    oai(rename = "AnlageOeffentlich", rename_all = "camelCase")
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageÖffentlich {
    /// Identity of the Anlage.
    pub anlage_id: Uuid,
    /// Shown name of the Anlage.
    pub name: String,
    /// The place the Anlage is in.
    pub ort: String,
    /// Two-letter country code of that place.
    #[cfg_attr(feature = "openapi", oai(rename = "laendercode"))]
    #[serde(rename = "laendercode")]
    pub ländercode: Ländercode,
    /// The Parkour it currently offers, without their setup.
    pub parkour: Vec<ParkourKurz>,
}

// ------------------------------------------------------------------ Anlage views

/// The Anlage with its whole master data, as `GET /anlagen/{id}` answers it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anlage {
    /// Identity of the Anlage.
    pub id: Uuid,
    /// Shown name of the Anlage.
    pub name: String,
    /// Its structured address.
    pub adresse: AnlageAdresse,
    /// Its optional contact details.
    pub kontakt: AnlageKontakt,
    /// Whether it is in use or archived.
    pub zustand: AnlageZustand,
    /// The role assignments in this Anlage.
    pub verwaltungen: Vec<AnlageVerwaltung>,
    /// Its physical Ziele.
    pub ziele: Vec<AnlageZiel>,
    /// Its Parkour, each with its setup and route.
    pub parkour: Vec<Parkour>,
}

/// One Anlage together with the caller's access to it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnlageZugriff {
    /// The Anlage itself, with its whole master data.
    pub anlage: Anlage,
    /// The caller's role in this Anlage, absent when it holds none.
    pub rolle: Option<AnlageVerwaltungRolle>,
    /// Whether the caller is a Systemadministrator and may act regardless of `rolle`.
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

impl From<domain::Ländercode> for Ländercode {
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

impl From<domain::ParkourPflock> for ParkourPflock {
    fn from(pflock: domain::ParkourPflock) -> Self {
        Self {
            id: pflock.id().0,
            farbe: pflock.farbe().into(),
        }
    }
}

impl From<&domain::ZielPflöcke> for ZielPflöcke {
    fn from(pflöcke: &domain::ZielPflöcke) -> Self {
        match pflöcke {
            domain::ZielPflöcke::InheritFromParkour => {
                Self::InheritFromParkour(InheritFromParkourPflöcke {})
            }
            domain::ZielPflöcke::Explicit(eigene) => Self::Explicit(ExplicitPflöcke {
                pflöcke: eigene.iter().cloned().map(Into::into).collect(),
            }),
        }
    }
}

impl From<domain::Zielplatzierung> for Zielplatzierung {
    fn from(platzierung: domain::Zielplatzierung) -> Self {
        Self {
            ziel_id: platzierung.ziel_id().0,
            pflöcke: platzierung.pflöcke().into(),
            abstände: platzierung
                .abstände()
                .iter()
                .map(|(pflock_id, abstand)| PflockAbstand {
                    pflock_id: pflock_id.0,
                    meter: Some(abstand.meter()),
                })
                .collect(),
        }
    }
}

impl From<domain::Parkour> for Parkour {
    fn from(parkour: domain::Parkour) -> Self {
        Self {
            id: parkour.id().0,
            name: parkour.name().as_str().to_owned(),
            zustand: zustand(parkour.zustand()),
            pflöcke: parkour.pflöcke().iter().cloned().map(Into::into).collect(),
            platzierungen: parkour
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
            parkour_id: verwendung.parkour_id.0,
            position: verwendung.position,
            pflöcke: (&verwendung.pflöcke).into(),
            abstände: verwendung
                .abstände
                .into_iter()
                .map(|(pflock_id, abstand)| PflockAbstand {
                    pflock_id: pflock_id.0,
                    meter: Some(abstand.meter()),
                })
                .collect(),
        }
    }
}

impl From<domain::AnlageZiel> for AnlageZiel {
    fn from(ziel: domain::AnlageZiel) -> Self {
        Self {
            id: ziel.id().0,
            tiername: ziel.tiername().as_str().to_owned(),
            zielnummer: ziel.zielnummer().get(),
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
            ziele: anlage
                .ziele()
                .iter()
                .map(|ziel| {
                    // The admin page renders where each Ziel stands, so the read attaches it.
                    let mut ziel: AnlageZiel = ziel.clone().into();
                    ziel.verwendungen = anlage
                        .ziel_verwendungen(domain::AnlageZielId(ziel.id))
                        .into_iter()
                        .map(Into::into)
                        .collect();
                    ziel
                })
                .collect(),
            parkour: anlage.parkour().iter().cloned().map(Into::into).collect(),
        }
    }
}

impl From<domain::Parkour> for ParkourKurz {
    fn from(parkour: domain::Parkour) -> Self {
        Self {
            id: parkour.id().0,
            name: parkour.name().as_str().to_owned(),
            anzahl_pflöcke: parkour.pflöcke().len(),
            anzahl_ziele: parkour.platzierungen().len(),
        }
    }
}
