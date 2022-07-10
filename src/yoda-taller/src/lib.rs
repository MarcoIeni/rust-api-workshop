use std::time::Duration;

use swapi::SwapiClient;

pub struct YodaTaller {
    swapi_client: SwapiClient,
}

impl YodaTaller {
    pub fn new(swapi_base_url: String, swapi_timeout: Duration) -> Self {
        Self {
            swapi_client: SwapiClient::new(swapi_base_url, swapi_timeout),
        }
    }

    pub fn is_taller_than(&self, name: &str) -> bool {
	false
    }
}
