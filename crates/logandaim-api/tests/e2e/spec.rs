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
        ["Anlagen", "Betrieb", "Konto", "Parkour", "Runden"],
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

    const OHNE_TOKEN: [(&str, &str); 8] = [
        ("/health", "get"),
        ("/health/mail", "get"),
        ("/registrierung/start", "post"),
        ("/registrierung/abschluss", "post"),
        ("/anmeldung", "post"),
        ("/session/erneuern", "post"),
        ("/passwort/vergessen", "post"),
        ("/passwort/wiederherstellen", "post"),
    ];
    const AUCH_ANONYM: [(&str, &str); 2] = [("/anlagen/oeffentlich", "get"), ("/parkour", "get")];

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

/// Every wire identifier in the published contract is ASCII; prose remains unrestricted.
#[test_log::test]
fn wire_identifikatoren_sind_ascii() {
    fn assert_ascii(value: &str, context: &str) {
        assert!(
            value.is_ascii(),
            "non-ASCII wire identifier in {context}: {value}"
        );
    }

    fn schema_identifiers_are_ascii(schema: &Value, context: &str) {
        if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
            for (name, property) in properties {
                assert_ascii(name, &format!("{context} property"));
                schema_identifiers_are_ascii(property, &format!("{context}.{name}"));
            }
        }
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            assert_ascii(reference, &format!("{context} reference"));
        }
        if let Some(discriminator) = schema.get("discriminator") {
            if let Some(name) = discriminator.get("propertyName").and_then(Value::as_str) {
                assert_ascii(name, &format!("{context} discriminator property"));
            }
            if let Some(mapping) = discriminator.get("mapping").and_then(Value::as_object) {
                for (name, reference) in mapping {
                    assert_ascii(name, &format!("{context} discriminator value"));
                    assert_ascii(
                        reference.as_str().expect("mapping reference"),
                        &format!("{context} discriminator reference"),
                    );
                }
            }
        }
        if let Some(items) = schema.get("enum").and_then(Value::as_array) {
            for item in items.iter().filter_map(Value::as_str) {
                assert_ascii(item, &format!("{context} enum value"));
            }
        }
        if let Some(schemas) = schema.get("oneOf").and_then(Value::as_array) {
            for (index, item) in schemas.iter().enumerate() {
                schema_identifiers_are_ascii(item, &format!("{context} oneOf[{index}]"));
            }
        }
        if let Some(schemas) = schema.get("anyOf").and_then(Value::as_array) {
            for (index, item) in schemas.iter().enumerate() {
                schema_identifiers_are_ascii(item, &format!("{context} anyOf[{index}]"));
            }
        }
        if let Some(schemas) = schema.get("allOf").and_then(Value::as_array) {
            for (index, item) in schemas.iter().enumerate() {
                schema_identifiers_are_ascii(item, &format!("{context} allOf[{index}]"));
            }
        }
    }

    let spec = spec();
    for (path, item) in spec["paths"].as_object().expect("paths") {
        assert_ascii(path, "operation path");
        for (method, operation) in item.as_object().expect("path item") {
            if let Some(parameters) = operation.get("parameters").and_then(Value::as_array) {
                for parameter in parameters {
                    assert_ascii(
                        parameter["name"].as_str().expect("parameter name"),
                        &format!("{method} {path} parameter"),
                    );
                }
            }
        }
    }
    for (name, schema) in spec["components"]["schemas"].as_object().expect("schemas") {
        assert_ascii(name, "component schema name");
        schema_identifiers_are_ascii(schema, &format!("schema {name}"));
    }
}

/// API source identifiers mirror the stable ASCII wire schema names and properties.
#[test_log::test]
fn schema_namen_und_felder_bleiben_wire_kompatibel() {
    let spec = spec();
    let schemas = spec["components"]["schemas"].as_object().expect("schemas");

    for name in [
        "BenutzerLoeschungRequest",
        "AnlageAenderungRequest",
        "SchussAenderungRequest",
        "Laendercode",
        "ZielPfloecke",
        "ParkourOeffentlich",
        "AnlageOeffentlich",
    ] {
        assert!(schemas.contains_key(name), "missing wire schema {name}");
    }
    for retired in ["BenutzerLoeschenRequest", "BenutzerLöschungRequest"] {
        assert!(
            !schemas.contains_key(retired),
            "unexpected schema {retired}"
        );
    }

    let properties = |name: &str| {
        schemas[name]["properties"]
            .as_object()
            .unwrap_or_else(|| panic!("schema {name} has no properties"))
    };
    for (schema, field) in [
        ("BenutzerLoeschungRequest", "bestaetigt"),
        ("AnlageAdresse", "strasse"),
        ("AnlageAdresse", "laendercode"),
        ("Zielverwendung", "pfloecke"),
        ("Zielverwendung", "abstaende"),
        ("Parkour", "pfloecke"),
        ("ParkourKurz", "anzahlPfloecke"),
        ("Ziel", "pfloecke"),
        ("Ziel", "schuesse"),
        ("Zielwertung", "anzahlSchuesse"),
    ] {
        assert!(
            properties(schema).contains_key(field),
            "schema {schema} is missing wire property {field}"
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
