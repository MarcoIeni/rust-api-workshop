//! When stars are aligned, our server works!
//! However, what happens when something is wrong?

use {
    crate::helpers::{
        people,
        swapi_mock::{empty_query_result, person_query_result},
        test_app::TestApp,
    },
    reqwest::StatusCode,
    std::time::Duration,
    workshop::{
        server::routes::{ErrorBody, YodaTallerResponse},
        YodaTallerResult,
    },
};

/// Spock is not part of the Swapi database.
/// So, if a user asks if Yoda is taller than Spock,
/// our server should return a 404.
#[tokio::test]
async fn return_404_if_spock() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = empty_query_result();
    app.swapi_server.mock_people_query(name, body).await;
    let response = app.send_taller_req(name).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
    assert_eq!(
        ErrorBody {
            query: name.to_string(),
            error: "Person not found".to_string()
        },
        response.json().await.unwrap()
    );
}

/// Some Star Wars characters are mysterious.
/// Some of them might even have an unknown height. 👀
///
/// If Swapi returns an invalid height, such us `unknown`, or `abc`,
/// our server should return a 404. Think of it as "height now found".
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
    assert_eq!(
        ErrorBody {
            query: arvel.name,
            error: "Person's height is unknown".to_string()
        },
        response.json().await.unwrap()
    );
}

/// Most of the time the Swapi API works well, and returns a response quickly.
/// However, in backend development, things work until they don't.
///
/// What should we do if the Swapi API doesn't respons for a long period of time?
/// We can't wait forever. At some point we need to fail and communicate to our client
/// that something is wrong.
///
/// If the request is taking longer than the swapi timeout, return a 500 to our users.
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
    assert_eq!(
        ErrorBody {
            query: luke.name,
            error: "Unexpected error".to_string()
        },
        response.json().await.unwrap()
    );
}
