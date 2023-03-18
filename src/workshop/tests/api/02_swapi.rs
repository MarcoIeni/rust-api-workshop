use std::time::Duration;

// The swapi module doesn't exist yet:
// - create it under `src/workshop/src/swapi.rs`.
// - Add the `swapi` module to the `src/workshop/src/lib.rs` file.
// - Create the structs `SwapiClient` and `Person`
//   in `src/workshop/src/swapi.rs` to re-use them in the next exercises.
use workshop::swapi::{Person, SwapiClient};

/// As a general rule, you should focus on making the test compile and pass first by mocking your functions.
/// For example, for this exercise implement the `people_by_name` function
/// so that it returns Luke's `Person` struct, without doing any network call.
///
/// ## Hint 💡
/// This test doesn't have `todo!()`, so you don't have to edit it to make it pass.
#[tokio::test]
async fn retrieve_luke_height_from_swapi() {
    // Edit the base_url if you are not using the suggested one.
    // For example, if you are not using docker, this should be `https://swapi.dev`.
    let base_url = "http://127.0.0.1:9992";
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url.to_string(), timeout).unwrap();
    let luke = Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    };
    let people: Vec<Person> = swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke]);
}

/// We have the basic structure of the `SwapiClient` struct.
/// Now edit the `people_by_name` function: use `reqwest` to create the HTTP client
/// and retrieve R2-D2's height from the swapi API.
///
/// ## Useful resources 📚
/// - [Search query - Swapi docs](https://swapi.dev/documentation#search):
///   documentation of the API call you need to make.
/// - [reqwest](https://docs.rs/reqwest/): the Rust HTTP client library.
/// - [serde](https://serde.rs/): the library to deserialize JSON responses from swapi.
async fn retrieve_rd_d2_height_from_swapi() {
    let base_url = "http://127.0.0.1:9992";
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url.to_string(), timeout).unwrap();
    let luke = Person {
        name: "r2-d2".to_string(),
        height: "96".to_string(),
    };
    let people: Vec<Person> = swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke]);
}

/// Spock is not a Star Wars character, so the `people_by_name` function
/// should return an empty vector.
#[tokio::test]
async fn spock_is_not_found() {
    let base_url = "http://127.0.0.1:9992";
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url.to_string(), timeout).unwrap();
    let people: Vec<Person> = swapi_client.people_by_name("Spock").await.unwrap();
    assert!(people.is_empty());
}
