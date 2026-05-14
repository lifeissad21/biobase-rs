mod common;

use biobase_rs::r::methods_versioned_class::{is_valid_version, versioned_is_current};
use biobase_rs::r::versions_class::{Version, Versions};

#[test]
fn test_versioned_class_current_and_valid_version_checks() {
    let object = common::versioned_fixture();
    let current = Versions::from_pairs([("eSet", "1.3"), ("ExpressionSet", "1")]).unwrap();

    assert!(versioned_is_current(&object, &current));
    assert!(is_valid_version(
        &object,
        "ExpressionSet",
        &Version::parse("1.0").unwrap()
    ));
    assert!(!is_valid_version(
        &object,
        "ExpressionSet",
        &Version::parse("2.0.0").unwrap()
    ));
}
