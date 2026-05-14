---
title: AssayData API
---

# AssayData API

`AssayData` is a named collection of assay matrices:

```rust
type AssayData = std::collections::BTreeMap<String, Matrix<f64>>;
```

It replaces Biobase's list/environment/locked-environment storage modes with a predictable Rust map.

## Required Members

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_assay_data::assay_data_valid_members;

let assay_data = BTreeMap::from([
    ("exprs".to_string(), Matrix::new(1, 2, vec![1.0, 2.0]).unwrap()),
]);

assert!(assay_data_valid_members(&assay_data, &["exprs"]).is_ok());
assert!(assay_data_valid_members(&assay_data, &["exprs", "se.exprs"]).is_err());
```

## Dimension Helpers

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_assay_data::{assay_data_dim, assay_data_dims};

let assay_data = BTreeMap::from([
    ("exprs".to_string(), Matrix::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]).unwrap()),
]);

assert_eq!(assay_data_dim(&assay_data), Some((2, 2)));
assert_eq!(assay_data_dims(&assay_data), vec![("exprs".to_string(), 2, 2)]);
```

## Names

`sample_names` returns matrix column names. `feature_names` returns matrix row names.

## Current Limitations

The helper functions inspect dimensions and required names. They do not yet enforce that every matrix in the map has identical dimensions and dimnames.
