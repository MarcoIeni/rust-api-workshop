//! You are able to retrieve the height from Swapi, great!
//! However a docker image of the service you are working with
//! is not always available.
//! In these cases you can write a mock, i.e. a piece of software
//! that mimics like the service you are calling.
//! Let's practice writing a mock using the
//! [wiremock](https://docs.rs/wiremock/) library!

use std::time::Duration;

use workshop::swapi::{Person, SwapiClient};

/// 💡 This test should pass even if you stop the swapi container!
#[tokio::test]
async fn retrieve_luke_height_from_swapi_mock() {
    let luke = Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    };

    // Start a [`MockServer`](https://docs.rs/wiremock/0.5.14/wiremock/struct.MockServer.html)
    // and mock the GET response you get in the `SwapiClient`.
    // You should return the response we have seen in the previous exercise
    // when looking for Luke: a 200 status code and Luke's name and height
    // in the `results` of the body.

    // Use the [uri](https://docs.rs/wiremock/0.5.14/wiremock/struct.MockServer.html#method.uri)
    // method to retrieve the base url.
    let base_url = todo!();
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout);
    let people: Vec<Person> = swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke])
}

/// Spock is not a Star Wars character, so the `people_by_name` function
/// should return an empty vector.
///
/// ## Hint 💡
/// Don't worry too much about copy pasting from the previous test.
/// We are going to clean in the next exercises.
#[tokio::test]
async fn spock_is_not_found_from_swapi_mock() {
    // Start a `MockServer` and mock the GET response you get in the `SwapiClient`.
    // You should return the response we have seen in the previous exercise
    // when looking for Spock: a 200 status code and an empty `results` in the body.
    todo!();

    let base_url = todo!();

    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout);
    let people: Vec<Person> = swapi_client.people_by_name("Spock").await.unwrap();
    assert!(people.is_empty());
}

/// Mocks allow to simulate edge cases of your dependencies.
/// - What happens if Swapi doesn't respond?
/// - Does your application return the right error?
///
/// Use wiremock to simulate a delay from Swapi.
///
/// 💡 Have a look at [ResponseTemplate](https://docs.rs/wiremock/0.5.14/wiremock/struct.ResponseTemplate.html#method.set_delay)
#[tokio::test]
async fn swapi_client_returns_timeout_error_if_timeout() {
    let luke = Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    };

    // Start a `MockServer` and mock the GET request you do in the `SwapiClient`.
    todo!();

    let base_url = todo!();
    // For this test to pass, you need to edit the `SwapiClient` to
    // take into account this timeout.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout);
    let err: reqwest::Error = swapi_client.people_by_name(&luke.name).await.unwrap_err();
    assert!(err.is_timeout());
}
