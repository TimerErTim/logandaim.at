use poem::{Route, http::StatusCode, test::TestClient};
use serde_json::Value;
use yams_api::openapi_service;
use yams_core::App;
use yams_fakes::FakeUnitOfWorkProvider;

pub fn base_app() -> App {
    App::builder()
        .uow_provider(Box::new(FakeUnitOfWorkProvider::empty()))
        .build()
}

pub struct YamsApiTestClient {
    client: TestClient<Route>,
}

impl YamsApiTestClient {
    pub fn new(app: App) -> Self {
        let service = openapi_service(app, std::iter::empty::<String>());
        let route = Route::new().nest("/api", service);
        Self {
            client: TestClient::new(route),
        }
    }

    pub fn client(&self) -> &TestClient<Route> {
        &self.client
    }

    pub async fn get_json(&self, path: &str) -> (StatusCode, Value) {
        json_response(self.client.get(path).send().await).await
    }
}

async fn json_response(resp: poem::test::TestResponse) -> (StatusCode, Value) {
    let status = resp.0.status();
    let body = resp.0.into_body().into_string().await.unwrap_or_default();
    let value: Value = serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("invalid JSON for {status}: {err}; body={body}"));
    (status, value)
}

pub fn assert_status_ok(status: StatusCode) {
    assert_eq!(status, StatusCode::OK, "expected 200 OK, got {status}");
}

#[test_log::test(pollster::test)]
async fn health_returns_ok() {
    let client = YamsApiTestClient::new(base_app());
    let (status, body) = client.get_json("/api/health").await;
    assert_status_ok(status);
    assert_eq!(body, Value::String("OK".into()));
}
