//! Guards on the published OpenAPI document itself.
//!
//! The spec is a public product surface: a reader groups by tag, reads the summary, and
//! learns the security scheme from it. A change that drops a tag, a description, or the
//! bearer scheme is a regression a reader sees, so it fails here rather than in review.

use async_trait::async_trait;
use logandaim_api::openapi_service;
use logandaim_core::{
    App,
    application::IdentityProviderConfiguration,
    ports::RepositoryResult,
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};
use serde_json::Value;

struct Unimplemented;

#[async_trait]
impl UnitOfWorkProvider for Unimplemented {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        unimplemented!()
    }
}

/// The document needs only the shape of the app: no port is ever called.
fn spec() -> Value {
    let app = App::builder()
        .uow_provider(Box::new(Unimplemented))
        .konto(IdentityProviderConfiguration::neu(
            "logandaim.at",
            "logandaim-api",
        ))
        .build();
    serde_json::from_str(&openapi_service(app, std::iter::empty::<String>()).spec())
        .expect("the spec is JSON")
}

fn operationen(spec: &Value) -> Vec<(&str, &str, &Value)> {
    spec["paths"]
        .as_object()
        .expect("paths")
        .iter()
        .flat_map(|(path, item)| {
            item.as_object()
                .expect("path item")
                .iter()
                .map(move |(method, operation)| (path.as_str(), method.as_str(), operation))
        })
        .collect()
}

/// Every operation belongs to one published tag, and the tag is one of the five sections.
#[test_log::test]
fn jede_operation_hat_genau_einen_veröffentlichten_tag() {
    let spec = spec();
    let dokumentiert: Vec<&str> = spec["tags"]
        .as_array()
        .expect("tags")
        .iter()
        .map(|tag| tag["name"].as_str().expect("tag name"))
        .collect();
    assert_eq!(
        dokumentiert,
        ["Anlagen", "Betrieb", "Konto", "Parcours", "Runden"],
        "the public sections are fixed"
    );

    for (path, method, operation) in operationen(&spec) {
        let tags = operation["tags"]
            .as_array()
            .unwrap_or_else(|| panic!("{method} {path} carries no tag, so it cannot be grouped"));
        assert_eq!(
            tags.len(),
            1,
            "{method} {path} belongs to exactly one section"
        );
        let tag = tags[0].as_str().expect("tag");
        assert!(
            dokumentiert.contains(&tag),
            "{method} {path} uses the undocumented tag {tag}"
        );
    }
}

/// The bearer scheme is published, and every operation says how it is authenticated.
///
/// Three kinds exist: no token at all (sign-in and its neighbours), a token or anonymous
/// (the two directories), and a token that the handler refuses when missing.
#[test_log::test]
fn bearer_schema_ist_veröffentlicht() {
    let spec = spec();
    let scheme = &spec["components"]["securitySchemes"]["bearerAuth"];
    assert_eq!(
        scheme["type"], "http",
        "the scheme is an HTTP authentication scheme"
    );
    assert_eq!(scheme["scheme"], "bearer");
    assert!(
        scheme["description"]
            .as_str()
            .is_some_and(|text| !text.is_empty()),
        "a reader has to learn what the header carries"
    );

    const OHNE_TOKEN: [(&str, &str); 7] = [
        ("/health", "get"),
        ("/registrierung/start", "post"),
        ("/registrierung/abschluss", "post"),
        ("/anmeldung", "post"),
        ("/session/erneuern", "post"),
        ("/passwort/vergessen", "post"),
        ("/passwort/wiederherstellen", "post"),
    ];
    const AUCH_ANONYM: [(&str, &str); 2] = [("/anlagen/oeffentlich", "get"), ("/parcours", "get")];

    for (path, method, operation) in operationen(&spec) {
        let key = (path, method);
        let Some(security) = operation["security"].as_array() else {
            assert!(
                OHNE_TOKEN.contains(&key),
                "{method} {path} omits security without being a sign-in route"
            );
            continue;
        };
        let ohne_token = security
            .iter()
            .any(|alt| alt.as_object().is_some_and(|o| o.is_empty()));
        assert_eq!(
            ohne_token,
            AUCH_ANONYM.contains(&key),
            "{method} {path} declares the wrong anonymous alternative"
        );
        assert!(
            security.iter().any(|alt| alt.get("bearerAuth").is_some()),
            "{method} {path} has no bearer alternative"
        );
    }
}

/// Every operation and every published schema is described, because that is the document.
#[test_log::test]
fn jede_operation_und_jedes_schema_ist_beschrieben() {
    let spec = spec();
    for (path, method, operation) in operationen(&spec) {
        assert!(
            operation["summary"]
                .as_str()
                .is_some_and(|text| !text.is_empty()),
            "{method} {path} has no summary"
        );
        assert!(
            operation["description"]
                .as_str()
                .is_some_and(|text| !text.is_empty()),
            "{method} {path} has no description"
        );
    }
    for (name, schema) in spec["components"]["schemas"].as_object().expect("schemas") {
        assert!(
            schema["description"]
                .as_str()
                .is_some_and(|text| !text.is_empty()),
            "schema {name} has no description"
        );
    }
}

/// Internal references never reach the published document.
#[test_log::test]
fn keine_internen_verweise_in_der_öffentlichen_spec() {
    let spec = spec();
    let text = spec.to_string();
    for marker in ["AC-", "SPEC-", "ADR-"] {
        assert!(
            !text.contains(marker),
            "the published spec mentions {marker}"
        );
    }
}
