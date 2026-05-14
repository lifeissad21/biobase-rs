use super::row_medians::{MatrixView, row_medians};

pub fn row_medians_integer(
    values: &[i32],
    rows: usize,
    columns: usize,
    na_rm: bool,
    by_row: bool,
) -> Result<Vec<f64>, String> {
    Ok(row_medians(
        MatrixView::new(values, rows, columns)?,
        na_rm,
        by_row,
    ))
}

pub fn row_medians_real(
    values: &[f64],
    rows: usize,
    columns: usize,
    na_rm: bool,
    by_row: bool,
) -> Result<Vec<f64>, String> {
    Ok(row_medians(
        MatrixView::new(values, rows, columns)?,
        na_rm,
        by_row,
    ))
}
