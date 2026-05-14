mod common;

use biobase_rs::r::methods_assay_data::{
    assay_data_dim, assay_data_dims, assay_data_valid_members, feature_names, sample_names,
};

#[test]
fn test_assay_data_required_members_and_dimensions() {
    let assay_data = common::assay_data_with_exprs();

    assert!(assay_data_valid_members(&assay_data, &["exprs"]).is_ok());
    assert!(assay_data_valid_members(&assay_data, &["exprs", "se.exprs"]).is_err());
    assert_eq!(assay_data_dim(&assay_data), Some((2, 3)));
    assert_eq!(
        assay_data_dims(&assay_data),
        vec![("exprs".to_string(), 2, 3)]
    );
    assert_eq!(
        sample_names(&assay_data)[0].unwrap(),
        ["sample_1", "sample_2", "sample_3"]
    );
    assert_eq!(feature_names(&assay_data)[0].unwrap(), ["gene_a", "gene_b"]);
}
