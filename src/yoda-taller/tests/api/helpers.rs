use std::time::Duration;

use crate::swapi_mock::SwapiMock;
use yoda_taller::{startup::Application, swapi::SwapiClient, YodaTaller};

pub struct TestApp {
    pub port: u16,
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
        let application = Application { port: 3000 };
        let _ = tokio::spawn(application.run());

        Self {
            swapi_client,
            yoda_taller,
            swapi_server,
            port: 3000,
        }
    }

    pub fn server_address(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}
