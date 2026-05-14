#[derive(Debug, Clone, PartialEq)]
pub struct MatchPointResult {
    /// One-based index of the nearest neighbor, matching R's result.
    pub index: Vec<Option<usize>>,
    pub distance: Vec<f64>,
}

pub fn matchpt(x: &[Vec<f64>], y: Option<&[Vec<f64>]>) -> Result<MatchPointResult, String> {
    let ncol = validate_matrix(x, "x")?;
    let y = y.unwrap_or(x);
    let y_ncol = validate_matrix(y, "y")?;
    if ncol != y_ncol {
        return Err("x and y must have the same number of columns".to_string());
    }

    let no_y = std::ptr::eq(x, y);
    let mut index = Vec::with_capacity(x.len());
    let mut distance = Vec::with_capacity(x.len());

    for (i, x_row) in x.iter().enumerate() {
        let mut best_index = None;
        let mut best_distance = f64::INFINITY;

        for (j, y_row) in y.iter().enumerate() {
            if no_y && i == j {
                continue;
            }

            let dist = x_row
                .iter()
                .zip(y_row.iter())
                .map(|(left, right)| {
                    let tmp = left - right;
                    tmp * tmp
                })
                .sum::<f64>();

            if dist < best_distance {
                best_index = Some(j + 1);
                best_distance = dist;
            }
        }

        index.push(best_index);
        distance.push(best_distance.sqrt());
    }

    Ok(MatchPointResult { index, distance })
}

fn validate_matrix(matrix: &[Vec<f64>], name: &str) -> Result<usize, String> {
    let Some(first) = matrix.first() else {
        return Ok(0);
    };
    let ncol = first.len();
    if matrix.iter().any(|row| row.len() != ncol) {
        return Err(format!("{name} must be a rectangular numeric matrix"));
    }
    Ok(ncol)
}
