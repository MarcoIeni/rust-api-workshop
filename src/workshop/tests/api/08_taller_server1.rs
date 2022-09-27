//! We have written the first endpoint of our server!
//! Now let's design the response of the `/taller/` endpoint.

use workshop::{server::taller_route::YodaTallerResponse, taller::YodaTallerOutcome};

/// The body of the `/taller/` endpoint should tell whether
/// Yoda is taller then the given characters or not.
#[tokio::test]
async fn yoda_taller_response_serializes_to_expected_json() {
    // This is an example of the json we want to return in the body.
    // It contains the following fields:
    // - query: the string inserted by the user in the url.
    //   E.g. in the request `curl 127.0.0.1:3000/taller/luke` the query
    // is `luke`
    // - person: the name of the first person found in Swapi
    // - taller: whether Yoda is taller than the person found or not.
    //
    // ## Note 📝
    // If you are bothered about this ugly indentation, check out the
    // [indoc](https://docs.rs/indoc/latest/indoc/) crate 😉
    let expected_response = r#"{
  "query": "luke",
  "person": "Luke Skywalker",
  "taller": false
}"#;

    // We already have a struct that contains both the `person` and `taller` fields.
    // Create a new struct caller YodaTallerResponse, and use
    // [flatten](https://serde.rs/attr-flatten.html)
    // to embed `YodaTallerOutcome` inside.
    let response = YodaTallerResponse {
        query: "luke".to_string(),
        result: YodaTallerOutcome {
            person: "Luke Skywalker".to_string(),
            taller: false,
        },
    };
    let json_response = serde_json::to_string_pretty(&response).unwrap();
    assert_eq!(expected_response, json_response);
}
