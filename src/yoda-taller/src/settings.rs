use std::time::Duration;

use anyhow::Context;

use crate::YodaTaller;

#[cfg(feature = "test_fixture")]
use crate::swapi::SwapiClient;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub swapi: SwapiSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct SwapiSettings {
    pub base_url: String,
    pub timeout_milliseconds: u64,
}

impl SwapiSettings {
    pub fn yoda_taller(&self) -> YodaTaller {
        let timeout_duration = Duration::from_millis(self.timeout_milliseconds);
        YodaTaller::new(self.base_url.clone(), timeout_duration)
    }

    #[cfg(feature = "test_fixture")]
    pub fn swapi_client(&self) -> SwapiClient {
        let timeout_duration = Duration::from_millis(self.timeout_milliseconds);
        SwapiClient::new(self.base_url.clone(), timeout_duration)
    }
}

impl Settings {
    pub fn read() -> anyhow::Result<Self> {
        let base_path =
            std::env::current_dir().context("Failed to determine the current directory")?;
        let config_file = base_path.join("config.yaml");
        let f = std::fs::File::open(config_file).context("cannot open config file")?;
        serde_yaml::from_reader(f).context("invalid config file format")
    }
}
