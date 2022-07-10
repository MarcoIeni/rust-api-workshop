use swapi::Person;
use swapi_mock::person_query_result;

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
    let is_taller = app.yoda_taller_client.is_taller_than(name).await.unwrap();
    assert!(!is_taller);
}
