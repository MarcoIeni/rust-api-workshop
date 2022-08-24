use std::time::Duration;

use crate::helpers::{
    people,
    swapi_mock::{empty_query_result, person_query_result},
};
use reqwest::StatusCode;
use yoda_taller::server::routes::YodaTallerResponse;

use crate::helpers::TestApp;

#[tokio::test]
async fn yoda_is_not_taller_than_himself() {
    let app = TestApp::spawn().await;
    let yoda = people::yoda();
    let query_body = person_query_result(&yoda);
    app.swapi_server
        .mock_people_query(&yoda.name, query_body)
        .await;
    let response = app.send_taller_req(&yoda.name).await;
    assert_eq!(StatusCode::OK, response.status());

    let body: YodaTallerResponse = response.json().await.unwrap();
    assert!(!body.taller);
}

#[tokio::test]
async fn luke_is_taller_than_yoda() {
    let app = TestApp::spawn().await;

    let luke = people::luke();
    let query_body = person_query_result(&luke);
    app.swapi_server
        .mock_people_query(&luke.name, query_body)
        .await;
    let response = app.send_taller_req(&luke.name).await;
    assert_eq!(StatusCode::OK, response.status());

    let _body: YodaTallerResponse = response.json().await.unwrap();
    // TODO assert!(body.taller);
}

#[tokio::test]
async fn return_500_if_timeout() {
    let app = TestApp::spawn().await;

    let luke = people::luke();
    let query_body = person_query_result(&luke);

    let delay = app.settings.swapi.timeout() + Duration::from_secs(1);
    app.swapi_server
        .mock_people_query_with_delay(&luke.name, query_body, delay)
        .await;
    let response = app.send_taller_req(&luke.name).await;
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
}

#[tokio::test]
async fn return_404_if_spock() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let response = app.send_taller_req(name).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn return_404_if_unknown_height() {
    let app = TestApp::spawn().await;

    // Arvel height is unknown.
    let arvel = people::arvel();

    let query_body = person_query_result(&arvel);
    app.swapi_server
        .mock_people_query(&arvel.name, query_body)
        .await;
    let response = app.send_taller_req(&arvel.name).await;

    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
