use yoda_taller::swapi::Person;

use crate::helpers::swapi_mock::{empty_query_result, person_query_result};
use crate::helpers::TestApp;

// Call swapi to assert how tall darth vader is.
#[tokio::test]
async fn darth_vader_is_tall() {
    let app = TestApp::spawn().await;
    let name = "Darth Vader";
    let darth_vader = Person {
        name: name.to_string(),
        height: "202".to_string(),
    };
    let response_body = person_query_result(&darth_vader);
    app.swapi_server
        .mock_people_query(name, response_body)
        .await;
    let people = app.swapi_client.people_by_name(name).await.unwrap();
    assert_eq!(people, vec![darth_vader]);
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
