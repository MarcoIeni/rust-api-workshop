use std::time::Duration;

use swapi::SwapiClient;
use swapi_mock::SwapiMock;

pub struct TestApp {
    pub swapi_client: SwapiClient,
    pub swapi_server: SwapiMock,
}

impl TestApp {
    pub async fn spawn() -> Self {
        // Launch a mock server to stand in for Postmark's API
        let swapi_server = SwapiMock::start().await;

        let swapi_client = SwapiClient::new(swapi_server.uri(), Duration::from_secs(20));

        Self {
            swapi_client,
            swapi_server,
        }
    }
}
