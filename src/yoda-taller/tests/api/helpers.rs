use std::time::Duration;

use swapi_mock::SwapiMock;
use yoda_taller::YodaTaller;

pub struct TestApp {
    pub yoda_taller_client: YodaTaller,
    pub swapi_server: SwapiMock,
}

impl TestApp {
    pub async fn spawn() -> Self {
        // Launch a mock server to stand in for Postmark's API
        let swapi_server = SwapiMock::start().await;

        Self {
            yoda_taller_client: YodaTaller::new(swapi_server.uri(), Duration::from_secs(20)),
            swapi_server,
        }
    }
}
