//! The happy case is done!
//! Now let's think about what could go wrong: how do we handle errors?

use {
    crate::helpers::{
        people,
        swapi_mock::{empty_query_result, person_query_result},
        test_app::{TestApp, SWAPI_TIMEOUT},
    },
};

/// What happens when the user looks for a person that doesn't exist?
/// Our server should return a 404 HTTP status code with an helpful message.
/// Therefore the `is_taller_than` method should clearly distinguish this
/// error case.
/// We will use `YodaTallerError::PersonNotFound`.
///
/// ## Hint 💡
/// Use the [thiserror](https://docs.rs/thiserror/) library to
/// model the error.
#[tokio::test]
async fn cannot_compare_yoda_and_non_existing_person() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller_err = app.yoda_taller.is_taller_than(name).await.unwrap_err();
    assert!(matches!(is_taller_err, YodaTallerError::PersonNotFound));
}

/// What happens when the user looks for a person with an unknown height?
/// For example, try to use curl to retrieve the height of "Arvel Crynyd".
///
/// Again, the `is_taller_than` method should clearly distinguish this
/// error case.
/// We will use `YodaTallerError::HeightNotFound`.
#[tokio::test]
async fn cannot_compare_yoda_and_person_with_invalid_height() {
    let app = TestApp::spawn().await;
    // Person {
    //     name: "Arvel Crynyd".to_string(),
    //     height: "unknown".to_string(),
    // }
    let arvel = people::arvel();
    let body = person_query_result(&arvel);
    app.swapi_server.mock_people_query(&arvel.name, body).await;
    let is_taller_err = app
        .yoda_taller
        .is_taller_than(&arvel.name)
        .await
        .unwrap_err();
    assert!(matches!(is_taller_err, YodaTallerError::HeightNotFound));
}

/// What happens if swapi behaves in weird ways, for example by
/// returning invalid bytes in the response?
/// Our library should return `YodaTallerError::UnexpectedError`.
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

/// What happens if swapi takes too long to answer?
/// Our library should return `YodaTallerError::UnexpectedError`.
#[tokio::test]
async fn return_timeout_error_if_timeout() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let body = person_query_result(&luke);
    let delay = SWAPI_TIMEOUT + Duration::from_secs(1);
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
