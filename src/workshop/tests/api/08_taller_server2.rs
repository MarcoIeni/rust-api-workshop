//! We have the response of the `/taller/` endpoint.
//! Now we are ready to build the endpoint!
//! This endpoint tells whether
//! Yoda is taller then the given characters or not.

use {
    crate::helpers::{people, swapi_mock::person_query_result, test_app::TestApp},
    reqwest::StatusCode,
    workshop::{server::taller_route::YodaTallerResponse, taller::YodaTallerOutcome},
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

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/taller/{}", &app.server_address(), &luke.name))
        .send()
        .await
        .expect("Failed to execute request.");

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
