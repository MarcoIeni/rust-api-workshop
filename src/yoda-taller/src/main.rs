use anyhow::Context;
use yoda_taller::{server::startup::Application, settings::Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::read().context("cannot read settings")?;
    Application::bind(settings)?.run().await
}
