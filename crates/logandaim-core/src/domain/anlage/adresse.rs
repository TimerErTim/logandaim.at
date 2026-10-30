//! Structured address of an `Anlage`.
//!
//! Distinct from [`Adresse`](crate::domain::Adresse), which the `konto` slice keeps as an
//! unvalidated record. An Anlage's address is required master data (SPEC-stammdaten
//! rule 3, AC-1), so every field is validated here.

use crate::domain::Ländercode;

#[derive(Debug, thiserror::Error)]
pub enum AnlageAdresseFehler {
    #[error("die Straße darf nicht leer sein")]
    StraßeLeer,
    #[error("die Postleitzahl darf nicht leer sein")]
    PostleitzahlLeer,
    #[error("der Ort darf nicht leer sein")]
    OrtLeer,
}

/// Required street, postal code, locality, and country of one Anlage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnlageAdresse {
    straße: String,
    postleitzahl: String,
    ort: String,
    ländercode: Ländercode,
}

impl AnlageAdresse {
    pub fn neu(
        straße: impl AsRef<str>,
        postleitzahl: impl AsRef<str>,
        ort: impl AsRef<str>,
        ländercode: Ländercode,
    ) -> Result<Self, AnlageAdresseFehler> {
        let straße = straße.as_ref().trim();
        if straße.is_empty() {
            return Err(AnlageAdresseFehler::StraßeLeer);
        }
        let postleitzahl = postleitzahl.as_ref().trim();
        if postleitzahl.is_empty() {
            return Err(AnlageAdresseFehler::PostleitzahlLeer);
        }
        let ort = ort.as_ref().trim();
        if ort.is_empty() {
            return Err(AnlageAdresseFehler::OrtLeer);
        }
        Ok(Self {
            straße: straße.to_owned(),
            postleitzahl: postleitzahl.to_owned(),
            ort: ort.to_owned(),
            ländercode,
        })
    }

    pub fn straße(&self) -> &str {
        &self.straße
    }

    pub fn postleitzahl(&self) -> &str {
        &self.postleitzahl
    }

    pub fn ort(&self) -> &str {
        &self.ort
    }

    pub fn ländercode(&self) -> Ländercode {
        self.ländercode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn vollstaendige_adresse_wird_angenommen() {
        let adresse =
            AnlageAdresse::neu("Waldweg 1", "3100", "St. Pölten", Ländercode::AT).unwrap();
        assert_eq!(adresse.straße(), "Waldweg 1");
        assert_eq!(adresse.postleitzahl(), "3100");
        assert_eq!(adresse.ort(), "St. Pölten");
        assert_eq!(adresse.ländercode(), Ländercode::AT);
    }

    #[test_log::test]
    fn pflichtfelder_duerfen_nicht_leer_sein() {
        assert!(matches!(
            AnlageAdresse::neu(" ", "3100", "St. Pölten", Ländercode::AT),
            Err(AnlageAdresseFehler::StraßeLeer)
        ));
        assert!(matches!(
            AnlageAdresse::neu("Waldweg 1", "", "St. Pölten", Ländercode::AT),
            Err(AnlageAdresseFehler::PostleitzahlLeer)
        ));
        assert!(matches!(
            AnlageAdresse::neu("Waldweg 1", "3100", "  ", Ländercode::AT),
            Err(AnlageAdresseFehler::OrtLeer)
        ));
    }
}
