use super::data_classes::{ExpressionSet, Matrix, Value};

pub fn expression_set(exprs: Matrix<f64>) -> ExpressionSet {
    ExpressionSet::from_exprs(exprs)
}

pub fn exprs(object: &ExpressionSet) -> Option<&Matrix<f64>> {
    object.exprs()
}

pub fn set_exprs(object: &mut ExpressionSet, value: Matrix<f64>) {
    object.base.assay_data.insert("exprs".to_string(), value);
}

pub fn as_data_frame(object: &ExpressionSet) -> Vec<Vec<Value>> {
    let Some(exprs) = object.exprs() else {
        return Vec::new();
    };
    let mut rows = Vec::with_capacity(exprs.columns());
    for column in 0..exprs.columns() {
        let mut row = Vec::with_capacity(exprs.rows());
        for feature in 0..exprs.rows() {
            row.push(Value::Number(*exprs.get(feature, column).unwrap()));
        }
        rows.push(row);
    }
    rows
}
