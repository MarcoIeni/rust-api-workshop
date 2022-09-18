use {crate::taller::YodaTaller, anyhow::Context, std::time::Duration};

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
    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_milliseconds)
    }

    pub fn yoda_taller(&self) -> anyhow::Result<YodaTaller> {
        YodaTaller::new(self.base_url.clone(), self.timeout())
    }

    #[cfg(feature = "test_fixture")]
    pub fn swapi_client(&self) -> anyhow::Result<SwapiClient> {
        SwapiClient::new(self.base_url.clone(), self.timeout())
    }
}

impl Settings {
    pub fn read() -> anyhow::Result<Self> {
        let base_path =
            std::env::current_dir().context("Failed to determine the current directory")?;
        let config_file = base_path.join("config.yaml");
        let f = std::fs::File::open(&config_file)
            .with_context(|| format!("cannot open config file {:?}", config_file))?;
        serde_yaml::from_reader(f).context("invalid config file format")
    }
}
