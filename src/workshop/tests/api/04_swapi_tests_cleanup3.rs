//! We now have `SwapiMock`, a struct which exposes convenient methods
//! to mock the Swapi API.
//! However, tests have still some duplication, such as the setup of
//! `SwapiMock` and a `SwapiClient`.
//!
//! Let's do one last iteration and create a new struct `TestApp` that
//! initializes both `SwapiMock` and `SwapiClient`.

use {
    crate::helpers::{
        people,
        swapi_mock,
        // Create the `test_app` module.
        // Inside this module, create:
        // - `TestApp` struct
        // - `SWAPI_TIMEOUT` constant, which is a `Duration` of 2 seconds.
        test_app::{TestApp, SWAPI_TIMEOUT},
    },
    std::time::Duration,
};

/// Let's rewrite the basic test where we retrieve Luke's height.
/// This time by extracting some duplicate code from the test.
#[tokio::test]
async fn luke_is_tall() {
    // The `spawn` function is used to initialize the `SwapiClient` and start the mock.
    let app = TestApp::spawn().await;
    let luke = people::luke();
    // Take the body of the response starting from name and height.
    let response_body: serde_json::Value = swapi_mock::person_query_result(&luke);
    // Add a `swapi_server` field of type `SwapiMock` to `TestApp`.
    // The `swapi_server` field should be initialized in the `spawn` method.
    app.swapi_server
        .mock_people_query(&luke.name, response_body)
        .await;
    // Add a `swapi_client` field of type `SwapiClient` to `TestApp`.
    // The `swapi_client` field should be initialized in the `spawn` method.
    let people = app.swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke]);
}

// Spock isn't a Star Wars character.
#[tokio::test]
async fn spock_is_not_found() {
    let app = TestApp::spawn().await;
    let response_body = swapi_mock::empty_query_result();
    let name = "Spock";
    app.swapi_server
        .mock_people_query(name, response_body)
        .await;
    let people = app.swapi_client.people_by_name(name).await.unwrap();
    assert!(people.is_empty());
}

#[tokio::test]
async fn swapi_client_returns_timeout_error_if_timeout() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let response_body = swapi_mock::person_query_result(&luke);
    // To avoid repeating the timeout of the `SwapiClient` for every test,
    // we configure the `SwapiClient` timeout once, in the `spawn` function.
    // However, some tests need to read this timeout.
    // So let's create the `SWAPI_TIMEOUT` constant.
    let delay = SWAPI_TIMEOUT + Duration::from_secs(1);
    app.swapi_server
        .mock_people_query_with_delay(&luke.name, response_body, delay)
        .await;
    let err = app
        .swapi_client
        .people_by_name(&luke.name)
        .await
        .unwrap_err();
    assert!(err.is_timeout());
}
