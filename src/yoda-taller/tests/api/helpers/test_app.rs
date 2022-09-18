use {
    super::{swapi_mock::SwapiMock, test_traces::init_test_traces},
    std::time::Duration,
    yoda_taller::{
        server::startup::Application,
        settings::{ApplicationSettings, Settings, SwapiSettings},
        swapi::SwapiClient,
        taller::YodaTaller,
    },
};

pub const SWAPI_TIMEOUT: Duration = Duration::from_secs(5);

pub struct TestApp {
    pub port: u16,
    pub swapi_client: SwapiClient,
    pub yoda_taller: YodaTaller,
    pub swapi_server: SwapiMock,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn spawn() -> Self {
        init_test_traces();
        let swapi_server = SwapiMock::start().await;

        let settings = Settings {
            application: ApplicationSettings { port: 0 },
            swapi: SwapiSettings {
                base_url: swapi_server.uri(),
                timeout_milliseconds: SWAPI_TIMEOUT.as_millis().try_into().unwrap(),
            },
        };
        let yoda_taller = settings.swapi.yoda_taller().unwrap();
        let swapi_client = settings.swapi.swapi_client().unwrap();
        let application_bind = Application::bind(settings).unwrap();
        let port = application_bind.tcp_listener().local_addr().unwrap().port();

        let _app = tokio::spawn(application_bind.run());
        let api_client = reqwest::Client::new();

        Self {
            swapi_client,
            yoda_taller,
            swapi_server,
            api_client,
            port,
        }
    }

    pub fn server_address(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub async fn send_taller_req(&self, name: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/taller/{name}", &self.server_address()))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
