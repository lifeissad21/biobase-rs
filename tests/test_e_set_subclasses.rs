mod common;

use biobase_rs::r::data_classes::{ESet, Miame};
use biobase_rs::r::methods_e_set::{
    annotation, assay_data_element, assay_data_element_replace, experiment_data, feature_data,
    pheno_data,
};

#[test]
fn test_eset_accessors_and_replacement() {
    let mut eset = ESet::new(common::assay_data_with_exprs());
    assert_eq!(assay_data_element(&eset, "exprs").unwrap().rows(), 2);
    assert_eq!(
        pheno_data(&eset).dim_labels(),
        ["sampleNames", "sampleColumns"]
    );
    assert_eq!(
        feature_data(&eset).dim_labels(),
        ["featureNames", "featureColumns"]
    );
    assert_eq!(experiment_data(&eset), &Miame::default());
    assert_eq!(annotation(&eset), "");

    let replacement = common::matrix(1, 1, &[7.0]);
    assay_data_element_replace(&mut eset, "exprs", replacement.clone());
    assert_eq!(assay_data_element(&eset, "exprs"), Some(&replacement));
}
