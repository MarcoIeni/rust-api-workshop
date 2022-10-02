//! The happy case is done!
//! Now let's think about what could go wrong: how do we handle errors?
//!
//! Change the `is_taller_than` function to return
//! `Result<YodaTallerOutcome, YodaTallerError>`.
//! `YodaTallerError` is an enum that derives `thiserror::Error`.
//!
//! ## Useful resources 📚
//! - [thiserror](https://docs.rs/thiserror/1.0.35/thiserror/index.html)

use {
    crate::helpers::{
        people, swapi_mock,
        test_app::{TestApp, SWAPI_TIMEOUT},
    },
    std::time::Duration,
    workshop::taller::YodaTallerError,
};

/// What happens if swapi behaves in weird ways, for example by
/// returning invalid bytes in the response?
/// Our library should return [`YodaTallerError::UnexpectedError`].
///
/// Add an `UnexpectedError` variant to the `YodaTallerError` enum.
/// `UnexpectedError` should contain the original `reqwest::Error`
/// of the `people_by_name` function.
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
/// Our library should return [`YodaTallerError::UnexpectedError`].
#[tokio::test]
async fn return_timeout_error_if_timeout() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let body = swapi_mock::person_query_result(&luke);
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

/// What happens when the user looks for a person that doesn't exist?
/// Our server should return a 404 HTTP status code with an helpful message.
/// Therefore the `is_taller_than` method should clearly distinguish this
/// error case.
/// We will call this type of error [`YodaTallerError::PersonNotFound`].
#[tokio::test]
async fn cannot_compare_yoda_and_non_existing_person() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = swapi_mock::empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let is_taller_err = app.yoda_taller.is_taller_than(name).await.unwrap_err();
    assert!(matches!(is_taller_err, YodaTallerError::PersonNotFound));
}

/// What happens when the user looks for a person with an unknown height?
/// For example, try to use curl to retrieve the height of "Arvel Crynyd".
///
/// Again, the `is_taller_than` method should clearly distinguish this
/// error case.
/// We will use [`YodaTallerError::HeightNotFound`].
#[tokio::test]
async fn cannot_compare_yoda_and_person_with_invalid_height() {
    let app = TestApp::spawn().await;
    // Create the function `arvel`, which returns this `Person`:
    // Person {
    //     name: "Arvel Crynyd".to_string(),
    //     height: "unknown".to_string(),
    // }
    // As you can see, Arvel has an unknown height.
    // As Luke and Yoda, this person is present in Swapi.
    let arvel = people::arvel();
    let body = swapi_mock::person_query_result(&arvel);
    app.swapi_server.mock_people_query(&arvel.name, body).await;
    let is_taller_err = app
        .yoda_taller
        .is_taller_than(&arvel.name)
        .await
        .unwrap_err();
    assert!(matches!(is_taller_err, YodaTallerError::HeightNotFound));
}
