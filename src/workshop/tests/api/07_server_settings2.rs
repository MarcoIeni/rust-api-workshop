
use workshop::{
    // - Create a `server` module as a folder.
    // - Create the `server/startup.rs` file.
    // - Declare the `startup` module in the `server/mod.rs` file.
    server::startup::Application,
    settings::{ApplicationSettings, Settings, SwapiSettings},
};

use reqwest::StatusCode;

/// Let's start by creating an HTTP server that exposes a single endpoint:
/// `/health_check`, which returns a 200 Status Code, without body.
///
/// This kind of endpoint is often used to check if the application is healthy
/// or if it needs to be restarted.
#[tokio::test]
async fn health_check_works() {
    let settings = Settings {
        // With port 0, the operating system chooses a random port
        // among the free ones.
        application: ApplicationSettings { port: 0 },
        // swapi settings are not important for this test
        swapi: SwapiSettings {
            base_url: "http://127.0.0.1:9992".to_string(),
            timeout_milliseconds: 2000,
        },
    };

    // the `Application::bind` method calls the `TcpListener::bind`
    let app = Application::bind(settings).unwrap();
    assert_eq!(application.settings, settings);
}
