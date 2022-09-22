use workshop::settings::{ApplicationSettings, Settings, SwapiSettings};

// TODO bring this up of health_check
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
