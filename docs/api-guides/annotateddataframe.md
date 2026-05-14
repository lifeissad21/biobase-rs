---
title: AnnotatedDataFrame API
---

# AnnotatedDataFrame API

`AnnotatedDataFrame` stores a data table and a second table describing the variables in that data table.

## Construction

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::Value;
use biobase_rs::r::methods_annotated_data_frame::annotated_data_frame;

let data = BTreeMap::from([
    ("score".to_string(), vec![Value::Number(0.1), Value::Number(0.9)]),
]);
let metadata = BTreeMap::from([
    ("labelDescription".to_string(), vec![Value::Text("sample score".to_string())]),
]);

let frame = annotated_data_frame(data, metadata);
assert_eq!(frame.dim_labels(), ["rowNames", "columnNames"]);
```

## Functions

| Function | Purpose |
|---|---|
| `annotated_data_frame(data, var_metadata)` | Construct with default row/column labels |
| `annotated_data_frame_from_matrix(matrix, by_row)` | Create empty metadata with feature or sample labels |
| `p_data(&object)` | Borrow data columns |
| `var_metadata(&object)` | Borrow metadata columns |
| `var_labels(&object)` | Return data column labels |

## Semantic Difference From R

The original R class enforced more validity around data and variable metadata. The current Rust implementation stores the two maps and exposes them, but does not yet enforce `labelDescription` or column length alignment.
