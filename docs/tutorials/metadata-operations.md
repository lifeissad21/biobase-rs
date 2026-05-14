---
title: Metadata Operations
---

# Metadata Operations

The original Biobase metadata workflow used `AnnotatedDataFrame` to store both values and descriptions of those values. In Rust, the same idea is represented by `AnnotatedDataFrame` and `DataFrame`.

## Create Sample Metadata

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::Value;
use biobase_rs::r::methods_annotated_data_frame::{
    annotated_data_frame, p_data, var_metadata, var_labels,
};

let data = BTreeMap::from([
    ("gender".to_string(), vec![
        Value::Text("Female".to_string()),
        Value::Text("Male".to_string()),
    ]),
    ("score".to_string(), vec![Value::Number(0.75), Value::Number(0.40)]),
]);

let meta = BTreeMap::from([
    ("labelDescription".to_string(), vec![
        Value::Text("Patient gender".to_string()),
        Value::Text("Tumor progress score".to_string()),
    ]),
]);

let pheno = annotated_data_frame(data, meta);

assert_eq!(var_labels(&pheno), vec!["gender", "score"]);
assert!(p_data(&pheno).contains_key("gender"));
assert!(var_metadata(&pheno).contains_key("labelDescription"));
```

## Attach Metadata To An ExpressionSet

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::{Matrix, Value};
use biobase_rs::r::methods_annotated_data_frame::annotated_data_frame;
use biobase_rs::r::methods_expression_set::expression_set;

let exprs = Matrix::new(1, 2, vec![10.0, 11.0]).unwrap();
let mut eset = expression_set(exprs);

let data = BTreeMap::from([
    ("type".to_string(), vec![
        Value::Text("control".to_string()),
        Value::Text("case".to_string()),
    ]),
]);
let meta = BTreeMap::from([
    ("labelDescription".to_string(), vec![
        Value::Text("Case/control status".to_string()),
    ]),
]);

eset.base.pheno_data = annotated_data_frame(data, meta);
assert_eq!(eset.base.pheno_data.var_labels(), vec!["type"]);
```

## Practical Rules

- Keep sample metadata vectors the same length as the number of assay columns.
- Keep feature metadata vectors aligned with assay rows.
- Include a `labelDescription` metadata column when possible.
- Validate alignment in your application code until full cross-slot validation is implemented in the crate.
