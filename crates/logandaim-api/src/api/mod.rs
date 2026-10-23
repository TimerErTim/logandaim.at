pub mod requests;

use std::sync::Arc;

use logandaim_core::{
    App, ResultReport,
    service::{SchussRegistrieren, SchussRegistrierenFehler, WertungBerechnen, WertungFehler},
};

use crate::schema::{Schuss, Wertung, schema_schuss_from_domain, schema_wertung_from_domain};

/// Typed facade between HTTP and use cases.
///
/// Everything here is a thin translation: request → use case → schema. No business rules.
/// Use-case errors pass through unchanged; `HttpStatusMapping` in `errors/http_status.rs`
/// turns them into status codes.
#[derive(Clone)]
pub struct LogandaimAppApi {
    app: Arc<App>,
}

impl LogandaimAppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }

    pub async fn schuss_registrieren(
        &self,
        erstellung: requests::SchussErstellung,
    ) -> ResultReport<Schuss, SchussRegistrierenFehler> {
        let schuss = self
            .app
            .execute(SchussRegistrieren::from(erstellung))
            .await?;
        Ok(schema_schuss_from_domain(schuss))
    }

    pub async fn wertung_berechnen(
        &self,
        anfrage: requests::WertungsAnfrage,
    ) -> ResultReport<Wertung, WertungFehler> {
        let ergebnis = self.app.execute(WertungBerechnen::from(anfrage)).await?;
        Ok(schema_wertung_from_domain(ergebnis))
    }
}
