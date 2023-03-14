//! Let's clean tests up!
//!
//! ## Hint 💡
//! First, make all the tests compile, than work on the implementation
//! by making the test pass one by one.

use {
    crate::helpers::{
        people,
        swapi_mock::{self, SwapiMock},
    },
    std::time::Duration,
    workshop::swapi::SwapiClient,
};

/// Let's rewrite the basic test where we retrieve Luke's height.
/// This time by extracting some duplicate code from the test.
#[tokio::test]
async fn luke_is_tall() {
    let swapi_server: SwapiMock = SwapiMock::start().await;
    let luke = people::luke();
    // The `person_query_result` function generates the body of the response
    // taking `name` and `height` from the `Person` given as argument.
    let response_body: serde_json::Value = swapi_mock::person_query_result(&luke);
    // Mount a [Mock](https://docs.rs/wiremock/0.5.17/wiremock/struct.Mock.html)
    // to the `swapi_server` to return Luke's query result.
    swapi_server
        .mock_people_query(&luke.name, response_body)
        .await;

    // Implement the `uri()` function for `SwapiMock` to expose the
    // [uri](https://docs.rs/wiremock/0.5.17/wiremock/struct.MockServer.html#method.uri)
    // function.
    let base_url = swapi_server.uri();

    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout).unwrap();
    let people = swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke]);
}

// Let's also clean the test with Spock.
#[tokio::test]
async fn spock_is_not_found() {
    let swapi_server = SwapiMock::start().await;
    // The `empty_query_result` function generates the body of the response
    // you get when you query with a person that doesn't exist.
    let response_body: serde_json::Value = swapi_mock::empty_query_result();
    let name = "Spock";
    swapi_server.mock_people_query(name, response_body).await;

    let base_url = swapi_server.uri();
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout).unwrap();
    let people = swapi_client.people_by_name(name).await.unwrap();
    assert!(people.is_empty());
}

#[tokio::test]
async fn swapi_client_returns_timeout_error_if_timeout() {
    let swapi_server = SwapiMock::start().await;
    let luke = people::luke();
    let response_body = swapi_mock::person_query_result(&luke);
    let timeout = Duration::from_secs(2);
    let delay = timeout + Duration::from_secs(1);
    swapi_server
        .mock_people_query_with_delay(&luke.name, response_body, delay)
        .await;

    let base_url = swapi_server.uri();
    let swapi_client = SwapiClient::new(base_url, timeout).unwrap();
    let err = swapi_client.people_by_name(&luke.name).await.unwrap_err();
    assert!(err.is_timeout());
}
