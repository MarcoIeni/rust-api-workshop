use std::time::Duration;

use swapi::SwapiClient;

pub struct TestApp {
    // pub address: String,
    // pub port: u16,
    pub swapi_client: SwapiClient,
}

impl TestApp {
    pub async fn spawn() -> Self {
        let swapi_client =
            SwapiClient::new("http://127.0.0.1:9992".to_string(), Duration::from_secs(20));
        Self { swapi_client }
    }
}
