---
title: Examples
---

# Examples

These examples are small and compile against the current crate API.

## Build A Named ExpressionSet

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::{expression_set, exprs};

let mut matrix = Matrix::new(2, 2, vec![5.0, 8.0, 6.0, 9.0]).unwrap();
matrix.set_dimnames(
    Some(vec!["gene_1".to_string(), "gene_2".to_string()]),
    Some(vec!["sample_1".to_string(), "sample_2".to_string()]),
).unwrap();

let eset = expression_set(matrix);
assert_eq!(exprs(&eset).unwrap().row_names().unwrap()[0], "gene_1");
```

## Summarize Embedded Data

```rust
use biobase_rs::data::gene_data;

let data = gene_data();
let first_feature = data.row_names()[0];
let first_sample = data.column_names()[0];
let value = data.get_by_names(first_feature, first_sample).unwrap();

assert_eq!(value, 192.742);
```

## Select Channels

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::{ESet, Matrix, NChannelSet};
use biobase_rs::r::methods_n_channel_set::{channel_names, select_channels};

let assay_data = BTreeMap::from([
    ("red".to_string(), Matrix::new(1, 1, vec![1.0]).unwrap()),
    ("green".to_string(), Matrix::new(1, 1, vec![2.0]).unwrap()),
]);
let set = NChannelSet { base: ESet::new(assay_data) };
let green = select_channels(&set, &["green"]);

assert_eq!(channel_names(&green), vec!["green"]);
```
