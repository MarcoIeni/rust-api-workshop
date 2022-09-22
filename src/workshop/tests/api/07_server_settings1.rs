//! Our crate is able to determine if Yoda is taller than a given person,
//! by also dealing with some corner cases.
//!
//! If we publish our crate as it is on crates.io, other developers
//! could use it as a library in their projects!
//!
//! However, our job is not done yet!
//! We want to expose this information to our user via an HTTP API.
//!
//! Before jumping into implementing the server, let's try to read
//! its settings from a yaml string.

use workshop::settings::{ApplicationSettings, Settings, SwapiSettings};

#[test]
fn settings_are_read_correctly() {
    let settings_yaml = r#"
application:
  port: 3000
swapi:
  base_url: "http://127.0.0.1:9992"
  timeout_milliseconds: 2000
"#;
    let actual_settings: Settings = serde_yaml::from_str(settings_yaml).unwrap();
    let expected_settings = Settings {
        application: ApplicationSettings { port: 3000 },
        swapi: SwapiSettings {
            base_url: "http://127.0.0.1:9992".to_string(),
            timeout_milliseconds: 2000,
        },
    };

    assert_eq!(expected_settings, actual_settings);
}
