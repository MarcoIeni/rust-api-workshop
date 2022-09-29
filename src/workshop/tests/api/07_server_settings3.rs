//! Let's use the settings to create our application!

use workshop::{
    // - Create a `server` module as a folder.
    // - Create the `server/startup.rs` file.
    // - Declare the `startup` module in the `server/mod.rs` file.
    server::startup::Application,
    settings::{ApplicationSettings, Settings, SwapiSettings},
};

#[tokio::test]
async fn application_uses_passed_settings() {
    let settings = Settings {
        application: ApplicationSettings { port: 0 },
        swapi: SwapiSettings {
            base_url: "http://127.0.0.1:9992".to_string(),
            timeout_milliseconds: 2000,
        },
    };

    // Create the `Application` type:
    // pub struct Application {
    //     pub settings: Settings,
    // }
    let app = Application::bind(settings.clone()).unwrap();
    assert_eq!(app.settings, settings);
}

// For now the bind application only assigns the settings.
// But don't worry. In the next test we will do the actual binding operation! 😎
