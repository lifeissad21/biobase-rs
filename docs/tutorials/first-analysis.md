---
title: First Analysis
---

# First Analysis

This tutorial reinterprets the original `ExpressionSetIntroduction` vignette. Instead of reading R tables and constructing S4 objects, we build a small native Rust expression dataset, name its rows and columns, wrap it in `ExpressionSet`, and compute row summaries.

## 1. Build Expression Values

Rows are features. Columns are samples. Values are column-major.

```rust
use biobase_rs::r::data_classes::Matrix;

let mut exprs = Matrix::new(
    3,
    2,
    vec![
        10.0, 20.0, 30.0, // sample_a
        12.0, 18.0, 33.0, // sample_b
    ],
).unwrap();

exprs.set_dimnames(
    Some(vec![
        "probe_1".to_string(),
        "probe_2".to_string(),
        "probe_3".to_string(),
    ]),
    Some(vec!["sample_a".to_string(), "sample_b".to_string()]),
).unwrap();

assert_eq!(exprs.get(2, 1), Some(&33.0));
```

## 2. Create An ExpressionSet

```rust
use biobase_rs::r::methods_expression_set::{expression_set, exprs};
use biobase_rs::r::data_classes::Matrix;

let exprs = Matrix::new(3, 2, vec![10.0, 20.0, 30.0, 12.0, 18.0, 33.0]).unwrap();
let eset = expression_set(exprs);
let assay = exprs(&eset).unwrap();

assert_eq!(assay.rows(), 3);
assert_eq!(assay.columns(), 2);
```

## 3. Attach Experiment Metadata

`Miame` is available as a Rust struct. You can fill fields directly on the `ESet` base.

```rust
use biobase_rs::r::data_classes::{Matrix, Miame};
use biobase_rs::r::methods_expression_set::expression_set;

let exprs = Matrix::new(1, 2, vec![1.0, 2.0]).unwrap();
let mut eset = expression_set(exprs);

eset.base.experiment_data = Miame {
    name: "Pierre Fermat".to_string(),
    lab: "Francis Galton Lab".to_string(),
    title: "Smoking-Cancer Experiment".to_string(),
    abstract_text: "An example ExpressionSet".to_string(),
    ..Miame::default()
};
eset.base.annotation = "hgu95av2".to_string();

assert_eq!(eset.base.experiment_data.title, "Smoking-Cancer Experiment");
```

## 4. Summarize Features

The original vignette explored accessors and basic analyses. In the current Rust port, row summaries are implemented explicitly.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::expression_set;
use biobase_rs::r::row_op_methods::{expression_set_row_medians, row_max};

let matrix = Matrix::new(2, 3, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]).unwrap();
let eset = expression_set(matrix);

let medians = expression_set_row_medians(&eset, false).unwrap();
let maxima = row_max(eset.exprs().unwrap()).unwrap();

assert_eq!(medians, vec![2.0, 5.0]);
assert_eq!(maxima, vec![3.0, 6.0]);
```

## What Changed From Biobase

The original tutorial taught R object construction, `read.table`, S4 accessors, `$`, and `[` subsetting. The Rust tutorial focuses on explicit constructors and functions. File readers and full expression-set subsetting are not implemented yet, so examples use in-memory matrices and embedded datasets.
