use {
    anyhow::Context,
    yoda_taller::{server::startup::Application, settings::Settings, traces},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    traces::init_prod_traces()?;

    let settings = settings()?;

    Application::bind(settings)?.run().await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

fn settings() -> anyhow::Result<Settings> {
    let base_path = std::env::current_dir().context("Failed to determine the current directory")?;
    let config_file = base_path.join("config.yaml");
    let settings = Settings::read(&config_file).context("cannot read settings")?;
    Ok(settings)
}
