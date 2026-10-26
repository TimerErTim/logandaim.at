//! Client-scoped throttle in one process's memory.
//!
//! This is the **production** adapter, not a test double: logandaim.at runs as one process
//! against one SQLite file, so a shared store would add a network hop and a second state to
//! keep consistent for no gain. A multi-process deployment would need a different adapter,
//! and that would be a deliberate change, not a default.
//!
//! Expired windows reset on read, so nothing here needs a background sweeper. The downside
//! is honest and worth stating: a restart forgives every spent budget, and memory grows with
//! the number of distinct clients. Both are acceptable at this size and neither weakens the
//! threshold behaviour AC-15 and AC-16 assert.

use std::collections::HashMap;
use std::sync::Mutex;

use chrono::{DateTime, Utc};

use crate::{
    ResultReport,
    ports::{Drossel, DrosselAktion, DrosselFehler, KundeSchlüssel},
};

/// One client's budget for one action.
struct Fenster {
    start: DateTime<Utc>,
    anzahl: u32,
}

pub struct InMemoryDrossel {
    fenster: Mutex<HashMap<(KundeSchlüssel, &'static str), Fenster>>,
}

impl InMemoryDrossel {
    pub fn new() -> Self {
        Self {
            fenster: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryDrossel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Drossel for InMemoryDrossel {
    async fn prüfe_und_zähle(
        &self,
        kunde: &KundeSchlüssel,
        aktion: DrosselAktion,
        jetzt: DateTime<Utc>,
    ) -> ResultReport<(), DrosselFehler> {
        let mut fenster = self
            .fenster
            .lock()
            .expect("the throttle mutex is not poisoned");
        let eintrag = fenster
            .entry((kunde.clone(), aktion.as_str()))
            .or_insert_with(|| Fenster {
                start: jetzt,
                anzahl: 0,
            });
        if jetzt - eintrag.start >= aktion.fenster() {
            eintrag.start = jetzt;
            eintrag.anzahl = 0;
        }
        if eintrag.anzahl >= aktion.schwelle() {
            return Err(error_stack::Report::new(DrosselFehler::ZuVieleVersuche));
        }
        eintrag.anzahl += 1;
        Ok(())
    }

    async fn zurücksetzen(
        &self,
        kunde: &KundeSchlüssel,
        aktion: DrosselAktion,
    ) -> ResultReport<(), DrosselFehler> {
        self.fenster
            .lock()
            .expect("the throttle mutex is not poisoned")
            .remove(&(kunde.clone(), aktion.as_str()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;

    fn kunde(fingerprint: &str) -> KundeSchlüssel {
        KundeSchlüssel::neu(
            "203.0.113.7".parse::<IpAddr>().expect("static address"),
            fingerprint,
        )
    }

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).expect("fixed instant")
    }

    #[test_log::test(pollster::test)]
    async fn das_budget_wird_genau_einmal_ausgeschöpft() {
        let drossel = InMemoryDrossel::new();
        let kunde = kunde("a");
        let aktion = DrosselAktion::RegistrierungStart;

        for i in 0..aktion.schwelle() {
            assert!(
                drossel
                    .prüfe_und_zähle(&kunde, aktion, jetzt())
                    .await
                    .is_ok(),
                "Versuch {i} liegt im Budget"
            );
        }
        assert!(
            drossel
                .prüfe_und_zähle(&kunde, aktion, jetzt())
                .await
                .is_err()
        );
    }

    #[test_log::test(pollster::test)]
    async fn nach_dem_fenster_ist_das_budget_wieder_frei() {
        let drossel = InMemoryDrossel::new();
        let kunde = kunde("a");
        let aktion = DrosselAktion::RegistrierungStart;

        for _ in 0..aktion.schwelle() {
            drossel
                .prüfe_und_zähle(&kunde, aktion, jetzt())
                .await
                .unwrap();
        }
        assert!(
            drossel
                .prüfe_und_zähle(&kunde, aktion, jetzt())
                .await
                .is_err()
        );

        let später = jetzt() + aktion.fenster();
        assert!(
            drossel
                .prüfe_und_zähle(&kunde, aktion, später)
                .await
                .is_ok()
        );
    }

    #[test_log::test(pollster::test)]
    async fn aktionen_und_clients_haben_getrennte_budgets() {
        let drossel = InMemoryDrossel::new();
        let a = kunde("a");
        let b = kunde("b");

        for _ in 0..DrosselAktion::RegistrierungStart.schwelle() {
            drossel
                .prüfe_und_zähle(&a, DrosselAktion::RegistrierungStart, jetzt())
                .await
                .unwrap();
        }
        assert!(
            drossel
                .prüfe_und_zähle(&a, DrosselAktion::RegistrierungStart, jetzt())
                .await
                .is_err()
        );

        // Another client, and another action of the same client, are unaffected.
        assert!(
            drossel
                .prüfe_und_zähle(&b, DrosselAktion::RegistrierungStart, jetzt())
                .await
                .is_ok()
        );
        assert!(
            drossel
                .prüfe_und_zähle(&a, DrosselAktion::Anmeldung, jetzt())
                .await
                .is_ok()
        );
    }

    #[test_log::test(pollster::test)]
    async fn zurücksetzen_gibt_das_budget_frei() {
        let drossel = InMemoryDrossel::new();
        let kunde = kunde("a");
        let aktion = DrosselAktion::Anmeldung;

        for _ in 0..aktion.schwelle() {
            drossel
                .prüfe_und_zähle(&kunde, aktion, jetzt())
                .await
                .unwrap();
        }
        assert!(
            drossel
                .prüfe_und_zähle(&kunde, aktion, jetzt())
                .await
                .is_err()
        );

        drossel.zurücksetzen(&kunde, aktion).await.unwrap();
        assert!(
            drossel
                .prüfe_und_zähle(&kunde, aktion, jetzt())
                .await
                .is_ok()
        );
    }
}
