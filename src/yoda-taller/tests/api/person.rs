use crate::helpers::swapi_mock::{empty_query_result, person_query_result};
use crate::helpers::{people, TestApp};

// Call swapi to assert how tall Luke is.
#[tokio::test]
async fn luke_is_tall() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let response_body = person_query_result(&luke);
    app.swapi_server
        .mock_people_query(&luke.name, response_body)
        .await;
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
