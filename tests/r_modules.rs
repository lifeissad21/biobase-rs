use std::collections::BTreeMap;

use biobase_rs::r::data_classes::{ExpressionSet, Matrix, Value};
use biobase_rs::r::row_op_methods::{expression_set_row_medians, row_max, row_medians, row_min};
use biobase_rs::r::strings::{lc_prefix, lc_suffix, strbreak};
use biobase_rs::r::tools::{is_unique, reverse_split, sub_list_extract};
use biobase_rs::r::versions_class::{Version, Versions};

#[test]
fn versions_parse_and_compare_like_numeric_versions() {
    assert!(Version::parse("1.2.0").unwrap() == Version::parse("1.2").unwrap());
    assert!(Version::parse("1.3").unwrap() > Version::parse("1.2.9").unwrap());

    let versions = Versions::from_pairs([("eSet", "1.3.0"), ("ExpressionSet", "1.0.0")]).unwrap();
    assert_eq!(versions.get("eSet").unwrap().to_string(), "1.3.0");
    assert_eq!(versions.len(), 2);
}

#[test]
fn string_helpers_match_biobase_behaviour() {
    assert_eq!(lc_prefix(&["abcXYZ", "abc123"], false), "abc");
    assert_eq!(lc_suffix(&["123xyz", "ABCxyz"], false), "xyz");
    assert_eq!(strbreak("abcdef", 4, 2, "\n").unwrap(), "abcd\n  ef");
}

#[test]
fn row_ops_use_column_major_matrix_layout() {
    let matrix = Matrix::new(2, 3, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]).unwrap();
    assert_eq!(row_medians(&matrix, false).unwrap(), vec![2.0, 5.0]);
    assert_eq!(row_min(&matrix).unwrap(), vec![1.0, 4.0]);
    assert_eq!(row_max(&matrix).unwrap(), vec![3.0, 6.0]);

    let eset = ExpressionSet::from_exprs(matrix);
    assert_eq!(
        expression_set_row_medians(&eset, false).unwrap(),
        vec![2.0, 5.0]
    );
}

#[test]
fn tool_helpers_cover_r_list_utilities() {
    let mut split = BTreeMap::new();
    split.insert("a".to_string(), vec!["x".to_string(), "y".to_string()]);
    split.insert("b".to_string(), vec!["x".to_string()]);
    let reversed = reverse_split(&split);
    assert_eq!(reversed["x"], vec!["a".to_string(), "b".to_string()]);

    assert_eq!(is_unique(&[1, 2, 1, 3]), vec![false, true, false, true]);

    let mut first = BTreeMap::new();
    first.insert("id".to_string(), Value::Integer(1));
    let mut second = BTreeMap::new();
    second.insert("id".to_string(), Value::Integer(2));
    assert_eq!(
        sub_list_extract(&[first, second], "id").unwrap(),
        vec![Value::Integer(1), Value::Integer(2)]
    );
}
