use {crate::helpers::test_app::TestApp, reqwest::StatusCode};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.server_address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(Some(0), response.content_length());
}
