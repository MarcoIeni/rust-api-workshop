//! We have a `SwapiClient` to retrieve the height from Swapi.
//! However, our original goal was to compare the height
//! of the character with the height of Yoda.
//!
//! Create a new module called `YodaTaller` to compare the heights.

use {
    crate::helpers::{people, swapi_mock::person_query_result, test_app::TestApp},
    workshop::YodaTallerOutcome,
};

#[tokio::test]
async fn yoda_is_not_taller_than_luke() {
    let app = TestApp::spawn().await;
    let luke = people::luke();
    let body = person_query_result(&luke);
    app.swapi_server.mock_people_query(&luke.name, body).await;
    // Create a new field called `yoda_taller` in the `TestApp`.
    // This field is of a new type `YodaTaller`, which can be initialized
    // with a `new` function, in the same way as `SwapiClient`.
    // In this exercise we only care about the happy case, so it's fine to return `reqwest::Error`
    // as an error and to unwrap in your code.
    let is_yoda_taller: YodaTallerOutcome =
        app.yoda_taller.is_taller_than(&luke.name).await.unwrap();
    assert_eq!(
        YodaTallerOutcome {
            person: luke.name,
            taller: false
        },
        is_yoda_taller
    );
}

#[tokio::test]
async fn yoda_is_not_taller_than_himself() {
    let app = TestApp::spawn().await;
    // Create the function `yoda`, which returns this `Person`:
    // Person {
    //     name: "Yoda".to_string(),
    //     height: "66".to_string(),
    // }
    let yoda = people::yoda();
    let body = person_query_result(&yoda);
    app.swapi_server.mock_people_query(&yoda.name, body).await;
    let is_yoda_taller: YodaTallerOutcome =
        app.yoda_taller.is_taller_than(&yoda.name).await.unwrap();
    assert_eq!(
        YodaTallerOutcome {
            person: yoda.name,
            taller: false
        },
        is_yoda_taller
    );
}

#[tokio::test]
async fn yoda_is_taller_than_yaddle() {
    let app = TestApp::spawn().await;
    // Create the function `yaddle`, which returns this `Person`:
    // Person {
    //     name: "Yaddle".to_string(),
    //     height: "61".to_string(),
    // }
    let yaddle = people::yaddle();
    let body = person_query_result(&yaddle);
    app.swapi_server.mock_people_query(&yaddle.name, body).await;
    let is_yoda_taller = app.yoda_taller.is_taller_than(&yaddle.name).await.unwrap();
    assert_eq!(
        YodaTallerOutcome {
            person: yaddle.name,
            taller: true
        },
        is_yoda_taller
    );
}
