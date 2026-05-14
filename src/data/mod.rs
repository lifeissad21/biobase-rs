//! Native Rust accessors for the data shipped with Biobase's `data/` folder.

pub mod aa_map;
pub mod assets;
pub mod gene_cov;
pub mod gene_data;

pub use aa_map::{
    AA_MAP, AminoAcid, SideChainProperty, amino_acid_by_one_letter, amino_acid_by_three_letter,
};
pub use assets::{
    DataAsset, GENE_COVARIATE_RDA, REPORTER_RDA, SAMPLE_EXPRESSION_SET_RDA, SAMPLE_MULTI_SET_RDA,
    SED_RDA, SW_RDA,
};
pub use gene_cov::{GENE_COV, GeneCovariate, gene_covariate};
pub use gene_data::{
    GENE_DATA, GENE_DATA_COLUMN_NAMES, GENE_DATA_COLUMNS, GENE_DATA_ROW_NAMES, GENE_DATA_ROWS,
    GENE_DATA_VALUE_COUNT, GENE_DATA_VALUES, GeneDataMatrix, gene_data,
};
