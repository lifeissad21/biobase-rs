use super::data_classes::{AnnotatedDataFrame, DataFrame, Matrix};

pub fn annotated_data_frame(data: DataFrame, var_metadata: DataFrame) -> AnnotatedDataFrame {
    AnnotatedDataFrame::new(
        data,
        var_metadata,
        ["rowNames".to_string(), "columnNames".to_string()],
    )
}

pub fn annotated_data_frame_from_matrix<T>(matrix: &Matrix<T>, by_row: bool) -> AnnotatedDataFrame {
    let labels = if by_row {
        ["featureNames".to_string(), "featureColumns".to_string()]
    } else {
        ["sampleNames".to_string(), "sampleColumns".to_string()]
    };
    let _ = matrix;
    AnnotatedDataFrame::new(DataFrame::new(), DataFrame::new(), labels)
}

pub fn p_data(object: &AnnotatedDataFrame) -> &DataFrame {
    object.data()
}

pub fn var_metadata(object: &AnnotatedDataFrame) -> &DataFrame {
    object.var_metadata()
}

pub fn var_labels(object: &AnnotatedDataFrame) -> Vec<&str> {
    object.var_labels()
}
