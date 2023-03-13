/// Calling an external service from your test suite is not ideal, because:
/// - If the service is down, your test suite fails.
/// - You could incur in rate limits.
/// - Your tests require an internet connection to pass.
///
/// To avoid calling directly https://swapi.dev/,
/// run the swapi docker image locally.
///
/// - Option 1: Run the pre-built docker image (faster)
///
///   ```sh
///   # The port 9992 must be free. You can use a different port if 9992 isn't free.
///   docker run -p 9992:8000 -it ghcr.io/marcoieni/swapi
///   ```
///
/// - Option 2: Build the docker image yourself (slower)
///
///   ```sh
///   git clone https://github.com/MarcoIeni/swapi
///   cd swapi
///
///   # Build the docker image
///   docker image build -t swapi .
///
///   # Run the docker image.
///   # The port 9992 must be free. You can use a different port if 9992 isn't free.
///   docker run -p 9992:8000 -it swapi
///   ```
///
/// You should be able to see info about Luke Skywalker by running:
/// ```sh
/// curl http://127.0.0.1:9992/api/people/1/
/// ```
///
/// Now you can also see Swapi docs by opening
/// `http://127.0.0.1:9992/` in your browser.
#[test]
fn run_the_swapi_docker_image() {
    let is_swapi_running = todo!();

    assert!(is_swapi_running);
}
