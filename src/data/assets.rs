pub const GENE_COVARIATE_RDA: &[u8] = include_bytes!("geneCovariate.rda");
pub const REPORTER_RDA: &[u8] = include_bytes!("reporter.rda");
pub const SAMPLE_EXPRESSION_SET_RDA: &[u8] = include_bytes!("sample.ExpressionSet.rda");
pub const SAMPLE_MULTI_SET_RDA: &[u8] = include_bytes!("sample.MultiSet.rda");
pub const SED_RDA: &[u8] = include_bytes!("seD.rda");
pub const SW_RDA: &[u8] = include_bytes!("SW.rda");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataAsset {
    GeneCovariate,
    Reporter,
    SampleExpressionSet,
    SampleMultiSet,
    SeD,
    Sw,
}

impl DataAsset {
    pub const ALL: [DataAsset; 6] = [
        DataAsset::GeneCovariate,
        DataAsset::Reporter,
        DataAsset::SampleExpressionSet,
        DataAsset::SampleMultiSet,
        DataAsset::SeD,
        DataAsset::Sw,
    ];

    pub fn file_name(self) -> &'static str {
        match self {
            DataAsset::GeneCovariate => "geneCovariate.rda",
            DataAsset::Reporter => "reporter.rda",
            DataAsset::SampleExpressionSet => "sample.ExpressionSet.rda",
            DataAsset::SampleMultiSet => "sample.MultiSet.rda",
            DataAsset::SeD => "seD.rda",
            DataAsset::Sw => "SW.rda",
        }
    }

    pub fn bytes(self) -> &'static [u8] {
        match self {
            DataAsset::GeneCovariate => GENE_COVARIATE_RDA,
            DataAsset::Reporter => REPORTER_RDA,
            DataAsset::SampleExpressionSet => SAMPLE_EXPRESSION_SET_RDA,
            DataAsset::SampleMultiSet => SAMPLE_MULTI_SET_RDA,
            DataAsset::SeD => SED_RDA,
            DataAsset::Sw => SW_RDA,
        }
    }
}
