mod common;

use std::collections::BTreeMap;

use biobase_rs::r::data_classes::Value;
use biobase_rs::r::methods_annotated_data_frame::{
    annotated_data_frame, annotated_data_frame_from_matrix, p_data, var_labels, var_metadata,
};

#[test]
fn test_annotated_data_frame_accessors() {
    let data = BTreeMap::from([
        (
            "score".to_string(),
            vec![Value::Number(0.1), Value::Number(0.9)],
        ),
        (
            "type".to_string(),
            vec![
                Value::Text("control".to_string()),
                Value::Text("case".to_string()),
            ],
        ),
    ]);
    let metadata = BTreeMap::from([(
        "labelDescription".to_string(),
        vec![
            Value::Text("sample score".to_string()),
            Value::Text("sample type".to_string()),
        ],
    )]);

    let adf = annotated_data_frame(data.clone(), metadata.clone());
    assert_eq!(p_data(&adf), &data);
    assert_eq!(var_metadata(&adf), &metadata);
    assert_eq!(var_labels(&adf), vec!["score", "type"]);

    let from_features = annotated_data_frame_from_matrix(&common::named_exprs(), true);
    assert_eq!(
        from_features.dim_labels(),
        ["featureNames", "featureColumns"]
    );
}
