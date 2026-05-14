use super::data_classes::{ExpressionSet, Matrix};

pub fn row_medians(matrix: &Matrix<f64>, na_rm: bool) -> Result<Vec<f64>, String> {
    let mut out = Vec::with_capacity(matrix.rows());
    for row in 0..matrix.rows() {
        let mut values = Vec::with_capacity(matrix.columns());
        for column in 0..matrix.columns() {
            let value = *matrix.get(row, column).unwrap();
            if value.is_nan() {
                if !na_rm {
                    values.clear();
                    values.push(f64::NAN);
                    break;
                }
            } else {
                values.push(value);
            }
        }
        if values.is_empty() || values.iter().any(|value| value.is_nan()) {
            out.push(f64::NAN);
            continue;
        }
        values.sort_by(f64::total_cmp);
        let mid = values.len() / 2;
        let median = if values.len() % 2 == 0 {
            (values[mid - 1] + values[mid]) / 2.0
        } else {
            values[mid]
        };
        out.push(median);
    }
    Ok(out)
}

pub fn row_q(matrix: &Matrix<f64>, which: usize) -> Result<Vec<f64>, String> {
    if which == 0 || which > matrix.columns() {
        return Err("'which' must be between 1 and the number of columns".to_string());
    }
    let mut out = Vec::with_capacity(matrix.rows());
    for row in 0..matrix.rows() {
        let mut values = Vec::with_capacity(matrix.columns());
        for column in 0..matrix.columns() {
            let value = *matrix.get(row, column).unwrap();
            if value.is_nan() {
                return Err("cannot handle missing values".to_string());
            }
            values.push(value);
        }
        values.sort_by(f64::total_cmp);
        out.push(values[which - 1]);
    }
    Ok(out)
}

pub fn row_min(matrix: &Matrix<f64>) -> Result<Vec<f64>, String> {
    row_q(matrix, 1)
}

pub fn row_max(matrix: &Matrix<f64>) -> Result<Vec<f64>, String> {
    row_q(matrix, matrix.columns())
}

pub fn expression_set_row_medians(
    expression_set: &ExpressionSet,
    na_rm: bool,
) -> Result<Vec<f64>, String> {
    let exprs = expression_set
        .exprs()
        .ok_or_else(|| "ExpressionSet has no exprs assay data".to_string())?;
    row_medians(exprs, na_rm)
}
