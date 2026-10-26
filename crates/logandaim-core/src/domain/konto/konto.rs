use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{BenutzerId, PasswordHash};
use crate::domain::EmailAdresse;

/// Identity of one Konto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KontoId(pub Uuid);

/// Creation input: everything of a Konto except its identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuesKonto {
    benutzer_id: BenutzerId,
    email: EmailAdresse,
    password_hash: PasswordHash,
    bestätigt_am: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl NeuesKonto {
    /// A Konto only ever exists for a confirmed address (rule 3), so the confirmation
    /// timestamp is not optional: it is the moment registration completed.
    pub fn neu(
        benutzer_id: BenutzerId,
        email: EmailAdresse,
        password_hash: PasswordHash,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            benutzer_id,
            email,
            password_hash,
            bestätigt_am: created_at,
            created_at,
        }
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn bestätigt_am(&self) -> DateTime<Utc> {
        self.bestätigt_am
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

/// One way to authenticate a Benutzer. MVP: a normalized E-Mail-Adresse plus a Passwort.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Konto {
    id: KontoId,
    benutzer_id: BenutzerId,
    email: EmailAdresse,
    password_hash: PasswordHash,
    bestätigt_am: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl Konto {
    pub fn neu(id: KontoId, neu: NeuesKonto) -> Self {
        Self {
            id,
            benutzer_id: neu.benutzer_id,
            email: neu.email,
            password_hash: neu.password_hash,
            bestätigt_am: neu.bestätigt_am,
            created_at: neu.created_at,
        }
    }

    pub fn from_parts(
        id: KontoId,
        benutzer_id: BenutzerId,
        email: EmailAdresse,
        password_hash: PasswordHash,
        bestätigt_am: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            benutzer_id,
            email,
            password_hash,
            bestätigt_am,
            created_at,
        }
    }

    pub fn id(&self) -> KontoId {
        self.id
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn bestätigt_am(&self) -> DateTime<Utc> {
        self.bestätigt_am
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Replace the credential. Used by `PasswortWiederherstellen`; every Anmelde-Sitzung
    /// of the owner is ended separately by that use case.
    pub fn setze_password_hash(&mut self, password_hash: PasswordHash) {
        self.password_hash = password_hash;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::konto::{KlartextPasswort, hash_password};

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn email() -> EmailAdresse {
        EmailAdresse::normalisiert("Anna.Muster@Example.AT ").unwrap()
    }

    /// One real argon2id hash, computed once for this module's tests.
    fn hash() -> &'static PasswordHash {
        static HASH: std::sync::OnceLock<PasswordHash> = std::sync::OnceLock::new();
        HASH.get_or_init(|| {
            let passwort = KlartextPasswort::neu("korrekt-pferd-batterie").unwrap();
            hash_password(&passwort).unwrap()
        })
    }

    #[test_log::test]
    fn neues_konto_bestätigt_die_adresse() {
        let neu = NeuesKonto::neu(BenutzerId(Uuid::nil()), email(), hash().clone(), jetzt());
        assert_eq!(neu.bestätigt_am(), jetzt());
        assert_eq!(neu.email().as_ref(), "anna.muster@example.at");
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let neu = NeuesKonto::neu(BenutzerId(Uuid::nil()), email(), hash().clone(), jetzt());
        let original = Konto::neu(KontoId(Uuid::nil()), neu);
        let geladen = Konto::from_parts(
            original.id(),
            original.benutzer_id(),
            original.email().clone(),
            original.password_hash().clone(),
            original.bestätigt_am(),
            original.created_at(),
        );
        assert_eq!(original, geladen);
    }
}
