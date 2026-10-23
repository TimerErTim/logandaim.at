pub mod requests;

use std::sync::Arc;

use logandaim_core::App;

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
}
