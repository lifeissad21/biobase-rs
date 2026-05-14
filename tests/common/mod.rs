#![allow(dead_code)]

use std::collections::BTreeMap;

use biobase_rs::r::data_classes::{AssayData, Matrix};
use biobase_rs::r::versioned_classes::Versioned;
use biobase_rs::r::versions_class::Versions;

pub fn matrix(rows: usize, columns: usize, values: &[f64]) -> Matrix<f64> {
    Matrix::new(rows, columns, values.to_vec()).unwrap()
}

pub fn named_exprs() -> Matrix<f64> {
    let mut exprs = matrix(2, 3, &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    exprs
        .set_dimnames(
            Some(vec!["gene_a".to_string(), "gene_b".to_string()]),
            Some(vec![
                "sample_1".to_string(),
                "sample_2".to_string(),
                "sample_3".to_string(),
            ]),
        )
        .unwrap();
    exprs
}

pub fn assay_data_with_exprs() -> AssayData {
    BTreeMap::from([("exprs".to_string(), named_exprs())])
}

pub fn versioned_fixture() -> Versioned {
    Versioned::new(Versions::from_pairs([("eSet", "1.3.0"), ("ExpressionSet", "1.0.0")]).unwrap())
}
