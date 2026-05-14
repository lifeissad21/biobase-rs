use biobase_rs::r::methods_miame::{abstract_text, miame, pub_med_ids};

#[test]
fn test_miame_accessors_default_to_empty_values() {
    let metadata = miame();

    assert_eq!(abstract_text(&metadata), "");
    assert!(pub_med_ids(&metadata).is_empty());
}
