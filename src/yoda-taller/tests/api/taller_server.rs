use std::time::Duration;

use crate::helpers::swapi_mock::{empty_query_result, person_query_result};
use yoda_taller::{server::routes::YodaTallerResponse, swapi::Person};

use crate::helpers::TestApp;

#[tokio::test]
async fn yoda_is_not_taller_than_himself() {
    let app = TestApp::spawn().await;
    let name = "Yoda";

    let yoda_mock = Person {
        name: name.to_string(),
        height: "66".to_string(),
    };
    let query_body = person_query_result(&yoda_mock);
    app.swapi_server.mock_people_query(name, query_body).await;
    let response = app.send_taller_req(name).await;
    assert_eq!(200, response.status().as_u16());

    let body: YodaTallerResponse = response.json().await.unwrap();
    assert!(!body.taller);
}

#[tokio::test]
async fn return_500_if_timeout() {
    let app = TestApp::spawn().await;
    let name = "Luke Skywalker";

    let luke_mock = Person {
        name: name.to_string(),
        height: "172".to_string(),
    };
    let query_body = person_query_result(&luke_mock);

    let delay = app.settings.swapi.timeout() + Duration::from_secs(1);
    app.swapi_server
        .mock_people_query_with_delay(name, query_body, delay)
        .await;
    let response = app.send_taller_req(name).await;
    assert_eq!(500, response.status().as_u16());
}

#[tokio::test]
async fn return_404_if_spock() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let response = app.send_taller_req(name).await;
    assert_eq!(404, response.status().as_u16());
}
