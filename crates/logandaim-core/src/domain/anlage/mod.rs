//! The `Anlage` aggregate: a physical site with its Parkours, Ziele, and authority.
//!
//! `Anlage` is the one aggregate root of this slice that owns entities: `Ziel` and
//! `Parkour` have no lifecycle outside it, so they are children, not roots. Authority
//! (`AnlageVerwaltung`) lives here too, which is why loading one aggregate answers both
//! "may you?" and "what is the state?" (SPEC-stammdaten rules 2, 4, 5, 10).
//!
//! The root is a **typestate** over its lifecycle: only `AnlageIn<Aktiv>` has mutators, and
//! [`AnlageIn::deaktiviere`] is the only transition. There is no `reaktiviere`, so
//! "archived data cannot be restored" (AC-11) is a compile error rather than a runtime
//! check. The children use a plain [`Zustand`] enum instead: they are never loaded or saved
//! on their own, so a phantom parameter would buy safety where no caller could act.

use std::num::NonZeroU32;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod adresse;
pub mod kontaktdaten;
pub mod parkour;
pub mod snapshot;
pub mod ziel;

pub use adresse::{AnlageAdresse, AnlageAdresseFehler};
pub use kontaktdaten::{AnlageKontakt, WebsiteUrl, WebsiteUrlFehler};
pub use parkour::{
    NeuerParkour, Parkour, ParkourFehler, ParkourId, Pflock, PflockId, Zielplatzierung,
};
pub use snapshot::{ParkourSnapshot, PflockSnapshot, SnapshotFehler, ZielSnapshot};
pub use ziel::{NeuesZiel, Ziel, ZielFehler, ZielId, Zustand, ZustandUnbekannt};

use crate::domain::common::{Abstand, NonEmptyString};
use crate::domain::konto::{Benutzer, BenutzerId};

/// Identity of one Anlage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnlageId(pub Uuid);

/// Non-empty name of one Anlage. Names need not be unique (AC-1).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnlageName(NonEmptyString);

#[derive(Debug, thiserror::Error)]
#[error("der Anlagename darf nicht leer sein")]
pub struct AnlageNameLeer;

impl AnlageName {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, AnlageNameLeer> {
        NonEmptyString::neu(wert)
            .map(Self)
            .map_err(|_| AnlageNameLeer)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for AnlageName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

/// A Benutzer's role inside one Anlage (SPEC-stammdaten "Roles").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnlageVerwaltungRolle {
    Administrator,
    Mitarbeiter,
}

impl AnlageVerwaltungRolle {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Administrator => "Administrator",
            Self::Mitarbeiter => "Mitarbeiter",
        }
    }

    /// Everything but the Anlage's descriptive fields and its Verwaltungen.
    pub fn darf_masterdaten_pflegen(self) -> bool {
        matches!(self, Self::Administrator | Self::Mitarbeiter)
    }

    /// Anlage fields, deactivation, and `AnlageVerwaltung` management (rule 2).
    pub fn darf_anlage_verwalten(self) -> bool {
        matches!(self, Self::Administrator)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("die Anlagenrolle ist unbekannt")]
pub struct AnlageVerwaltungRolleUnbekannt;

impl std::str::FromStr for AnlageVerwaltungRolle {
    type Err = AnlageVerwaltungRolleUnbekannt;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Administrator" => Ok(Self::Administrator),
            "Mitarbeiter" => Ok(Self::Mitarbeiter),
            _ => Err(AnlageVerwaltungRolleUnbekannt),
        }
    }
}

/// One Benutzer's role inside one Anlage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnlageVerwaltung {
    benutzer_id: BenutzerId,
    rolle: AnlageVerwaltungRolle,
}

impl AnlageVerwaltung {
    pub fn neu(benutzer_id: BenutzerId, rolle: AnlageVerwaltungRolle) -> Self {
        Self { benutzer_id, rolle }
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn rolle(&self) -> AnlageVerwaltungRolle {
        self.rolle
    }
}

/// Lifecycle markers of the `Anlage` typestate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Aktiv;

/// The archived state carries when it happened, so a reader can date the stamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Archiviert {
    deaktiviert_am: DateTime<Utc>,
}

impl Archiviert {
    pub fn deaktiviert_am(self) -> DateTime<Utc> {
        self.deaktiviert_am
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AnlageFehler {
    #[error("der Benutzer verwaltet diese Anlage bereits")]
    VerwaltungExistiert,
    #[error("die Verwaltung dieses Benutzers existiert nicht")]
    VerwaltungUnbekannt,
    #[error("die letzte Administrator-Verwaltung kann nicht entzogen werden")]
    LetzterAdministrator,
    #[error("das Ziel wurde nicht gefunden")]
    ZielNichtGefunden,
    #[error("der Parkour wurde nicht gefunden")]
    ParkourNichtGefunden,
    #[error("das Ziel gehört nicht zu dieser Anlage")]
    ZielUnbekannt,
    #[error("das Ziel ist archiviert und kann nicht mehr geändert werden")]
    ZielArchiviert,
    #[error("der Parkour gehört nicht zu dieser Anlage")]
    ParkourUnbekannt,
    #[error("das Ziel ist bereits in diesem Parkour platziert")]
    ZielBereitsPlatziert,
    #[error("der Parkour verwendet ein Ziel, das nicht zu dieser Anlage gehört")]
    ZielFremdeAnlage,
}

/// Creation input of an Anlage: everything except its identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeueAnlage {
    name: AnlageName,
    adresse: AnlageAdresse,
    kontakt: AnlageKontakt,
}

impl NeueAnlage {
    pub fn neu(name: AnlageName, adresse: AnlageAdresse, kontakt: AnlageKontakt) -> Self {
        Self {
            name,
            adresse,
            kontakt,
        }
    }

    pub fn name(&self) -> &AnlageName {
        &self.name
    }

    pub fn adresse(&self) -> &AnlageAdresse {
        &self.adresse
    }

    pub fn kontakt(&self) -> &AnlageKontakt {
        &self.kontakt
    }
}

/// One physical site of a club or commercial operator, in one lifecycle state.
#[derive(Debug, Clone)]
pub struct AnlageIn<S> {
    id: AnlageId,
    name: AnlageName,
    adresse: AnlageAdresse,
    kontakt: AnlageKontakt,
    verwaltungen: Vec<AnlageVerwaltung>,
    ziele: Vec<Ziel>,
    parkour: Vec<Parkour>,
    state: S,
}

pub type AnlageAktiv = AnlageIn<Aktiv>;
pub type AnlageArchiviert = AnlageIn<Archiviert>;
/// The sum of both states, for code that does not know the state at compile time.
#[derive(Debug, Clone)]
pub enum Anlage {
    Aktiv(AnlageAktiv),
    Archiviert(AnlageArchiviert),
}

impl<S> AnlageIn<S> {
    pub fn id(&self) -> AnlageId {
        self.id
    }

    pub fn name(&self) -> &AnlageName {
        &self.name
    }

    pub fn adresse(&self) -> &AnlageAdresse {
        &self.adresse
    }

    pub fn kontakt(&self) -> &AnlageKontakt {
        &self.kontakt
    }

    pub fn verwaltungen(&self) -> &[AnlageVerwaltung] {
        &self.verwaltungen
    }

    pub fn ziele(&self) -> &[Ziel] {
        &self.ziele
    }

    pub fn parkour(&self) -> &[Parkour] {
        &self.parkour
    }

    /// The caller's role inside this Anlage, or `None` when they hold no Verwaltung.
    pub fn rolle_von(&self, benutzer_id: BenutzerId) -> Option<AnlageVerwaltungRolle> {
        self.verwaltungen
            .iter()
            .find(|verwaltung| verwaltung.benutzer_id == benutzer_id)
            .map(|verwaltung| verwaltung.rolle)
    }

    pub fn ist_verwaltet_von(&self, benutzer_id: BenutzerId) -> bool {
        self.rolle_von(benutzer_id).is_some()
    }

    /// Whether this Anlage still has at least one Administrator.
    pub fn hat_administrator(&self) -> bool {
        self.verwaltungen
            .iter()
            .any(|verwaltung| verwaltung.rolle == AnlageVerwaltungRolle::Administrator)
    }

    pub fn ziel(&self, ziel_id: ZielId) -> Option<&Ziel> {
        self.ziele.iter().find(|ziel| ziel.id() == ziel_id)
    }

    pub fn parkour_mit_id(&self, parkour_id: ParkourId) -> Option<&Parkour> {
        self.parkour
            .iter()
            .find(|parkour| parkour.id() == parkour_id)
    }

    /// An active Parkour by identity, or `None` when it is missing or archived.
    pub fn aktiver_parkour(&self, parkour_id: ParkourId) -> Option<&Parkour> {
        self.parkour_mit_id(parkour_id)
            .filter(|parkour| !parkour.ist_archiviert())
    }

    /// The Parkour a Runde may be started on: active, and belonging to an active Anlage.
    pub fn startbarer_parkour(&self, parkour_id: ParkourId) -> Option<&Parkour> {
        self.aktiver_parkour(parkour_id)
    }

    /// Capture the immutable setup of one Parkour as a value copy (AC-7, AC-9).
    ///
    /// A thin wrapper over the capture rule in [`snapshot`], which is where it lives.
    pub fn snapshot_für(&self, parkour_id: ParkourId) -> Result<ParkourSnapshot, SnapshotFehler> {
        let parkour = self
            .parkour_mit_id(parkour_id)
            .ok_or(SnapshotFehler::ParkourNichtGefunden)?;
        snapshot::snapshot_aus_parkour(parkour, |ziel_id| self.ziel(ziel_id))
    }

    /// Where one physical Ziel is used across this Anlage's Parkours: in which routes, at
    /// which order position, and at which distances.
    ///
    /// The Parkour's name and list position are not repeated here: a caller holding the
    /// `parkour_id` reads them from this same Anlage.
    pub fn ziel_verwendungen(&self, ziel_id: ZielId) -> Vec<Zielverwendung> {
        self.parkour
            .iter()
            .filter(|parkour| !parkour.ist_archiviert())
            .filter_map(|parkour| {
                let position = parkour
                    .platzierungen()
                    .iter()
                    .position(|platzierung| platzierung.ziel_id() == ziel_id)?;
                let platzierung = &parkour.platzierungen()[position];
                Some(Zielverwendung {
                    parkour_id: parkour.id(),
                    position,
                    abstände: platzierung.abstände().to_vec(),
                })
            })
            .collect()
    }
}

impl AnlageAktiv {
    /// The only way to build an active Anlage. It may start without an Administrator
    /// (AC-1, rule 2).
    pub fn neu(id: AnlageId, neu: NeueAnlage) -> Self {
        Self {
            id,
            name: neu.name,
            adresse: neu.adresse,
            kontakt: neu.kontakt,
            verwaltungen: Vec::new(),
            ziele: Vec::new(),
            parkour: Vec::new(),
            state: Aktiv,
        }
    }

    /// Rebuild a persisted active Anlage.
    pub fn from_parts(
        id: AnlageId,
        name: AnlageName,
        adresse: AnlageAdresse,
        kontakt: AnlageKontakt,
        verwaltungen: Vec<AnlageVerwaltung>,
        ziele: Vec<Ziel>,
        parkour: Vec<Parkour>,
    ) -> Self {
        Self {
            id,
            name,
            adresse,
            kontakt,
            verwaltungen,
            ziele,
            parkour,
            state: Aktiv,
        }
    }

    /// Edit the descriptive fields (rule 2, FR-1).
    pub fn beschreibende_daten_ändern(
        &mut self,
        name: AnlageName,
        adresse: AnlageAdresse,
        kontakt: AnlageKontakt,
    ) {
        self.name = name;
        self.adresse = adresse;
        self.kontakt = kontakt;
    }

    /// Grant a role. A Benutzer holds at most one Verwaltung per Anlage.
    pub fn verwaltung_gewähren(
        &mut self,
        benutzer_id: BenutzerId,
        rolle: AnlageVerwaltungRolle,
    ) -> Result<(), AnlageFehler> {
        if self.ist_verwaltet_von(benutzer_id) {
            return Err(AnlageFehler::VerwaltungExistiert);
        }
        self.verwaltungen
            .push(AnlageVerwaltung::neu(benutzer_id, rolle));
        Ok(())
    }

    /// Change an existing Verwaltung's role.
    pub fn verwaltung_ändern(
        &mut self,
        benutzer_id: BenutzerId,
        rolle: AnlageVerwaltungRolle,
    ) -> Result<(), AnlageFehler> {
        // Downgrading the last Administrator would leave the Anlage unmanaged.
        let administratoren = self.administratoren();
        let Some(verwaltung) = self
            .verwaltungen
            .iter_mut()
            .find(|verwaltung| verwaltung.benutzer_id == benutzer_id)
        else {
            return Err(AnlageFehler::VerwaltungUnbekannt);
        };
        if verwaltung.rolle == AnlageVerwaltungRolle::Administrator
            && rolle != AnlageVerwaltungRolle::Administrator
            && administratoren == 1
        {
            return Err(AnlageFehler::LetzterAdministrator);
        }
        verwaltung.rolle = rolle;
        Ok(())
    }

    /// Revoke a Verwaltung. The last Administrator cannot be revoked (AC-2, rule 2).
    pub fn verwaltung_entziehen(&mut self, benutzer_id: BenutzerId) -> Result<(), AnlageFehler> {
        let Some(verwaltung) = self
            .verwaltungen
            .iter()
            .find(|verwaltung| verwaltung.benutzer_id == benutzer_id)
        else {
            return Err(AnlageFehler::VerwaltungUnbekannt);
        };
        if verwaltung.rolle == AnlageVerwaltungRolle::Administrator && self.administratoren() == 1 {
            return Err(AnlageFehler::LetzterAdministrator);
        }
        self.verwaltungen
            .retain(|verwaltung| verwaltung.benutzer_id != benutzer_id);
        Ok(())
    }

    fn administratoren(&self) -> usize {
        self.verwaltungen
            .iter()
            .filter(|verwaltung| verwaltung.rolle == AnlageVerwaltungRolle::Administrator)
            .count()
    }

    /// Add a physical Ziel at this Anlage. Its ordinal is derived from the Tiername, so two
    /// Ziele named "Reh" become "Reh 1" and "Reh 2" without the caller choosing.
    pub fn ziel_hinzufügen(&mut self, id: ZielId, tiername: NonEmptyString) -> ZielId {
        let zielnummer = self.naechste_zielnummer(&tiername);
        self.ziele
            .push(Ziel::neu(id, NeuesZiel::neu(tiername, zielnummer)));
        id
    }

    /// The next free ordinal for this Tiername: 1-based and never reused while any Ziel of
    /// that name exists, archived ones included.
    fn naechste_zielnummer(&self, tiername: &NonEmptyString) -> NonZeroU32 {
        let vergeben = self
            .ziele
            .iter()
            .filter(|ziel| ziel.tiername() == tiername)
            .count();
        NonZeroU32::new(
            u32::try_from(vergeben)
                .unwrap_or(u32::MAX)
                .saturating_add(1),
        )
        .unwrap_or(NonZeroU32::MIN)
    }

    /// Edit a Ziel's animal name. Its identity and its ordinal do not change (AC-5).
    pub fn ziel_ändern(
        &mut self,
        ziel_id: ZielId,
        tiername: NonEmptyString,
    ) -> Result<(), AnlageFehler> {
        self.ziele
            .iter_mut()
            .find(|ziel| ziel.id() == ziel_id)
            .ok_or(AnlageFehler::ZielNichtGefunden)?
            .ändern(tiername)
            .map_err(|_| AnlageFehler::ZielArchiviert)
    }

    /// Archive a Ziel and drop it from every current arrangement, keeping its identity
    /// and history (AC-11).
    pub fn ziel_archivieren(&mut self, ziel_id: ZielId) -> Result<(), AnlageFehler> {
        self.ziele
            .iter_mut()
            .find(|ziel| ziel.id() == ziel_id)
            .ok_or(AnlageFehler::ZielNichtGefunden)?
            .archiviere()
            .map_err(|_| AnlageFehler::ZielArchiviert)?;
        for parkour in &mut self.parkour {
            if parkour.ist_archiviert() {
                continue;
            }
            let _ = parkour.platzierung_entfernen(ziel_id);
        }
        Ok(())
    }

    /// Delete a Ziel outright, for master data with no history (AC-12).
    pub fn ziel_entfernen(&mut self, ziel_id: ZielId) -> Result<(), AnlageFehler> {
        if self.ziel(ziel_id).is_none() {
            return Err(AnlageFehler::ZielUnbekannt);
        }
        self.ziele.retain(|ziel| ziel.id() != ziel_id);
        for parkour in &mut self.parkour {
            if parkour.ist_archiviert() {
                continue;
            }
            let _ = parkour.platzierung_entfernen(ziel_id);
        }
        Ok(())
    }

    /// Replace the physical target behind one Ziel identity (AC-5).
    ///
    /// The old identity is archived — it keeps its history — and a new identity takes over
    /// the same route positions with the same distances. The new Ziel therefore inherits the
    /// *slot*, never the old target's Schüsse.
    pub fn ziel_ersetzen(
        &mut self,
        alt: ZielId,
        neu: ZielId,
        tiername: NonEmptyString,
    ) -> Result<ZielId, AnlageFehler> {
        let altes = self.ziel(alt).ok_or(AnlageFehler::ZielUnbekannt)?;
        if altes.ist_archiviert() {
            return Err(AnlageFehler::ZielArchiviert);
        }
        // Hand the slot over *before* archiving: archiving strips the old placement from
        // every active route, so doing it first would lose the position the new Ziel takes.
        for parkour in &mut self.parkour {
            if parkour.ist_archiviert() {
                continue;
            }
            parkour
                .platzierung_ersetzen(alt, neu)
                .map_err(|_| AnlageFehler::ZielBereitsPlatziert)?;
        }
        self.ziel_archivieren(alt)?;
        self.ziel_hinzufügen(neu, tiername);
        Ok(neu)
    }

    /// Rename a Parkour. Refused while archived.
    pub fn parkour_umbenennen(
        &mut self,
        parkour_id: ParkourId,
        name: NonEmptyString,
    ) -> Result<(), ParkourFehler> {
        self.parkour
            .iter_mut()
            .find(|parkour| parkour.id() == parkour_id)
            .ok_or(ParkourFehler::ParkourNichtGefunden)?
            .umbenennen(name)
    }

    /// Add a Parkour. Every placed Ziel must belong to this Anlage (AC-4).
    pub fn parkour_hinzufügen(
        &mut self,
        id: ParkourId,
        neu: NeuerParkour,
    ) -> Result<ParkourId, AnlageFehler> {
        for platzierung in neu.platzierungen() {
            if self.ziel(platzierung.ziel_id()).is_none() {
                return Err(AnlageFehler::ZielFremdeAnlage);
            }
        }
        self.parkour.push(Parkour::neu(id, neu));
        Ok(id)
    }

    /// Replace a Parkour's whole setup. Every placed Ziel must still belong here.
    pub fn parkour_setup_ändern(
        &mut self,
        parkour_id: ParkourId,
        pflöcke: Vec<Pflock>,
        platzierungen: Vec<Zielplatzierung>,
    ) -> Result<(), ParkourFehler> {
        for platzierung in &platzierungen {
            if self.ziel(platzierung.ziel_id()).is_none() {
                return Err(ParkourFehler::ZielFremdeAnlage);
            }
        }
        self.parkour
            .iter_mut()
            .find(|parkour| parkour.id() == parkour_id)
            .ok_or(ParkourFehler::ParkourNichtGefunden)?
            .setup_ändern(pflöcke, platzierungen)
    }

    /// Archive a Parkour, keeping its identity and its history references (AC-11).
    pub fn parkour_archivieren(&mut self, parkour_id: ParkourId) -> Result<(), ParkourFehler> {
        self.parkour
            .iter_mut()
            .find(|parkour| parkour.id() == parkour_id)
            .ok_or(ParkourFehler::ParkourNichtGefunden)?
            .archiviere()
    }

    /// Delete a Parkour outright, for one with no history (AC-12).
    pub fn parkour_entfernen(&mut self, parkour_id: ParkourId) -> Result<(), AnlageFehler> {
        if self.parkour_mit_id(parkour_id).is_none() {
            return Err(AnlageFehler::ParkourUnbekannt);
        }
        self.parkour.retain(|parkour| parkour.id() != parkour_id);
        Ok(())
    }

    /// Deactivate the Anlage. The only lifecycle transition; there is no way back.
    pub fn deaktiviere(self, am: DateTime<Utc>) -> AnlageArchiviert {
        AnlageIn {
            id: self.id,
            name: self.name,
            adresse: self.adresse,
            kontakt: self.kontakt,
            verwaltungen: self.verwaltungen,
            ziele: self.ziele,
            parkour: self.parkour,
            state: Archiviert { deaktiviert_am: am },
        }
    }
}

impl AnlageArchiviert {
    /// Rebuild a persisted archived Anlage.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: AnlageId,
        name: AnlageName,
        adresse: AnlageAdresse,
        kontakt: AnlageKontakt,
        verwaltungen: Vec<AnlageVerwaltung>,
        ziele: Vec<Ziel>,
        parkour: Vec<Parkour>,
        deaktiviert_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            adresse,
            kontakt,
            verwaltungen,
            ziele,
            parkour,
            state: Archiviert { deaktiviert_am },
        }
    }

    pub fn deaktiviert_am(&self) -> DateTime<Utc> {
        self.state.deaktiviert_am
    }
}

impl From<AnlageAktiv> for Anlage {
    fn from(anlage: AnlageAktiv) -> Self {
        Self::Aktiv(anlage)
    }
}

impl From<AnlageArchiviert> for Anlage {
    fn from(anlage: AnlageArchiviert) -> Self {
        Self::Archiviert(anlage)
    }
}

impl Anlage {
    pub fn aktiv(self) -> Option<AnlageAktiv> {
        match self {
            Self::Aktiv(anlage) => Some(anlage),
            Self::Archiviert(_) => None,
        }
    }

    pub fn archiviert(self) -> Option<AnlageArchiviert> {
        match self {
            Self::Aktiv(_) => None,
            Self::Archiviert(anlage) => Some(anlage),
        }
    }

    /// Whether the Anlage has been deactivated (AC-10).
    pub fn ist_archiviert(&self) -> bool {
        matches!(self, Self::Archiviert(_))
    }

    /// The identity, whichever state the aggregate is in.
    pub fn id(&self) -> AnlageId {
        match self {
            Self::Aktiv(anlage) => anlage.id(),
            Self::Archiviert(anlage) => anlage.id(),
        }
    }

    pub fn name(&self) -> &AnlageName {
        match self {
            Self::Aktiv(anlage) => anlage.name(),
            Self::Archiviert(anlage) => anlage.name(),
        }
    }

    pub fn adresse(&self) -> &AnlageAdresse {
        match self {
            Self::Aktiv(anlage) => anlage.adresse(),
            Self::Archiviert(anlage) => anlage.adresse(),
        }
    }

    pub fn kontakt(&self) -> &AnlageKontakt {
        match self {
            Self::Aktiv(anlage) => anlage.kontakt(),
            Self::Archiviert(anlage) => anlage.kontakt(),
        }
    }

    pub fn verwaltungen(&self) -> &[AnlageVerwaltung] {
        match self {
            Self::Aktiv(anlage) => anlage.verwaltungen(),
            Self::Archiviert(anlage) => anlage.verwaltungen(),
        }
    }

    pub fn ziele(&self) -> &[Ziel] {
        match self {
            Self::Aktiv(anlage) => anlage.ziele(),
            Self::Archiviert(anlage) => anlage.ziele(),
        }
    }

    pub fn parkour(&self) -> &[Parkour] {
        match self {
            Self::Aktiv(anlage) => anlage.parkour(),
            Self::Archiviert(anlage) => anlage.parkour(),
        }
    }

    pub fn rolle_von(&self, benutzer_id: BenutzerId) -> Option<AnlageVerwaltungRolle> {
        match self {
            Self::Aktiv(anlage) => anlage.rolle_von(benutzer_id),
            Self::Archiviert(anlage) => anlage.rolle_von(benutzer_id),
        }
    }

    pub fn ziel(&self, ziel_id: ZielId) -> Option<&Ziel> {
        match self {
            Self::Aktiv(anlage) => anlage.ziel(ziel_id),
            Self::Archiviert(anlage) => anlage.ziel(ziel_id),
        }
    }

    pub fn parkour_mit_id(&self, parkour_id: ParkourId) -> Option<&Parkour> {
        match self {
            Self::Aktiv(anlage) => anlage.parkour_mit_id(parkour_id),
            Self::Archiviert(anlage) => anlage.parkour_mit_id(parkour_id),
        }
    }

    /// Placement aggregation, delegated so a reader holding the sum never matches on state.
    pub fn ziel_verwendungen(&self, ziel_id: ZielId) -> Vec<Zielverwendung> {
        match self {
            Self::Aktiv(anlage) => anlage.ziel_verwendungen(ziel_id),
            Self::Archiviert(anlage) => anlage.ziel_verwendungen(ziel_id),
        }
    }

    /// Capture a Parkour snapshot, delegated to whichever state holds the children.
    pub fn snapshot_für(&self, parkour_id: ParkourId) -> Result<ParkourSnapshot, SnapshotFehler> {
        snapshot::snapshot(self, parkour_id)
    }
}

/// One Parkour's use of one physical Ziel, as the Anlage sees it.
///
/// Deliberately holds no copy of the Parkour's name or list position: a caller with the
/// `parkour_id` reads those from the same `Anlage`, so they cannot go stale here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zielverwendung {
    /// The Parkour that places this Ziel.
    pub parkour_id: ParkourId,
    /// 0-based position of the Ziel in that Parkour's route.
    pub position: usize,
    /// Distance per Pflock, for the distances that are configured.
    pub abstände: Vec<(PflockId, Abstand)>,
}

/// Whether a Benutzer may act on an Anlage at all.
///
/// The use case combines this with the aggregate's own role, and the answer never comes
/// from a permission table (NFR-2). The identity comes from the `Benutzer` itself.
pub fn ist_systemadministrator_oder_verwalter(
    benutzer: &Benutzer,
    anlage: &impl AnlagenSicht,
) -> bool {
    benutzer.ist_systemadministrator() || anlage.rolle_von(benutzer.id()).is_some()
}

/// The read-only view a role helper needs from either lifecycle state.
pub trait AnlagenSicht {
    fn rolle_von(&self, benutzer_id: BenutzerId) -> Option<AnlageVerwaltungRolle>;
}

impl<S> AnlagenSicht for AnlageIn<S> {
    fn rolle_von(&self, benutzer_id: BenutzerId) -> Option<AnlageVerwaltungRolle> {
        AnlageIn::rolle_von(self, benutzer_id)
    }
}

impl AnlagenSicht for Anlage {
    fn rolle_von(&self, benutzer_id: BenutzerId) -> Option<AnlageVerwaltungRolle> {
        Anlage::rolle_von(self, benutzer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::common::Farbe;

    fn adresse() -> AnlageAdresse {
        AnlageAdresse::neu(
            "Waldweg 1",
            "3100",
            "St. Pölten",
            crate::domain::Ländercode::AT,
        )
        .unwrap()
    }

    fn anlage() -> AnlageAktiv {
        AnlageAktiv::neu(
            AnlageId(Uuid::nil()),
            NeueAnlage::neu(
                AnlageName::neu("Breitenstein").unwrap(),
                adresse(),
                AnlageKontakt::leer(),
            ),
        )
    }

    fn benutzer() -> BenutzerId {
        BenutzerId(Uuid::from_u128(1))
    }

    fn zweiter() -> BenutzerId {
        BenutzerId(Uuid::from_u128(2))
    }

    fn ziel_id() -> ZielId {
        ZielId(Uuid::from_u128(10))
    }

    fn name(wert: &str) -> NonEmptyString {
        NonEmptyString::neu(wert).unwrap()
    }

    fn pflock(id: u128, farbe: &str) -> Pflock {
        Pflock::neu(PflockId(Uuid::from_u128(id)), Farbe::neu(farbe).unwrap())
    }

    fn parkour_neu(ziel: ZielId) -> NeuerParkour {
        NeuerParkour::neu(
            name("Waldrunde"),
            vec![pflock(1, "#c0392b"), pflock(2, "#1b4f9c")],
            vec![Zielplatzierung::neu(ziel, Vec::new()).unwrap()],
        )
        .unwrap()
    }

    #[test_log::test]
    fn ac1_neue_anlage_ist_aktiv_und_ohne_administrator() {
        let anlage = anlage();
        assert!(anlage.verwaltungen().is_empty());
        assert!(!anlage.hat_administrator());
    }

    #[test_log::test]
    fn ac2_die_letzte_administrator_verwaltung_kann_nicht_entzogen_werden() {
        let mut anlage = anlage();
        anlage
            .verwaltung_gewähren(benutzer(), AnlageVerwaltungRolle::Administrator)
            .unwrap();
        assert!(matches!(
            anlage.verwaltung_entziehen(benutzer()),
            Err(AnlageFehler::LetzterAdministrator)
        ));
        assert!(anlage.hat_administrator());

        anlage
            .verwaltung_gewähren(zweiter(), AnlageVerwaltungRolle::Administrator)
            .unwrap();
        anlage.verwaltung_entziehen(benutzer()).unwrap();
        assert!(anlage.hat_administrator());
    }

    #[test_log::test]
    fn ac2_eine_verwaltung_existiert_nur_einmal_pro_benutzer() {
        let mut anlage = anlage();
        anlage
            .verwaltung_gewähren(benutzer(), AnlageVerwaltungRolle::Mitarbeiter)
            .unwrap();
        assert!(matches!(
            anlage.verwaltung_gewähren(benutzer(), AnlageVerwaltungRolle::Administrator),
            Err(AnlageFehler::VerwaltungExistiert)
        ));
        anlage
            .verwaltung_ändern(benutzer(), AnlageVerwaltungRolle::Administrator)
            .unwrap();
        assert_eq!(
            anlage.rolle_von(benutzer()),
            Some(AnlageVerwaltungRolle::Administrator)
        );
    }

    #[test_log::test]
    fn eine_mitarbeiter_verwaltung_kann_nicht_zum_null_administrator_fuehren() {
        let mut anlage = anlage();
        anlage
            .verwaltung_gewähren(benutzer(), AnlageVerwaltungRolle::Administrator)
            .unwrap();
        assert!(matches!(
            anlage.verwaltung_ändern(benutzer(), AnlageVerwaltungRolle::Mitarbeiter),
            Err(AnlageFehler::LetzterAdministrator)
        ));
    }

    #[test_log::test]
    fn ein_verwalter_wird_aus_dem_benutzer_erkannt() {
        let mut anlage = anlage();
        anlage
            .verwaltung_gewähren(benutzer(), AnlageVerwaltungRolle::Mitarbeiter)
            .unwrap();
        let verwalter = Benutzer::neu(
            benutzer(),
            crate::domain::konto::NeuerBenutzer::registrierung(
                DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
            ),
        );
        let fremder = Benutzer::neu(
            zweiter(),
            crate::domain::konto::NeuerBenutzer::registrierung(
                DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
            ),
        );
        assert!(ist_systemadministrator_oder_verwalter(&verwalter, &anlage));
        assert!(!ist_systemadministrator_oder_verwalter(&fremder, &anlage));
    }

    #[test_log::test]
    fn ac4_ein_parkour_verwendet_nur_ziele_dieser_anlage() {
        let mut anlage = anlage();
        assert!(matches!(
            anlage.parkour_hinzufügen(ParkourId(Uuid::nil()), parkour_neu(ziel_id())),
            Err(AnlageFehler::ZielFremdeAnlage)
        ));

        anlage.ziel_hinzufügen(ziel_id(), name("Keiler"));
        anlage
            .parkour_hinzufügen(ParkourId(Uuid::nil()), parkour_neu(ziel_id()))
            .unwrap();
        assert_eq!(anlage.parkour().len(), 1);
    }

    #[test_log::test]
    fn ac4_ein_ziel_kann_in_mehreren_parkour_liegen() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(ziel_id(), name("Keiler"));
        anlage
            .parkour_hinzufügen(ParkourId(Uuid::nil()), parkour_neu(ziel_id()))
            .unwrap();
        anlage
            .parkour_hinzufügen(ParkourId(Uuid::from_u128(2)), parkour_neu(ziel_id()))
            .unwrap();
        let verwendungen = anlage.ziel_verwendungen(ziel_id());
        assert_eq!(verwendungen.len(), 2);
        // The Parkour's name is read from the Anlage, never copied into the usage row.
        let name = anlage
            .parkour_mit_id(verwendungen[0].parkour_id)
            .unwrap()
            .name();
        assert_eq!(name.as_str(), "Waldrunde");
    }

    #[test_log::test]
    fn ac7_der_snapshot_ist_eine_wertkopie() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(ziel_id(), name("Keiler"));
        let parkour_id = ParkourId(Uuid::nil());
        anlage
            .parkour_hinzufügen(parkour_id, parkour_neu(ziel_id()))
            .unwrap();

        let snapshot = anlage.snapshot_für(parkour_id).unwrap();
        assert_eq!(snapshot.parkour_name().as_str(), "Waldrunde");
        assert_eq!(snapshot.ziele()[0].tiername().as_str(), "Keiler");

        // A later edit changes the aggregate, never the captured copy.
        anlage.ziel_ändern(ziel_id(), name("Reh")).unwrap();
        assert_eq!(snapshot.ziele()[0].tiername().as_str(), "Keiler");
        assert_eq!(snapshot.ziele()[0].zielnummer().get(), 1);

        let neuer_snapshot = anlage.snapshot_für(parkour_id).unwrap();
        assert_eq!(neuer_snapshot.ziele()[0].tiername().as_str(), "Reh");
    }

    #[test_log::test]
    fn zielnummer_trennt_gleichnamige_ziele() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(ZielId(Uuid::from_u128(1)), name("Reh"));
        anlage.ziel_hinzufügen(ZielId(Uuid::from_u128(2)), name("Reh"));
        anlage.ziel_hinzufügen(ZielId(Uuid::from_u128(3)), name("Reh"));
        anlage.ziel_hinzufügen(ZielId(Uuid::from_u128(4)), name("Keiler"));

        let nummern: Vec<(u128, u32)> = anlage
            .ziele()
            .iter()
            .map(|ziel| (ziel.id().0.as_u128(), ziel.zielnummer().get()))
            .collect();
        assert_eq!(
            nummern,
            vec![(1, 1), (2, 2), (3, 3), (4, 1)],
            "Reh 1, Reh 2, Reh 3, and Keiler counted on its own name"
        );

        // Archiving does not free an ordinal: names stay unambiguous across history.
        anlage.ziel_archivieren(ZielId(Uuid::from_u128(2))).unwrap();
        anlage.ziel_hinzufügen(ZielId(Uuid::from_u128(5)), name("Reh"));
        assert_eq!(
            anlage
                .ziel(ZielId(Uuid::from_u128(5)))
                .unwrap()
                .zielnummer()
                .get(),
            4
        );
    }

    #[test_log::test]
    fn ac11_archivieren_entfernt_das_ziel_aus_den_parkour() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(ziel_id(), name("Keiler"));
        anlage
            .parkour_hinzufügen(ParkourId(Uuid::nil()), parkour_neu(ziel_id()))
            .unwrap();

        anlage.ziel_archivieren(ziel_id()).unwrap();
        assert!(anlage.ziel(ziel_id()).unwrap().ist_archiviert());
        assert!(anlage.parkour()[0].platzierungen().is_empty());
        assert!(anlage.ziel_verwendungen(ziel_id()).is_empty());
    }

    #[test_log::test]
    fn ac12_entfernen_loescht_das_ziel_und_seine_platzierungen() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(ziel_id(), name("Keiler"));
        anlage
            .parkour_hinzufügen(ParkourId(Uuid::nil()), parkour_neu(ziel_id()))
            .unwrap();

        anlage.ziel_entfernen(ziel_id()).unwrap();
        assert!(anlage.ziel(ziel_id()).is_none());
        assert!(anlage.parkour()[0].platzierungen().is_empty());
    }

    #[test_log::test]
    fn deaktivieren_ist_eine_einbahnstrasse() {
        let mut anlage = anlage();
        anlage
            .verwaltung_gewähren(benutzer(), AnlageVerwaltungRolle::Administrator)
            .unwrap();
        let am = DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        let archiviert = anlage.deaktiviere(am);
        assert_eq!(archiviert.deaktiviert_am(), am);
        assert!(archiviert.hat_administrator());
        // No `reaktiviere` exists: the archived type has no mutator at all.
    }

    #[test_log::test]
    fn die_summe_delegiert_die_gemeinsamen_zugriffe() {
        let anlage: Anlage = anlage().into();
        let Anlage::Aktiv(anlage) = anlage else {
            panic!("eine neue Anlage ist aktiv");
        };
        assert_eq!(anlage.name().as_str(), "Breitenstein");
    }

    #[test_log::test]
    fn rollen_roundtrip() {
        for rolle in [
            AnlageVerwaltungRolle::Administrator,
            AnlageVerwaltungRolle::Mitarbeiter,
        ] {
            assert_eq!(
                rolle.as_str().parse::<AnlageVerwaltungRolle>().unwrap(),
                rolle
            );
        }
        assert!("Benutzer".parse::<AnlageVerwaltungRolle>().is_err());
    }
}
