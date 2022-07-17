use std::time::Duration;

use crate::swapi_mock::SwapiMock;
use yoda_taller::{swapi::SwapiClient, YodaTaller};

pub struct TestApp {
    pub swapi_client: SwapiClient,
    pub yoda_taller: YodaTaller,
    pub swapi_server: SwapiMock,
}

impl TestApp {
    pub async fn spawn() -> Self {
        // Launch a mock server to stand in for Postmark's API
        let swapi_server = SwapiMock::start().await;

        let swapi_client = SwapiClient::new(swapi_server.uri(), Duration::from_secs(20));
        let yoda_taller = YodaTaller::new(swapi_server.uri(), Duration::from_secs(20));
        Self {
            swapi_client,
            yoda_taller,
            swapi_server,
        }
    }
}
