use yoda_taller::{
    settings::{ApplicationSettings, Settings, SwapiSettings},
    startup::Application,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO read settings from file.
    let settings = Settings {
        application: ApplicationSettings { port: 3000 },
        swapi: SwapiSettings {
            base_url: "url".to_string(),
            timeout_milliseconds: 111,
        },
    };
    Application::bind(settings)?.run().await
}
