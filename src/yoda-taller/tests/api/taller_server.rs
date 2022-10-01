use {
    crate::helpers::{
        people, swapi_mock,
        test_app::{TestApp, SWAPI_TIMEOUT},
    },
    reqwest::StatusCode,
    std::time::Duration,
    yoda_taller::{
        server::taller_route::{ErrorBody, YodaTallerResponse},
        taller::YodaTallerOutcome,
    },
};

#[tokio::test]
async fn yoda_is_not_taller_than_luke() {
    let app = TestApp::spawn().await;

    let luke = people::luke();
    let query_body = swapi_mock::person_query_result(&luke);
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

#[tokio::test]
async fn yoda_is_not_taller_than_himself() {
    let app = TestApp::spawn().await;
    let yoda = people::yoda();
    let query_body = swapi_mock::person_query_result(&yoda);
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

#[tokio::test]
async fn yoda_is_taller_than_yaddle() {
    let app = TestApp::spawn().await;
    let yaddle = people::yaddle();
    let query_body = swapi_mock::person_query_result(&yaddle);
    app.swapi_server
        .mock_people_query(&yaddle.name, query_body)
        .await;
    let response = app.send_taller_req(&yaddle.name).await;
    assert_eq!(StatusCode::OK, response.status());

    let body = response.json().await.unwrap();
    assert_eq!(
        YodaTallerResponse {
            query: yaddle.name.clone(),
            result: YodaTallerOutcome {
                person: yaddle.name,
                taller: true
            }
        },
        body
    );
}

#[tokio::test]
async fn return_404_if_spock() {
    let app = TestApp::spawn().await;
    let name = "Spock";

    let body = swapi_mock::empty_query_result();
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

#[tokio::test]
async fn return_404_if_unknown_height() {
    let app = TestApp::spawn().await;

    // Arvel height is unknown.
    let arvel = people::arvel();

    let query_body = swapi_mock::person_query_result(&arvel);
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

#[tokio::test]
async fn return_500_if_timeout() {
    let app = TestApp::spawn().await;

    let luke = people::luke();
    let query_body = swapi_mock::person_query_result(&luke);

    let delay = SWAPI_TIMEOUT + Duration::from_secs(1);
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
