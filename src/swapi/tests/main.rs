use swapi::Person;

#[test]
fn darth_vader_is_tall() {
    let darth_vader = swapi::people_by_name("Darth Vader");
    assert_eq!(
        darth_vader,
        vec![Person {
            name: "Darth Vader".to_string(),
            height: 202,
        }]
    );
}
