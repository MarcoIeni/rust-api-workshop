use swapi::Person;

use crate::helpers::TestApp;
use swapi_mock::{empty_query_result, person_query_result};

#[tokio::test]
async fn darth_vader_is_tall() {
    let app = TestApp::spawn().await;
    let name = "Darth Vader";
    let expected_person = Person {
        name: name.to_string(),
        height: "202".to_string(),
    };
    let body = person_query_result(&expected_person);
    app.swapi_server.mock_people_query(name, body).await;
    let darth_vader = app.swapi_client.people_by_name(name).await.unwrap();
    assert_eq!(darth_vader, vec![expected_person]);
}

#[tokio::test]
async fn spock_is_not_found() {
    let app = TestApp::spawn().await;
    let body = empty_query_result();
    let name = "Spock";
    app.swapi_server.mock_people_query(name, body).await;
    let spock = app.swapi_client.people_by_name(name).await.unwrap();
    assert!(spock.is_empty());
}
