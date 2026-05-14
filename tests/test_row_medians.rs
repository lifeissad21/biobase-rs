mod common;

use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::row_op_methods::{row_medians, row_q};

fn assert_float_vec_equal(left: &[f64], right: &[f64]) {
    assert_eq!(left.len(), right.len());
    for (idx, (left, right)) in left.iter().zip(right.iter()).enumerate() {
        if left.is_nan() || right.is_nan() {
            assert!(
                left.is_nan() && right.is_nan(),
                "NaN mismatch at index {idx}: {left:?} vs {right:?}"
            );
        } else {
            assert!(
                (left - right).abs() < 1e-12,
                "value mismatch at index {idx}: {left:?} vs {right:?}"
            );
        }
    }
}

fn reference_row_medians(matrix: &Matrix<f64>, na_rm: bool) -> Vec<f64> {
    (0..matrix.rows())
        .map(|row| {
            let mut values = (0..matrix.columns())
                .filter_map(|column| {
                    let value = *matrix.get(row, column).unwrap();
                    if value.is_nan() && na_rm {
                        None
                    } else {
                        Some(value)
                    }
                })
                .collect::<Vec<_>>();
            if values.is_empty() || values.iter().any(|value| value.is_nan()) {
                return f64::NAN;
            }
            values.sort_by(f64::total_cmp);
            let mid = values.len() / 2;
            if values.len() % 2 == 0 {
                (values[mid - 1] + values[mid]) / 2.0
            } else {
                values[mid]
            }
        })
        .collect()
}

fn row_medians_via_row_q(matrix: &Matrix<f64>) -> Vec<f64> {
    let columns = matrix.columns();
    let half = (columns + 1) / 2;
    if columns % 2 == 1 {
        row_q(matrix, half).unwrap()
    } else {
        let lower = row_q(matrix, half).unwrap();
        let upper = row_q(matrix, half + 1).unwrap();
        lower
            .iter()
            .zip(upper.iter())
            .map(|(lower, upper)| (lower + upper) / 2.0)
            .collect()
    }
}

#[test]
fn row_medians_match_reference_median_without_missing_values() {
    let cases = [
        common::matrix(3, 3, &[1.0, 9.0, 4.0, 2.0, 8.0, 5.0, 3.0, 7.0, 6.0]),
        common::matrix(2, 4, &[4.0, 10.0, 1.0, 8.0, 7.0, 6.0, 2.0, 12.0]),
        common::matrix(1, 5, &[5.0, 1.0, 9.0, 3.0, 7.0]),
    ];

    for matrix in cases {
        let current = row_medians(&matrix, false).unwrap();
        let expected = reference_row_medians(&matrix, false);
        assert_float_vec_equal(&current, &expected);
        assert_float_vec_equal(&current, &row_medians_via_row_q(&matrix));
    }
}

#[test]
fn row_medians_match_reference_median_with_missing_values_removed() {
    let cases = [
        common::matrix(
            3,
            3,
            &[1.0, f64::NAN, 4.0, 2.0, 8.0, f64::NAN, 3.0, 7.0, 6.0],
        ),
        common::matrix(2, 4, &[f64::NAN, 10.0, 1.0, 8.0, 7.0, f64::NAN, 2.0, 12.0]),
        common::matrix(2, 2, &[f64::NAN, 1.0, f64::NAN, 3.0]),
    ];

    for matrix in cases {
        let current = row_medians(&matrix, true).unwrap();
        let expected = reference_row_medians(&matrix, true);
        assert_float_vec_equal(&current, &expected);
    }
}

#[test]
fn row_medians_preserve_missing_values_when_na_rm_is_false() {
    let matrix = common::matrix(2, 3, &[1.0, 4.0, f64::NAN, 5.0, 3.0, 6.0]);
    let current = row_medians(&matrix, false).unwrap();

    assert!(current[0].is_nan());
    assert_eq!(current[1], 5.0);
}
