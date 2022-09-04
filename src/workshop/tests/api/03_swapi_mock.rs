//! You are able to retrieve the height from Swapi, great!
//! However a docker image of the service you are working with
//! is not always available.
//! In these cases you can write a mock, i.e. a piece of software
//! that mimics like the service you are calling.
//! Let's practice writing a mock using the
//! [wiremock](https://docs.rs/wiremock/) library!
//!
//! 💡 This test should pass even if you stop the swapi container!
#[tokio::test]
fn retrieve_luke_height_from_swapi_mock() {
    let luke = Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    };

    // Start a `MockServer` and mock the GET request you do in the `SwapiClient`.
    todo!();

    let base_url = todo!();
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout);
    let people: Vec<Person> = swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke])
}
