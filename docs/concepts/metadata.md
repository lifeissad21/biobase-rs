---
title: Metadata
---

# Metadata

Biobase used `AnnotatedDataFrame` to keep measured variables and descriptions of those variables together. `biobase-rs` keeps the same concept with a simpler Rust representation:

```rust
pub struct AnnotatedDataFrame {
    data: DataFrame,
    var_metadata: DataFrame,
    dim_labels: [String; 2],
}
```

`DataFrame` is currently:

```rust
type DataFrame = std::collections::BTreeMap<String, Vec<Value>>;
```

Columns are keyed by name. Values are represented by `Value`.

## Sample Metadata

Sample metadata belongs in `ESet::pheno_data`. It describes columns of the assay matrix.

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::Value;
use biobase_rs::r::methods_annotated_data_frame::{annotated_data_frame, var_labels};

let data = BTreeMap::from([
    ("type".to_string(), vec![
        Value::Text("control".to_string()),
        Value::Text("case".to_string()),
    ]),
    ("score".to_string(), vec![Value::Number(0.2), Value::Number(0.9)]),
]);

let metadata = BTreeMap::from([
    ("labelDescription".to_string(), vec![
        Value::Text("case/control status".to_string()),
        Value::Text("sample score".to_string()),
    ]),
]);

let frame = annotated_data_frame(data, metadata);
assert_eq!(var_labels(&frame), vec!["score", "type"]);
```

Because the backing map is a `BTreeMap`, labels are returned in sorted key order.

## Feature Metadata

Feature metadata belongs in `ESet::feature_data`. The helper `annotated_data_frame_from_matrix(&matrix, true)` creates an empty metadata object with feature-oriented display labels.

## Current Limitations

The Rust port does not yet validate that every data column has matching metadata or that metadata row names match the data columns. The original Biobase convention that `labelDescription` should describe each variable is still useful and should be followed by callers.
