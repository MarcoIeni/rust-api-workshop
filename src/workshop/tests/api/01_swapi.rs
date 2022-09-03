#[test]
fn run_the_swapi_docker_image() {
    //
    // ```
    // # Change directory to where you cloned https://github.com/MarcoIeni/swapi
    // cd swapi
    //
    // # Build the docker image
    // docker image build -t swapi .
    //
    // # Run the docker image
    // docker run  -p 9992:8000 -it swapi
    //
    // # You should see info about Luke Skywalker
    // curl http://127.0.0.1:9992/api/people/1/
    // ```
    let is_swapi_running = false;

    assert!(is_swapi_running);
}
