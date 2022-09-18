//! We have successfully mocked the part of the Swapi API we use.
//! Now that we have our mock we don't need the Swapi docker image anymore,
//! so we can comment the module `swapi` in the `src/workshop/tests/api/main.rs`
//! file and stop the swapi docker image.
//!
//! Now, before implementing the next feature, let's look once again
//! at the file `03_swapi_mock.rs`.
//! As you can see, those tests contain a lot of setup code.
//! Let's clean it up before it becomes too messy!

// Create the `helpers` module, by adding the `helpers` folder and
// adding it to the `src/workshop/tests/api/main.rs` file.
use crate::helpers::people;
use workshop::swapi::Person;

/// We are going to use the Luke's example many times.
/// Let's save data of people we are going to use in an helper module.
#[test]
fn luke_height_is_correct() {
    assert_eq!(
        Person {
            name: "Luke Skywalker".to_string(),
            height: "172".to_string(),
        },
        people::luke()
    )
}
