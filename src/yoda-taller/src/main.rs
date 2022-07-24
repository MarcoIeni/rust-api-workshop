use yoda_taller::{
    settings::{ApplicationSettings, Settings, SwapiSettings},
    startup::Application,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings {
        application: ApplicationSettings { port: 3000 },
        swapi: SwapiSettings {
            base_url: "url".to_string(),
            timeout_milliseconds: 111,
        },
    };
    Application::build(settings)?.run().await
}
