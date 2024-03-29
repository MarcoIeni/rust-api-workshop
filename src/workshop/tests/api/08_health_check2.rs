//! You have created an HTTP server in Rust using `axum`, congrats! 👏
//! Now, let's clean our test suite, by integrating our `Application`
//! into the `TestApp`:
//! - Move the `Application::bind` function in `TestApp::spawn`.
//! - save the `port` in a new field of the `TestApp` struct.
//! - run the app with `tokio::spawn` in `TestApp::spawn`.

use {crate::helpers::test_app::TestApp, reqwest::StatusCode};

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;

    let client = reqwest::Client::new();
    let response = client
        // The `server_address` function returns the address of the application server.
        // You should run the server from the `TestApp`, by using a random port
        // chosen by the operating system.
        .get(&format!("{}/health_check", &app.server_address()))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(StatusCode::OK, response.status());
    // There's no body in the response.
    assert_eq!(Some(0), response.content_length());
}

// We now have a server running!
// Let's try to run our app:
// - uncomment the code in `src/workshop/src/main.rs`
// - run your app with `cargo run -p workshop
// - ping your endpoint: `curl -v 127.0.0.1:3000/health_check`
