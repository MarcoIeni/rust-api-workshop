use {
    crate::helpers::{
        people,
        swapi_mock::{empty_query_result, person_query_result},
        test_app::{TestApp, SWAPI_TIMEOUT},
    },
    std::time::Duration,
};

// Call swapi to assert how tall Luke is.
#[tokio::test]
async fn luke_is_tall() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let response_body = person_query_result(&luke);
    app.swapi_server
        .mock_people_query(&luke.name, response_body)
        .await;
    let people = app.swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke]);
}

// Spock isn't a Star Wars character.
#[tokio::test]
async fn spock_is_not_found() {
    let app = TestApp::spawn().await;
    let response_body = empty_query_result();
    let name = "Spock";
    app.swapi_server
        .mock_people_query(name, response_body)
        .await;
    let people = app.swapi_client.people_by_name(name).await.unwrap();
    assert!(people.is_empty());
}

#[tokio::test]
async fn swapi_client_returns_timeout_error_if_timeout() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let response_body = person_query_result(&luke);
    let delay = SWAPI_TIMEOUT + Duration::from_secs(1);
    app.swapi_server
        .mock_people_query_with_delay(&luke.name, response_body, delay)
        .await;
    let err = app
        .swapi_client
        .people_by_name(&luke.name)
        .await
        .unwrap_err();
    assert!(err.is_timeout());
}
