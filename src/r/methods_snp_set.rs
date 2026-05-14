use super::data_classes::{Matrix, SnpSet};

pub fn snp_call(object: &SnpSet) -> Option<&Matrix<f64>> {
    object.snp_call()
}

pub fn snp_call_probability(object: &SnpSet) -> Option<&Matrix<f64>> {
    object.snp_call_probability()
}
