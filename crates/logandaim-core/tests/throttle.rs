//! AC-15 / AC-16: the client-scoped throttle as the use cases see it.
//!
//! These live outside `cases/` because that tree proves UoW adapter parity. A throttle
//! budget is a property of the use case plus the in-memory adapter, not of a repository.

use std::net::{IpAddr, Ipv4Addr};

use logandaim_core::{
    App,
    adapters::InMemoryThrottle,
    application::{AuthContext, IdentityProviderConfiguration},
    domain::{EmailAdresse, konto::KlartextPasswort},
    ports::ClientKey,
    service::{Anmeldung, AnmeldungFehler, RegistrierungStarten, RegistrierungStartenFehler},
};
use logandaim_fakes::FakeUnitOfWorkProvider;

fn client_key(ip: u8) -> ClientKey {
    ClientKey::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, ip)), "test-client")
}

fn adresse(wert: &str) -> EmailAdresse {
    EmailAdresse::normalisiert(wert).expect("Testadresse ist gültig")
}

fn passwort() -> KlartextPasswort {
    KlartextPasswort::neu("korrekt-pferd-batterie").expect("Testpasswort ist gültig")
}

fn app() -> App {
    App::builder()
        .uow_provider(Box::new(FakeUnitOfWorkProvider::empty()))
        .throttle(std::sync::Arc::new(InMemoryThrottle::new()))
        .konto(IdentityProviderConfiguration::neu(
            "logandaim.at",
            "logandaim-api",
        ))
        .build()
}

fn text<E>(report: error_stack::Report<E>) -> String
where
    E: std::error::Error + Send + Sync + 'static,
{
    report.current_context().to_string()
}

#[test_log::test(pollster::test)]
async fn ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client() {
    let app = app();
    let client = client_key(30);

    for _ in 0..20 {
        let ergebnis = app
            .execute(
                AuthContext::Anonymous,
                Anmeldung {
                    email: adresse("niemand@muster.at"),
                    passwort: passwort(),
                    client_key: client.clone(),
                },
            )
            .await;
        assert!(ergebnis.is_err());
        assert_ne!(
            text(ergebnis.unwrap_err()),
            AnmeldungFehler::Gedrosselt.to_string(),
            "innerhalb des Budgets ist die Antwort ein Zugangsfehler, keine Drosselung"
        );
    }

    let einundzwanzigster = app
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: adresse("niemand@muster.at"),
                passwort: passwort(),
                client_key: client,
            },
        )
        .await;
    assert_eq!(
        text(einundzwanzigster.unwrap_err()),
        AnmeldungFehler::Gedrosselt.to_string()
    );

    // Another client is unaffected: the limit is never per address.
    let anderer_client = app
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: adresse("niemand@muster.at"),
                passwort: passwort(),
                client_key: client_key(31),
            },
        )
        .await;
    assert_ne!(
        text(anderer_client.unwrap_err()),
        AnmeldungFehler::Gedrosselt.to_string()
    );
}

#[test_log::test(pollster::test)]
async fn ac16_registrierung_ist_auf_fünf_pro_stunde_gedrosselt() {
    let app = app();
    let client = client_key(32);

    for nummer in 0..5 {
        app.execute(
            AuthContext::Anonymous,
            RegistrierungStarten {
                email: adresse(&format!("person{nummer}@muster.at")),
                client_key: client.clone(),
            },
        )
        .await
        .expect("die ersten fünf Anfragen gehen durch");
    }

    let sechste = app
        .execute(
            AuthContext::Anonymous,
            RegistrierungStarten {
                email: adresse("person6@muster.at"),
                client_key: client,
            },
        )
        .await;
    assert_eq!(
        text(sechste.unwrap_err()),
        RegistrierungStartenFehler::Gedrosselt.to_string()
    );
}
