#[derive(Debug, Clone, Copy)]
pub struct MatrixView<'a, T> {
    values: &'a [T],
    rows: usize,
    columns: usize,
}

impl<'a, T> MatrixView<'a, T> {
    pub fn new(values: &'a [T], rows: usize, columns: usize) -> Result<Self, String> {
        if values.len() != rows * columns {
            return Err(format!(
                "matrix dimensions require {} values, got {}",
                rows * columns,
                values.len()
            ));
        }
        Ok(Self {
            values,
            rows,
            columns,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        self.values.get(row + column * self.rows)
    }
}

pub fn row_medians<T>(matrix: MatrixView<'_, T>, na_rm: bool, by_row: bool) -> Vec<f64>
where
    T: Copy + IntoMedianValue,
{
    let nrow = if by_row { matrix.rows } else { matrix.columns };
    let ncol = if by_row { matrix.columns } else { matrix.rows };
    let mut out = Vec::with_capacity(nrow);

    for ii in 0..nrow {
        let mut row_data = Vec::with_capacity(ncol);
        let mut has_missing = false;
        for jj in 0..ncol {
            let value = if by_row {
                matrix.get(ii, jj)
            } else {
                matrix.get(jj, ii)
            }
            .expect("row and column are in range")
            .into_median_value();

            if value.is_nan() {
                if na_rm {
                    continue;
                }
                has_missing = true;
                break;
            }
            row_data.push(value);
        }

        if has_missing {
            out.push(f64::NAN);
        } else if row_data.is_empty() {
            out.push(f64::NAN);
        } else {
            out.push(median_unchecked(&mut row_data));
        }
    }

    out
}

fn median_unchecked(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    let mid = values.len() / 2;
    if values.len() % 2 == 1 {
        values[mid]
    } else {
        (values[mid - 1] + values[mid]) / 2.0
    }
}

pub trait IntoMedianValue {
    fn into_median_value(self) -> f64;
}

impl IntoMedianValue for f64 {
    fn into_median_value(self) -> f64 {
        self
    }
}

impl IntoMedianValue for i32 {
    fn into_median_value(self) -> f64 {
        if self == i32::MIN {
            f64::NAN
        } else {
            self as f64
        }
    }
}
