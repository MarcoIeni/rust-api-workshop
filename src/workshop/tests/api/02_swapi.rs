use std::time::Duration;

// The swapi module doesn't exist yet, create it under
// `src/workshop/src/swapi.rs`.
// Then add the module to the `src/workshop/src/lib.rs` file.
use workshop::swapi::{Person, SwapiClient};

/// Query the swapi instance running in docker to retrieve
/// Luke's height.
/// You need to create the structs [`SwapiClient`] and [`Person`]
/// under the `src/workshop/src` directory to re-use them in the next exercises.
///
/// ## Useful resources 📚
/// - [Swapi docs](https://swapi.dev/documentation)
/// - [Rust HTTP client](https://docs.rs/reqwest/)
///
/// ## Hint 1 💡
/// Always focus on making the test compile and pass first by mocking your functions.
/// For example, `people_by_name` could just return the expected `Person` struct, without doing any network call at first.
/// Please make sure to use `reqwest` to create the HTTP client and retrieve
/// Luke's height before jumping to the next test.
///
/// ## Hint 2 💡
/// This test doesn't have `todo!()`, so you don't have to edit it to make it pass.
#[tokio::test]
async fn retrieve_luke_height_from_swapi() {
    let base_url = "http://127.0.0.1:9992";
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url.to_string(), timeout);
    let luke = Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    };
    let people: Vec<Person> = swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke])
}

/// Spock is not a Star Wars character, so the `people_by_name` function
/// should return an empty vector.
#[tokio::test]
async fn spock_is_not_found() {
    let base_url = "http://127.0.0.1:9992";
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout);
    let people: Vec<Person> = swapi_client.people_by_name("Spock").await.unwrap();
    assert!(people.is_empty());
}
