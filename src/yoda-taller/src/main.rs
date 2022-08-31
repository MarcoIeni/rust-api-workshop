use anyhow::Context;
use yoda_taller::{server::startup::Application, settings::Settings, traces};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    traces::init_prod_traces()?;
    let settings = Settings::read().context("cannot read settings")?;
    Application::bind(settings)?.run().await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
