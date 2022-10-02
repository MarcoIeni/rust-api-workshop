//! Let's create a function to read the settings from a file.
//! It will be mainly useful for our main, i.e.
//! `src/workshop/src/main.rs`.

use {
    std::{io::Write, path::Path},
    workshop::settings::{ApplicationSettings, Settings, SwapiSettings},
};

#[tokio::test]
async fn application_uses_passed_settings() {
    let settings_yaml = r#"
application:
  port: 3000
swapi:
  base_url: "http://127.0.0.1:9992"
  timeout_milliseconds: 2000
"#;

    // create a file that will be deleted after the test.
    let mut config_file = tempfile::NamedTempFile::new().unwrap();
    config_file.write_all(settings_yaml.as_bytes()).unwrap();
    let config_path: &Path = config_file.path();
    // The `read` function reads the settings from a yaml file.
    // This functions can fail in two ways:
    // - The file doesn't exist
    // - The file contains invalid yaml
    //
    // If we needed to distinguish the failure type, we could use the `thiserror` crate
    // as we have done before.
    // However, we don't care *why* the `read` function fails, because it's an
    // unrecoverable error, that we just need to report to the developer that
    // is trying to run the server.
    // In these cases, you can use the
    // [anyhow](https://docs.rs/anyhow/latest) crate.
    let actual_settings: Settings = Settings::read(config_path).unwrap();

    let expected_settings = Settings {
        application: ApplicationSettings { port: 3000 },
        swapi: SwapiSettings {
            base_url: "http://127.0.0.1:9992".to_string(),
            timeout_milliseconds: 2000,
        },
    };
    assert_eq!(actual_settings, expected_settings);
}
