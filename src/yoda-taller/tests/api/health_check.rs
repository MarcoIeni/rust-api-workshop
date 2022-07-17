use crate::helpers::TestApp;

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
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
