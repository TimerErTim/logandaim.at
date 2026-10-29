//! The `Anlage` aggregate: a physical site with its Parcours, Ziele, and authority.
//!
//! `Anlage` is the one aggregate root of this slice that owns entities: `Ziel` and
//! `Parcours` have no lifecycle outside it, so they are children, not roots. Authority
//! (`AnlageVerwaltung`) lives here too, which is why loading one aggregate answers both
//! "may you?" and "what is the state?" (SPEC-stammdaten rules 2, 4, 5, 10).
//!
//! The root is a **typestate** over its lifecycle: only `AnlageIn<Aktiv>` has mutators, and
//! [`AnlageIn::deaktiviere`] is the only transition. There is no `reaktiviere`, so
//! "archived data cannot be restored" (AC-11) is a compile error rather than a runtime
//! check. The children use a plain [`Zustand`] enum instead: they are never loaded or saved
//! on their own, so a phantom parameter would buy safety where no caller could act.

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod adresse;
pub mod kontaktdaten;
pub mod parcours;
pub mod snapshot;
pub mod ziel;

pub use adresse::{AnlageAdresse, AnlageAdresseFehler};
pub use kontaktdaten::{AnlageKontakt, Website, WebsiteFehler};
pub use parcours::{
    Abstand, NeuerParcours, Parcours, ParcoursFehler, ParcoursId, ParcoursName, Pflock,
    PflockFarbe, PflockId, Zielplatzierung,
};
pub use snapshot::{
    ParcoursSnapshot, SnapshotFehler, SnapshotPflock, SnapshotQuelle, SnapshotZiel,
};
pub use ziel::{
    NeuesZiel, Tiername, Ziel, ZielFehler, ZielId, Zielgruppe, Zustand, ZustandUnbekannt,
};

use crate::domain::konto::{Benutzer, BenutzerId};

/// Identity of one Anlage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnlageId(pub Uuid);

/// Non-empty name of one Anlage. Names need not be unique (AC-1).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnlageName(String);

#[derive(Debug, thiserror::Error)]
#[error("der Anlagename darf nicht leer sein")]
pub struct AnlageNameLeer;

impl AnlageName {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, AnlageNameLeer> {
        let wert = wert.as_ref().trim();
        if wert.is_empty() {
            return Err(AnlageNameLeer);
        }
        Ok(Self(wert.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AnlageName {
    fn as_ref(&self) -> &str {
        &self.0
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
    #[error("der Parcours wurde nicht gefunden")]
    ParcoursNichtGefunden,
    #[error("das Ziel gehört nicht zu dieser Anlage")]
    ZielUnbekannt,
    #[error("das Ziel ist archiviert und kann nicht mehr geändert werden")]
    ZielArchiviert,
    #[error("der Parcours gehört nicht zu dieser Anlage")]
    ParcoursUnbekannt,
    #[error("das Ziel ist bereits in diesem Parcours platziert")]
    ZielBereitsPlatziert,
    #[error("der Parcours verwendet ein Ziel, das nicht zu dieser Anlage gehört")]
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
    parcours: Vec<Parcours>,
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

    pub fn parcours(&self) -> &[Parcours] {
        &self.parcours
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

    pub fn parcours_mit_id(&self, parcours_id: ParcoursId) -> Option<&Parcours> {
        self.parcours
            .iter()
            .find(|parcours| parcours.id() == parcours_id)
    }

    /// An active Parcours by identity, or `None` when it is missing or archived.
    pub fn aktiver_parcours(&self, parcours_id: ParcoursId) -> Option<&Parcours> {
        self.parcours_mit_id(parcours_id)
            .filter(|parcours| !parcours.ist_archiviert())
    }

    /// The Parcours a Runde may be started on: active, and belonging to an active Anlage.
    pub fn startbarer_parcours(&self, parcours_id: ParcoursId) -> Option<&Parcours> {
        self.aktiver_parcours(parcours_id)
    }

    /// Capture the immutable setup of one Parcours as a value copy (AC-7, AC-9).
    ///
    /// Only the `Anlage` can read its own children's current values, which is why
    /// snapshot capture lives here rather than in `Runde` or `Parcours`.
    pub fn snapshot_für(&self, parcours_id: ParcoursId) -> Option<ParcoursSnapshot> {
        let parcours = self.parcours_mit_id(parcours_id)?;
        let ziele = parcours
            .platzierungen()
            .iter()
            .map(|platzierung| {
                let ziel = self.ziel(platzierung.ziel_id())?;
                // In the Parcours' own Pflock order (far to near), never in storage order:
                // the snapshot must read the same however an adapter happens to spell its rows.
                let abstände = parcours
                    .pflöcke()
                    .iter()
                    .filter_map(|pflock| {
                        platzierung
                            .abstand_für(pflock.id())
                            .map(|abstand| (pflock.id(), abstand.meter()))
                    })
                    .collect();
                Some((
                    ziel.id(),
                    ziel.tiername().clone(),
                    ziel.zielgruppe(),
                    abstände,
                ))
            })
            .collect::<Option<Vec<_>>>()?;

        let quelle = SnapshotQuelle::neu(
            parcours.id(),
            parcours.name().as_str(),
            parcours
                .pflöcke()
                .iter()
                .map(|pflock| (pflock.id(), pflock.farbe().as_str().to_owned()))
                .collect(),
            ziele,
        );
        quelle.in_snapshot().ok()
    }

    /// Where one physical Ziel is used across this Anlage's Parcours: in which routes,
    /// at which order position, and at which distances.
    ///
    /// This is the placement aggregation the aggregate contributes; the per-Ziel
    /// *Wertung* belongs to `Schussmenge`, because an Anlage never sees a Schuss.
    pub fn ziel_verwendungen(&self, ziel_id: ZielId) -> Vec<Zielverwendung> {
        self.parcours
            .iter()
            .filter(|parcours| !parcours.ist_archiviert())
            .enumerate()
            .filter_map(|(parcours_ordnung, parcours)| {
                let position = parcours
                    .platzierungen()
                    .iter()
                    .position(|platzierung| platzierung.ziel_id() == ziel_id)?;
                let platzierung = &parcours.platzierungen()[position];
                let abstände = parcours
                    .pflöcke()
                    .iter()
                    .filter_map(|pflock| {
                        platzierung
                            .abstand_für(pflock.id())
                            .map(|abstand| (pflock.farbe().as_str().to_owned(), abstand.meter()))
                    })
                    .collect();
                Some(Zielverwendung {
                    parcours_id: parcours.id(),
                    parcours_name: parcours.name().as_str().to_owned(),
                    parcours_ordnung,
                    position,
                    abstände,
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
            parcours: Vec::new(),
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
        parcours: Vec<Parcours>,
    ) -> Self {
        Self {
            id,
            name,
            adresse,
            kontakt,
            verwaltungen,
            ziele,
            parcours,
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

    /// Add a physical Ziel at this Anlage.
    pub fn ziel_hinzufügen(&mut self, id: ZielId, neu: NeuesZiel) -> ZielId {
        self.ziele.push(Ziel::neu(id, neu));
        id
    }

    /// Edit a Ziel's descriptive fields. Its identity does not change (AC-5).
    pub fn ziel_ändern(
        &mut self,
        ziel_id: ZielId,
        tiername: Tiername,
        zielgruppe: Zielgruppe,
    ) -> Result<(), AnlageFehler> {
        self.ziele
            .iter_mut()
            .find(|ziel| ziel.id() == ziel_id)
            .ok_or(AnlageFehler::ZielNichtGefunden)?
            .ändern(tiername, zielgruppe)
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
        for parcours in &mut self.parcours {
            if parcours.ist_archiviert() {
                continue;
            }
            let _ = parcours.platzierung_entfernen(ziel_id);
        }
        Ok(())
    }

    /// Delete a Ziel outright, for master data with no history (AC-12).
    pub fn ziel_entfernen(&mut self, ziel_id: ZielId) -> Result<(), AnlageFehler> {
        if self.ziel(ziel_id).is_none() {
            return Err(AnlageFehler::ZielUnbekannt);
        }
        self.ziele.retain(|ziel| ziel.id() != ziel_id);
        for parcours in &mut self.parcours {
            if parcours.ist_archiviert() {
                continue;
            }
            let _ = parcours.platzierung_entfernen(ziel_id);
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
        neues_ziel: NeuesZiel,
    ) -> Result<ZielId, AnlageFehler> {
        let altes = self.ziel(alt).ok_or(AnlageFehler::ZielUnbekannt)?;
        if altes.ist_archiviert() {
            return Err(AnlageFehler::ZielArchiviert);
        }
        // Hand the slot over *before* archiving: archiving strips the old placement from
        // every active route, so doing it first would lose the position the new Ziel takes.
        for parcours in &mut self.parcours {
            if parcours.ist_archiviert() {
                continue;
            }
            parcours
                .platzierung_ersetzen(alt, neu)
                .map_err(|_| AnlageFehler::ZielBereitsPlatziert)?;
        }
        self.ziel_archivieren(alt)?;
        self.ziele.push(Ziel::neu(neu, neues_ziel));
        Ok(neu)
    }

    /// Rename a Parcours. Refused while archived.
    pub fn parcours_umbenennen(
        &mut self,
        parcours_id: ParcoursId,
        name: ParcoursName,
    ) -> Result<(), ParcoursFehler> {
        self.parcours
            .iter_mut()
            .find(|parcours| parcours.id() == parcours_id)
            .ok_or(ParcoursFehler::ParcoursNichtGefunden)?
            .umbenennen(name)
    }

    /// Add a Parcours. Every placed Ziel must belong to this Anlage (AC-4).
    pub fn parcours_hinzufügen(
        &mut self,
        id: ParcoursId,
        neu: NeuerParcours,
    ) -> Result<ParcoursId, AnlageFehler> {
        for platzierung in neu.platzierungen() {
            if self.ziel(platzierung.ziel_id()).is_none() {
                return Err(AnlageFehler::ZielFremdeAnlage);
            }
        }
        self.parcours.push(Parcours::neu(id, neu));
        Ok(id)
    }

    /// Replace a Parcours' whole setup. Every placed Ziel must still belong here.
    pub fn parcours_setup_ändern(
        &mut self,
        parcours_id: ParcoursId,
        pflöcke: Vec<Pflock>,
        platzierungen: Vec<Zielplatzierung>,
    ) -> Result<(), ParcoursFehler> {
        for platzierung in &platzierungen {
            if self.ziel(platzierung.ziel_id()).is_none() {
                return Err(ParcoursFehler::ZielFremdeAnlage);
            }
        }
        self.parcours
            .iter_mut()
            .find(|parcours| parcours.id() == parcours_id)
            .ok_or(ParcoursFehler::ParcoursNichtGefunden)?
            .setup_ändern(pflöcke, platzierungen)
    }

    /// Archive a Parcours, keeping its identity and its history references (AC-11).
    pub fn parcours_archivieren(&mut self, parcours_id: ParcoursId) -> Result<(), ParcoursFehler> {
        self.parcours
            .iter_mut()
            .find(|parcours| parcours.id() == parcours_id)
            .ok_or(ParcoursFehler::ParcoursNichtGefunden)?
            .archiviere()
    }

    /// Delete a Parcours outright, for one with no history (AC-12).
    pub fn parcours_entfernen(&mut self, parcours_id: ParcoursId) -> Result<(), AnlageFehler> {
        if self.parcours_mit_id(parcours_id).is_none() {
            return Err(AnlageFehler::ParcoursUnbekannt);
        }
        self.parcours
            .retain(|parcours| parcours.id() != parcours_id);
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
            parcours: self.parcours,
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
        parcours: Vec<Parcours>,
        deaktiviert_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            adresse,
            kontakt,
            verwaltungen,
            ziele,
            parcours,
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

    pub fn parcours(&self) -> &[Parcours] {
        match self {
            Self::Aktiv(anlage) => anlage.parcours(),
            Self::Archiviert(anlage) => anlage.parcours(),
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

    pub fn parcours_mit_id(&self, parcours_id: ParcoursId) -> Option<&Parcours> {
        match self {
            Self::Aktiv(anlage) => anlage.parcours_mit_id(parcours_id),
            Self::Archiviert(anlage) => anlage.parcours_mit_id(parcours_id),
        }
    }

    /// Placement aggregation, delegated so a reader holding the sum never matches on state.
    pub fn ziel_verwendungen(&self, ziel_id: ZielId) -> Vec<Zielverwendung> {
        match self {
            Self::Aktiv(anlage) => anlage.ziel_verwendungen(ziel_id),
            Self::Archiviert(anlage) => anlage.ziel_verwendungen(ziel_id),
        }
    }

    /// Capture a Parcours snapshot, delegated to whichever state holds the children.
    pub fn snapshot_für(&self, parcours_id: ParcoursId) -> Option<ParcoursSnapshot> {
        match self {
            Self::Aktiv(anlage) => anlage.snapshot_für(parcours_id),
            Self::Archiviert(anlage) => anlage.snapshot_für(parcours_id),
        }
    }
}

/// One Parcours' use of one physical Ziel, as the Anlage sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zielverwendung {
    /// Identity and name of the Parcours (its own copy, not a wizard-held reference).
    pub parcours_id: ParcoursId,
    pub parcours_name: String,
    /// Position of the Parcours in the Anlage's list, so a reader can order them.
    pub parcours_ordnung: usize,
    /// 0-based position of the Ziel in that Parcours' route.
    pub position: usize,
    /// Pflock label to metres, for the configured distances.
    pub abstände: Vec<(String, u32)>,
}

/// Whether a Benutzer may act on an Anlage at all.
///
/// The use case combines this with the aggregate's own role, and the answer never comes
/// from a permission table (NFR-2).
pub fn ist_systemadministrator_oder_verwalter(
    benutzer: &Benutzer,
    anlage: &impl AnlagenSicht,
    benutzer_id: BenutzerId,
) -> bool {
    benutzer.ist_systemadministrator() || anlage.rolle_von(benutzer_id).is_some()
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

    fn pflock(id: u128, farbe: &str) -> Pflock {
        Pflock::neu(
            PflockId(Uuid::from_u128(id)),
            PflockFarbe::neu(farbe).unwrap(),
        )
    }

    fn parcours_neu(ziel: ZielId) -> NeuerParcours {
        NeuerParcours::neu(
            ParcoursName::neu("Waldrunde").unwrap(),
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
    fn ac4_ein_parcours_verwendet_nur_ziele_dieser_anlage() {
        let mut anlage = anlage();
        assert!(matches!(
            anlage.parcours_hinzufügen(ParcoursId(Uuid::nil()), parcours_neu(ziel_id())),
            Err(AnlageFehler::ZielFremdeAnlage)
        ));

        anlage.ziel_hinzufügen(
            ziel_id(),
            NeuesZiel::neu(
                Tiername::neu("Keiler").unwrap(),
                Zielgruppe::neu(2).unwrap(),
            ),
        );
        anlage
            .parcours_hinzufügen(ParcoursId(Uuid::nil()), parcours_neu(ziel_id()))
            .unwrap();
        assert_eq!(anlage.parcours().len(), 1);
    }

    #[test_log::test]
    fn ac4_ein_ziel_kann_in_mehreren_parcours_liegen() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(
            ziel_id(),
            NeuesZiel::neu(
                Tiername::neu("Keiler").unwrap(),
                Zielgruppe::neu(2).unwrap(),
            ),
        );
        anlage
            .parcours_hinzufügen(ParcoursId(Uuid::nil()), parcours_neu(ziel_id()))
            .unwrap();
        anlage
            .parcours_hinzufügen(ParcoursId(Uuid::from_u128(2)), parcours_neu(ziel_id()))
            .unwrap();
        assert_eq!(anlage.ziel_verwendungen(ziel_id()).len(), 2);
    }

    #[test_log::test]
    fn ac7_der_snapshot_ist_eine_wertkopie() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(
            ziel_id(),
            NeuesZiel::neu(
                Tiername::neu("Keiler").unwrap(),
                Zielgruppe::neu(2).unwrap(),
            ),
        );
        let parcours_id = ParcoursId(Uuid::nil());
        anlage
            .parcours_hinzufügen(parcours_id, parcours_neu(ziel_id()))
            .unwrap();

        let snapshot = anlage.snapshot_für(parcours_id).unwrap();
        assert_eq!(snapshot.parcours_name(), "Waldrunde");
        assert_eq!(snapshot.ziele()[0].tiername(), "Keiler");

        // A later edit changes the aggregate, never the captured copy.
        anlage
            .ziel_ändern(
                ziel_id(),
                Tiername::neu("Reh").unwrap(),
                Zielgruppe::neu(4).unwrap(),
            )
            .unwrap();
        assert_eq!(snapshot.ziele()[0].tiername(), "Keiler");
        assert_eq!(snapshot.ziele()[0].zielgruppe(), 2);

        let neuer_snapshot = anlage.snapshot_für(parcours_id).unwrap();
        assert_eq!(neuer_snapshot.ziele()[0].tiername(), "Reh");
    }

    #[test_log::test]
    fn ac11_archivieren_entfernt_das_ziel_aus_den_parcours() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(
            ziel_id(),
            NeuesZiel::neu(
                Tiername::neu("Keiler").unwrap(),
                Zielgruppe::neu(2).unwrap(),
            ),
        );
        anlage
            .parcours_hinzufügen(ParcoursId(Uuid::nil()), parcours_neu(ziel_id()))
            .unwrap();

        anlage.ziel_archivieren(ziel_id()).unwrap();
        assert!(anlage.ziel(ziel_id()).unwrap().ist_archiviert());
        assert!(anlage.parcours()[0].platzierungen().is_empty());
        assert!(anlage.ziel_verwendungen(ziel_id()).is_empty());
    }

    #[test_log::test]
    fn ac12_entfernen_loescht_das_ziel_und_seine_platzierungen() {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(
            ziel_id(),
            NeuesZiel::neu(
                Tiername::neu("Keiler").unwrap(),
                Zielgruppe::neu(2).unwrap(),
            ),
        );
        anlage
            .parcours_hinzufügen(ParcoursId(Uuid::nil()), parcours_neu(ziel_id()))
            .unwrap();

        anlage.ziel_entfernen(ziel_id()).unwrap();
        assert!(anlage.ziel(ziel_id()).is_none());
        assert!(anlage.parcours()[0].platzierungen().is_empty());
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
