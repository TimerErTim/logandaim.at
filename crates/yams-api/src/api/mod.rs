pub mod requests;

use std::sync::Arc;

use yams_core::App;

#[derive(Clone)]
pub struct YamsAppApi {
    app: Arc<App>,
}

impl YamsAppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }
}
