//! Our crate is able to determine if Yoda is taller than a given person,
//! by also dealing with corner cases.
//!
//! If we publish our crate as it is on crates.io, other developers
//! could use it as a library in their projects!
//!
//! However, our job is not done yet!
//! We want to expose this information to our user via an HTTP API.

use crate::helpers::TestApp;

/// Let's start by creating a single HTTP server that exposes a single endpoint:
/// `/health_check`, which simply returns a 200.
///
/// This kind of endpoint is often used to check if the application is healthy
/// or if it needs to be restarted.
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // The `server_address` function returns the address of the application server.
        // You should run the server from the `TestApp`, by using a random port
        // chosen by the operating system.
        .get(&format!("{}/health_check", &app.server_address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    // There's no body in the response.
    assert_eq!(Some(0), response.content_length());
}
