//! We now have `SwapiMock`, a struct which exposes convenient methods
//! to mock the Swapi API.
//! However, tests have still some duplication, such as the setup of
//! `SwapiMock` and a `SwapiClient`.
//!
//! Let's do one last iteration and create a new struct `TestApp` that
//! initializes both `SwapiMock` and `SwapiClient`
use {
    crate::helpers::{
        people,
        swapi_mock::{empty_query_result, person_query_result},
        test_app::TestApp,
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
    let response_body: serde_json::Value = person_query_result(&luke);
    // Mock Luke.
    app.swapi_server
        .mock_people_query(&luke.name, response_body)
        .await;
    // Use the `SwapiClient` you have initialized in the `spawn` function.
    let people = app.swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke]);
}

// Spock isn't a Star Wars character.
#[tokio::test]
async fn spock_is_not_found() {
    let app = TestApp::spawn().await;
    let response_body = empty_query_result();
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
    let response_body = person_query_result(&luke);
    let delay = app.settings.swapi.timeout() + Duration::from_secs(1);
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
