use biobase_rs::data::{
    DataAsset, GeneCovariate, SideChainProperty, amino_acid_by_one_letter,
    amino_acid_by_three_letter, gene_covariate, gene_data,
};

#[test]
fn amino_acid_lookup_matches_r_data() {
    let aspartic_acid = amino_acid_by_one_letter('D').unwrap();
    assert_eq!(aspartic_acid.name, "aspartic.acid");
    assert_eq!(aspartic_acid.three_letter, "asp");
    assert_eq!(aspartic_acid.side_chain_property, SideChainProperty::Polar);
    assert_eq!(aspartic_acid.hydrophilic, Some(true));
    assert_eq!(aspartic_acid.acidic, Some(true));

    assert_eq!(amino_acid_by_three_letter("VAL").unwrap().one_letter, 'V');
}

#[test]
fn gene_covariates_match_r_data() {
    assert_eq!(
        gene_covariate('A'),
        Some(&GeneCovariate {
            row_name: 'A',
            cov1: 1,
            cov2: 1,
            cov3: 1,
        })
    );
    assert_eq!(
        gene_covariate('Z'),
        Some(&GeneCovariate {
            row_name: 'Z',
            cov1: 2,
            cov2: 1,
            cov3: 3,
        })
    );
}

#[test]
fn gene_data_matrix_has_r_dimensions_and_column_major_indexing() {
    let matrix = gene_data();
    assert_eq!(matrix.rows(), 500);
    assert_eq!(matrix.columns(), 26);
    assert_eq!(matrix.values().len(), 13_000);
    assert_eq!(matrix.row_names()[0], "AFFX-MurIL2_at");
    assert_eq!(matrix.column_names()[0], "A");
    assert_eq!(matrix.column_names()[25], "Z");
    assert_eq!(matrix.get(0, 0), Some(192.742));
    assert_eq!(matrix.get_by_names("AFFX-MurIL2_at", "A"), Some(192.742));
}

#[test]
fn all_serialized_r_assets_are_embedded() {
    for asset in DataAsset::ALL {
        assert!(!asset.file_name().is_empty());
        assert!(!asset.bytes().is_empty());
    }
}
