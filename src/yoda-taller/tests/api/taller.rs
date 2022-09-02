use std::time::Duration;

use crate::helpers::swapi_mock::{empty_query_result, person_query_result};
use yoda_taller::{YodaTallerError, YodaTallerResponse};

use crate::helpers::{people, TestApp};

#[tokio::test]
async fn yoda_is_not_taller_than_himself() {
    let app = TestApp::spawn().await;
    let yoda = people::yoda();
    let body = person_query_result(&yoda);
    app.swapi_server.mock_people_query(&yoda.name, body).await;
    let is_yoda_taller = app.yoda_taller.is_taller_than(&yoda.name).await.unwrap();
    assert_eq!(
        YodaTallerResponse {
            person: yoda.name,
            taller: false
        },
        is_yoda_taller
    );
}

#[tokio::test]
async fn yoda_is_shorter_than_luke() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let body = person_query_result(&luke);
    app.swapi_server.mock_people_query(&luke.name, body).await;
    let is_yoda_taller = app.yoda_taller.is_taller_than(&luke.name).await.unwrap();
    assert_eq!(
        YodaTallerResponse {
            person: luke.name,
            taller: false
        },
        is_yoda_taller
    );
}

#[tokio::test]
async fn yoda_is_taller_than_yaddle() {
    let app = TestApp::spawn().await;
    let yaddle = people::yaddle();
    let body = person_query_result(&yaddle);
    app.swapi_server.mock_people_query(&yaddle.name, body).await;
    let is_yoda_taller = app.yoda_taller.is_taller_than(&yaddle.name).await.unwrap();
    assert_eq!(
        YodaTallerResponse {
            person: yaddle.name,
            taller: true
        },
        is_yoda_taller
    );
}

#[tokio::test]
async fn cannot_compare_yoda_and_non_existing_person() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller_err = app.yoda_taller.is_taller_than(name).await.unwrap_err();
    assert!(matches!(is_taller_err, YodaTallerError::PersonNotFound(_)));
}

#[tokio::test]
async fn cannot_compare_yoda_and_person_with_invalid_height() {
    let app = TestApp::spawn().await;
    let arvel = people::arvel();
    let body = person_query_result(&arvel);
    app.swapi_server.mock_people_query(&arvel.name, body).await;
    let is_taller_err = app
        .yoda_taller
        .is_taller_than(&arvel.name)
        .await
        .unwrap_err();
    assert!(matches!(
        is_taller_err,
        YodaTallerError::HeightNotFound { .. }
    ));
}

#[tokio::test]
async fn return_decode_error_if_invalid_response() {
    let app = TestApp::spawn().await;
    let name = "Luke Skywalker";

    let body = serde_json::json!( {
        "invalid": "response"
    });
    app.swapi_server.mock_people_query(name, body).await;
    let err = app.yoda_taller.is_taller_than(name).await.unwrap_err();
    match err {
        YodaTallerError::UnexpectedError(e) => assert!(e.is_decode()),
        _ => panic!("unexpected error"),
    }
}

#[tokio::test]
async fn return_timeout_error_if_timeout() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let body = person_query_result(&luke);
    let delay = app.settings.swapi.timeout() + Duration::from_secs(1);
    app.swapi_server
        .mock_people_query_with_delay(&luke.name, body, delay)
        .await;
    let err = app
        .yoda_taller
        .is_taller_than(&luke.name)
        .await
        .unwrap_err();
    match err {
        YodaTallerError::UnexpectedError(e) => assert!(e.is_timeout()),
        _ => panic!("unexpected error"),
    }
}
