use std::collections::BTreeMap;

use super::row_medians::MatrixView;

pub fn row_q(matrix: MatrixView<'_, f64>, which: usize) -> Result<Vec<f64>, String> {
    if which == 0 || which > matrix.columns() {
        return Err(format!(
            "cannot calculate order statistic on object with {} columns",
            matrix.columns()
        ));
    }

    let mut out = Vec::with_capacity(matrix.rows());
    for row in 0..matrix.rows() {
        let mut row_data = Vec::with_capacity(matrix.columns());
        for column in 0..matrix.columns() {
            let value = *matrix.get(row, column).unwrap();
            if value.is_nan() {
                return Err("'imat' must not contain missing values".to_string());
            }
            row_data.push(value);
        }
        row_data.sort_by(f64::total_cmp);
        out.push(row_data[which - 1]);
    }
    Ok(out)
}

pub fn lc_prefix(values: &[impl AsRef<str>], ignore_case: bool) -> Result<String, String> {
    if values.len() < 2 {
        return Ok(values
            .first()
            .map(|value| value.as_ref().to_string())
            .unwrap_or_default());
    }

    let normalized = values
        .iter()
        .map(|value| {
            if ignore_case {
                value.as_ref().to_ascii_uppercase()
            } else {
                value.as_ref().to_string()
            }
        })
        .collect::<Vec<_>>();

    let min_len = normalized.iter().map(String::len).min().unwrap_or(0);
    let first = normalized[0].as_bytes();
    let mut end = 0;
    'outer: while end < min_len {
        let c = first[end];
        for value in &normalized {
            if value.as_bytes()[end] != c {
                break 'outer;
            }
        }
        end += 1;
    }
    Ok(normalized[0][..end].to_string())
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlotObject<T> {
    slots: BTreeMap<String, T>,
}

impl<T> SlotObject<T> {
    pub fn new() -> Self {
        Self {
            slots: BTreeMap::new(),
        }
    }

    pub fn slot(&self, name: &str) -> Option<&T> {
        self.slots.get(name)
    }

    pub fn slots(&self) -> &BTreeMap<String, T> {
        &self.slots
    }
}

pub fn unsafe_set_slot<T>(object: &mut SlotObject<T>, slot: impl Into<String>, value: T) {
    object.slots.insert(slot.into(), value);
}
