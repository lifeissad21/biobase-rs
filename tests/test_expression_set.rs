mod common;

use biobase_rs::r::data_classes::Value;
use biobase_rs::r::methods_expression_set::{as_data_frame, expression_set, exprs, set_exprs};

#[test]
fn test_expression_set_constructor_and_exprs_accessors() {
    let exprs_matrix = common::named_exprs();
    let mut eset = expression_set(exprs_matrix.clone());

    assert_eq!(exprs(&eset), Some(&exprs_matrix));
    assert_eq!(as_data_frame(&eset).len(), 3);
    assert_eq!(
        as_data_frame(&eset)[0],
        vec![Value::Number(1.0), Value::Number(4.0)]
    );

    let replacement = common::matrix(1, 2, &[10.0, 20.0]);
    set_exprs(&mut eset, replacement.clone());
    assert_eq!(exprs(&eset), Some(&replacement));
}
