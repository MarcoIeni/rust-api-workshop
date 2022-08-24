//! Some characters used for testings.

use yoda_taller::swapi::Person;

/// Yoda himself.
pub fn yoda() -> Person {
    Person {
        name: "Yoda".to_string(),
        height: "66".to_string(),
    }
}

/// A character taller than Yoda.
pub fn luke() -> Person {
    Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    }
}

/// A character with unkown height.
pub fn arvel() -> Person {
    Person {
        name: "Arvel Crynyd".to_string(),
        height: "unknown".to_string(),
    }
}
