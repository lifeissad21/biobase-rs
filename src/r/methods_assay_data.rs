use super::data_classes::{AssayData, Matrix};

pub fn assay_data_valid_members(assay_data: &AssayData, required: &[&str]) -> Result<(), String> {
    let missing = required
        .iter()
        .filter(|name| !assay_data.contains_key(**name))
        .copied()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "assayData missing required member(s): {}",
            missing.join(", ")
        ))
    }
}

pub fn assay_data_dim(assay_data: &AssayData) -> Option<(usize, usize)> {
    assay_data
        .values()
        .next()
        .map(|matrix| (matrix.rows(), matrix.columns()))
}

pub fn assay_data_dims(assay_data: &AssayData) -> Vec<(String, usize, usize)> {
    assay_data
        .iter()
        .map(|(name, matrix)| (name.clone(), matrix.rows(), matrix.columns()))
        .collect()
}

pub fn sample_names(assay_data: &AssayData) -> Vec<Option<&[String]>> {
    assay_data
        .values()
        .map(Matrix::column_names)
        .collect::<Vec<_>>()
}

pub fn feature_names(assay_data: &AssayData) -> Vec<Option<&[String]>> {
    assay_data
        .values()
        .map(Matrix::row_names)
        .collect::<Vec<_>>()
}
