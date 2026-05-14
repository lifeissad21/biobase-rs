use super::data_classes::{AnnotatedDataFrame, ESet, Matrix, Miame};

pub fn assay_data_element<'a>(object: &'a ESet, name: &str) -> Option<&'a Matrix<f64>> {
    object.assay_data_element(name)
}

pub fn assay_data_element_replace(object: &mut ESet, name: impl Into<String>, value: Matrix<f64>) {
    object.assay_data.insert(name.into(), value);
}

pub fn pheno_data(object: &ESet) -> &AnnotatedDataFrame {
    &object.pheno_data
}

pub fn feature_data(object: &ESet) -> &AnnotatedDataFrame {
    &object.feature_data
}

pub fn experiment_data(object: &ESet) -> &Miame {
    &object.experiment_data
}

pub fn annotation(object: &ESet) -> &str {
    &object.annotation
}
