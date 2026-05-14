use super::data_classes::{AnnotatedDataFrame, AssayData, ESet, Miame};
use super::versions_class::Versions;

pub trait ClassVersion {
    fn class_version(&self) -> Option<&Versions> {
        None
    }
}

pub trait HasAssayData {
    fn assay_data(&self) -> &AssayData;
}

pub trait HasExperimentData {
    fn experiment_data(&self) -> &Miame;
}

pub trait HasAnnotatedData {
    fn pheno_data(&self) -> &AnnotatedDataFrame;
    fn feature_data(&self) -> &AnnotatedDataFrame;
    fn protocol_data(&self) -> &AnnotatedDataFrame;
}

impl HasAssayData for ESet {
    fn assay_data(&self) -> &AssayData {
        &self.assay_data
    }
}

impl HasExperimentData for ESet {
    fn experiment_data(&self) -> &Miame {
        &self.experiment_data
    }
}

impl HasAnnotatedData for ESet {
    fn pheno_data(&self) -> &AnnotatedDataFrame {
        &self.pheno_data
    }

    fn feature_data(&self) -> &AnnotatedDataFrame {
        &self.feature_data
    }

    fn protocol_data(&self) -> &AnnotatedDataFrame {
        &self.protocol_data
    }
}
