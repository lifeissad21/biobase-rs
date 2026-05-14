mod common;

use biobase_rs::r::methods_expression_set::{expression_set, exprs};
use biobase_rs::r::row_op_methods::{expression_set_row_medians, row_max, row_min, row_q};

#[test]
fn test_es_apply_equivalent_row_operations_on_expression_set() {
    let eset = expression_set(common::named_exprs());

    assert_eq!(
        expression_set_row_medians(&eset, false).unwrap(),
        vec![2.0, 5.0]
    );
    assert_eq!(row_q(exprs(&eset).unwrap(), 2).unwrap(), vec![2.0, 5.0]);
    assert_eq!(row_min(exprs(&eset).unwrap()).unwrap(), vec![1.0, 4.0]);
    assert_eq!(row_max(exprs(&eset).unwrap()).unwrap(), vec![3.0, 6.0]);
}
