use swapi::Person;

use crate::helpers::TestApp;

#[tokio::test]
async fn darth_vader_is_tall() {
    let app = TestApp::spawn().await;
    let darth_vader = app
        .swapi_client
        .people_by_name("Darth Vader")
        .await
        .unwrap();
    assert_eq!(
        darth_vader,
        vec![Person {
            name: "Darth Vader".to_string(),
            height: "202".to_string(),
        }]
    );
}

#[tokio::test]
async fn spock_is_not_found() {
    let app = TestApp::spawn().await;
    let spock = app.swapi_client.people_by_name("Spock").await.unwrap();
    assert!(spock.is_empty());
}
