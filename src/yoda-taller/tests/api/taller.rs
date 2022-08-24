use std::time::Duration;

use crate::helpers::swapi_mock::{empty_query_result, person_query_result};
use yoda_taller::swapi::Person;
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
    let is_taller = app.yoda_taller.is_taller_than(name).await.unwrap();
    assert!(!is_taller);
}

#[tokio::test]
async fn luke_is_taller_than_yoda() {
    let app = TestApp::spawn().await;
    let name = "Luke Skywalker";

    let luke_mock = Person {
        name: name.to_string(),
        height: "172".to_string(),
    };
    let body = person_query_result(&luke_mock);
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller = app.yoda_taller.is_taller_than(name).await.unwrap();
    assert!(!is_taller);
}

#[tokio::test]
async fn yaddle_is_shorter_than_yoda() {
    let app = TestApp::spawn().await;
    let name = "Yaddle";

    let luke_mock = Person {
        name: name.to_string(),
        height: "61".to_string(),
    };
    let body = person_query_result(&luke_mock);
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller = app.yoda_taller.is_taller_than(name).await.unwrap();
    assert!(is_taller);
}

#[tokio::test]
async fn cannot_compare_yoda_and_spock() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller_err = app.yoda_taller.is_taller_than(name).await.unwrap_err();
    assert!(matches!(is_taller_err, YodaTallerError::HeightNotFound(_)));
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

#[ignore = "timeout takes too much"]
#[tokio::test]
async fn return_timeout_error_if_timeout() {
    let app = TestApp::spawn().await;
    let name = "Luke Skywalker";

    let luke_mock = Person {
        name: name.to_string(),
        height: "172".to_string(),
    };
    let body = person_query_result(&luke_mock);
    let delay = app.settings.swapi.timeout() + Duration::from_secs(1);
    app.swapi_server
        .mock_people_query_with_delay(name, body, delay)
        .await;
    let err = app.yoda_taller.is_taller_than(name).await.unwrap_err();
    match err {
        YodaTallerError::UnexpectedError(e) => assert!(e.is_timeout()),
        _ => panic!("unexpected error"),
    }
}
