//! We have written the first endpoint of our server!
//! Now let's write the `/taller/` endpoint, that tells whether
//! Yoda is taller then the given characters or not.

use {
    crate::helpers::{people, swapi_mock::person_query_result, test_app::TestApp},
    reqwest::StatusCode,
    workshop::{server::routes::YodaTallerResponse, YodaTallerOutcome},
};

/// As usual, let's start with the happy case.
/// Let's ask our sever if Yoda is taller than Luke.
/// We expect a response with a status code 200 and the body:
/// ```json
/// {
///   "query": "Luke Skywalker",
///   "person": "Luke Skywalker",
///   "taller": false
/// }
/// ```
#[tokio::test]
async fn yoda_is_not_taller_than_luke() {
    let app = TestApp::spawn().await;

    let luke = people::luke();
    let query_body = person_query_result(&luke);
    app.swapi_server
        .mock_people_query(&luke.name, query_body)
        .await;
    let response = app.send_taller_req(&luke.name).await;
    assert_eq!(StatusCode::OK, response.status());

    let body = response.json().await.unwrap();
    assert_eq!(
        YodaTallerResponse {
            query: luke.name.clone(),
            result: YodaTallerOutcome {
                person: luke.name,
                taller: false
            }
        },
        body
    );
}

/// Let's ask to our server is yoda is taller than himself.
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

    let body = response.json().await.unwrap();
    assert_eq!(
        YodaTallerResponse {
            query: yoda.name.clone(),
            result: YodaTallerOutcome {
                person: yoda.name,
                taller: false
            }
        },
        body
    );
}
