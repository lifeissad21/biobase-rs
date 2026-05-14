use std::collections::BTreeMap;

use biobase_rs::C::envir::{SlotObject, lc_prefix, row_q, unsafe_set_slot};
use biobase_rs::C::matchpt::matchpt;
use biobase_rs::C::rinit::registered_call_methods;
use biobase_rs::C::row_medians::{MatrixView, row_medians};
use biobase_rs::C::row_medians_type_template::{row_medians_integer, row_medians_real};
use biobase_rs::C::sublist_extract::{SimplifiedValue, Value, sublist_extract};

#[test]
fn matchpt_finds_nearest_neighbors_with_r_style_indices() {
    let x = vec![vec![0.0, 0.0], vec![10.0, 0.0], vec![1.0, 0.0]];
    let result = matchpt(&x, None).unwrap();
    assert_eq!(result.index, vec![Some(3), Some(3), Some(1)]);
    assert_eq!(result.distance, vec![1.0, 9.0, 1.0]);

    let y = vec![vec![2.0, 0.0], vec![20.0, 0.0]];
    let result = matchpt(&x, Some(&y)).unwrap();
    assert_eq!(result.index, vec![Some(1), Some(1), Some(1)]);
    assert_eq!(result.distance, vec![2.0, 8.0, 1.0]);
}

#[test]
fn row_median_translation_preserves_column_major_indexing() {
    let values = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let matrix = MatrixView::new(&values, 2, 3).unwrap();
    assert_eq!(row_medians(matrix, false, true), vec![2.0, 5.0]);
    assert_eq!(row_q(matrix, 2).unwrap(), vec![2.0, 5.0]);

    assert_eq!(
        row_medians_real(&values, 2, 3, false, true).unwrap(),
        vec![2.0, 5.0]
    );
    assert_eq!(
        row_medians_integer(&[1, 4, 2, 5, 3, 6], 2, 3, false, true).unwrap(),
        vec![2.0, 5.0]
    );
}

#[test]
fn envir_helpers_cover_lc_prefix_and_slot_setting() {
    assert_eq!(lc_prefix(&["abCd", "abxy"], false).unwrap(), "ab");
    assert_eq!(lc_prefix(&["abCd", "ABxy"], true).unwrap(), "AB");

    let mut object = SlotObject::new();
    unsafe_set_slot(&mut object, "title", "example".to_string());
    assert_eq!(object.slot("title").unwrap(), "example");
}

#[test]
fn sublist_extract_simplifies_scalar_values() {
    let mut first = BTreeMap::new();
    first.insert("id".to_string(), Value::Integer(1));
    let mut second = BTreeMap::new();
    second.insert("id".to_string(), Value::Integer(2));

    let result = sublist_extract(&[first.clone(), second.clone()], "id", true, true).unwrap();
    assert_eq!(result, SimplifiedValue::Integer(vec![1, 2]));

    let result = sublist_extract(&[first, second], "id", false, true).unwrap();
    assert_eq!(
        result,
        SimplifiedValue::List(vec![Value::Integer(1), Value::Integer(2)])
    );
}

#[test]
fn rinit_lists_translated_call_methods() {
    let methods = registered_call_methods();
    assert_eq!(methods.len(), 5);
    assert!(methods.iter().any(|method| method.name == "rowMedians"));
    assert!(
        methods
            .iter()
            .any(|method| method.name == "sublist_extract")
    );
}
