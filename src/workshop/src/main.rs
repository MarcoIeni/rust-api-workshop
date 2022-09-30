#![allow(unused_imports)]
use {
    anyhow::Context,
    workshop::{server::startup::Application, settings::Settings},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Uncomment these lines after exercise `07_server_settings2.rs`
    /*
    let settings = {
        let base_path =
            std::env::current_dir().context("Failed to determine the current directory")?;
            let config_file = base_path.join("config.yaml");
            Settings::read(&config_file).context("cannot read settings")?
    };
    */

    // Uncomment these lines after exercise `08_health_check1`
    /*
    Application::bind(settings)?.run().await?;
    */

    Ok(())
}
