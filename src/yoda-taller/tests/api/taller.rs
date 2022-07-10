use swapi::Person;
use swapi_mock::{empty_query_result, person_query_result};
use yoda_taller::YodaTallerError;

use crate::helpers::TestApp;

#[tokio::test]
async fn yoda_is_not_taller_than_himself() {
    let app = TestApp::spawn().await;
    let name = "Yoda";

    let yoda_mock = Person {
        name: name.to_string(),
        height: "66".to_string(),
    };
    let body = person_query_result(&yoda_mock);
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller = app.yoda_taller_client.is_taller_than(name).await.unwrap();
    assert!(!is_taller);
}

#[tokio::test]
async fn cannot_compare_yoda_and_spock() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller = app
        .yoda_taller_client
        .is_taller_than(name)
        .await
        .unwrap_err();
    assert_eq!(is_taller, YodaTallerError::PersonNotFound);
}
