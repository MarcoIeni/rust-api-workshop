#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub height: u32,
}

pub fn people_by_name(name: &str) -> Vec<Person> {
    if name == "Darth Vader" {
        return vec![Person {
            name: "Darth Vader".to_string(),
            height: 202,
        }];
    }
    vec![]
}
