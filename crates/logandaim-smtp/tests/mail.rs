//! The adapter's envelope is asserted without a relay: recipient, subject, and body are
//! what AC-19 depends on, and a transport that reaches a live Mailpit would make this
//! crate's test suite depend on a daemon.

use lettre::message::header::Subject;
use logandaim_core::{
    domain::EmailAdresse,
    ports::{Mail, MailError},
};
use logandaim_smtp::{SmtpKonfiguration, SmtpMailSender};

fn absender() -> EmailAdresse {
    EmailAdresse::normalisiert("noreply@logandaim.at").expect("static address is valid")
}

fn sender() -> SmtpMailSender {
    let mut konfiguration = SmtpKonfiguration::mailpit(absender());
    konfiguration.frontend_basis_url = "https://logandaim.at".to_owned();
    SmtpMailSender::neu(konfiguration).expect("mailpit relay builds")
}

#[test_log::test(tokio::test)]
async fn die_nachricht_trägt_empfänger_betreff_und_klartext_rumpf() {
    let sender = sender();
    let mail = Mail::email_bestätigung(
        EmailAdresse::normalisiert("anna@muster.at").expect("address is valid"),
        "abc",
    );

    let nachricht = sender.baue_nachricht(&mail).expect("the message builds");

    // The SMTP envelope is what the relay actually routes on, so assert it directly
    // rather than the `To:` header a client later renders.
    let empfänger: Vec<String> = nachricht
        .envelope()
        .to()
        .iter()
        .map(|adresse| adresse.to_string())
        .collect();
    assert_eq!(empfänger, vec!["anna@muster.at".to_owned()]);

    let betreff = nachricht
        .headers()
        .get::<Subject>()
        .expect("the subject header is set");
    assert_eq!(
        betreff,
        Subject::from("Bestätige deine E-Mail-Adresse".to_owned())
    );

    // Plain text: the link must survive without an HTML renderer rewriting it. The body
    // arrives quoted-printable encoded, so decode before comparing.
    let roh = nachricht.formatted();
    let roh = String::from_utf8_lossy(&roh);
    assert!(roh.contains("Content-Type: text/plain"), "kein HTML: {roh}");
    let rumpf = entfalte_quoted_printable(&roh);
    assert!(
        rumpf.contains("https://logandaim.at/registrierung?token=abc"),
        "der Link steht unversehrt im Rumpf: {rumpf}"
    );
}

/// A message that cannot even be built is a delivery failure, never a panic and never a
/// silent success: the use case logs it and the request still answers (NFR-5).
#[test_log::test(tokio::test)]
async fn eine_ungültige_adresse_ist_ein_zustellungsfehler() {
    let sender = sender();
    // `EmailAdresse` only checks for `@` and `.`; the space makes the mailbox unparseable.
    let mail = Mail::email_bestätigung(
        EmailAdresse::normalisiert("anna muster@muster.at").expect("domain address is lenient"),
        "abc",
    );

    let fehler = sender
        .baue_nachricht(&mail)
        .expect_err("die Adresse lässt sich nicht als Mailbox parsen");
    assert!(matches!(fehler.current_context(), MailError::Zustellung));
}

/// Undo `quoted-printable`: soft line breaks and `=XX` escapes both have to go before a
/// link can be compared.
fn entfalte_quoted_printable(nachricht: &str) -> String {
    let rumpf = nachricht
        .split_once("\r\n\r\n")
        .map(|(_, rumpf)| rumpf)
        .unwrap_or(nachricht);
    let ohne_umbruch = rumpf.replace("=\r\n", "");

    let bytes = ohne_umbruch.as_bytes();
    let mut ergebnis = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'='
            && i + 2 < bytes.len()
            && let Ok(byte) =
                u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16)
        {
            ergebnis.push(byte);
            i += 3;
            continue;
        }
        ergebnis.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&ergebnis).into_owned()
}
