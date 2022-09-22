//! Now, let's create our HTTP server!
//! We will start by exposing just a single endpoint:
//! `/health_check`, which returns a 200 Status Code, without body.
//!
//! This kind of endpoint is often used to check if the application is healthy
//! or if it needs to be restarted.

use workshop::{
    // - Create a `server` module as a folder.
    // - Create the `server/startup.rs` file.
    // - Declare the `startup` module in the `server/mod.rs` file.
    server::startup::Application,
    settings::{ApplicationSettings, Settings, SwapiSettings},
};

use reqwest::StatusCode;

#[tokio::test]
async fn health_check_works() {
    let settings = Settings {
        // With port 0, the operating system chooses a random port
        // among the free ones.
        application: ApplicationSettings { port: 0 },
        // Swapi settings are not important for this test.
        swapi: SwapiSettings {
            base_url: "http://127.0.0.1:9992".to_string(),
            timeout_milliseconds: 2000,
        },
    };

    // The `Application::bind` method calls the
    // [`TcpListener::bind`](https://doc.rust-lang.org/stable/std/net/struct.TcpListener.html#method.bind)
    // to bind to the address `0.0.0.0:port`.
    // The port needs to be read from the settings (in this case is 0).
    let application_bind = Application::bind(settings).unwrap();
    let port = application_bind.tcp_listener().local_addr().unwrap().port();
    let _app = tokio::spawn(application_bind.run());
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://localhost:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(StatusCode::OK, response.status());
    // There's no body in the response.
    assert_eq!(Some(0), response.content_length());
}
