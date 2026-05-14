mod common;

use biobase_rs::r::data_classes::{AssayData, ESet, SnpSet};
use biobase_rs::r::methods_snp_set::{snp_call, snp_call_probability};

#[test]
fn test_snp_set_accessors_return_expected_assays() {
    let mut assay_data = AssayData::new();
    assay_data.insert("call".to_string(), common::matrix(1, 2, &[0.0, 1.0]));
    assay_data.insert(
        "callProbability".to_string(),
        common::matrix(1, 2, &[0.7, 0.9]),
    );
    let snp_set = SnpSet {
        base: ESet::new(assay_data),
    };

    assert_eq!(snp_call(&snp_set).unwrap().values(), &[0.0, 1.0]);
    assert_eq!(
        snp_call_probability(&snp_set).unwrap().values(),
        &[0.7, 0.9]
    );
}
