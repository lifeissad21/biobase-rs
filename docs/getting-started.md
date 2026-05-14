---
title: Getting Started
---

# Getting Started

The central workflow is the same conceptually as Biobase: place assay values in a matrix, attach names and metadata, then use container APIs that keep expression values and sample-oriented operations coordinated.

## Create A Named Matrix

`Matrix<T>` stores values in column-major order to preserve Biobase and R matrix semantics.

```rust
use biobase_rs::r::data_classes::Matrix;

let mut exprs = Matrix::new(
    2,
    3,
    vec![
        1.0, 4.0, // sample_1: gene_a, gene_b
        2.0, 5.0, // sample_2
        3.0, 6.0, // sample_3
    ],
).unwrap();

exprs.set_dimnames(
    Some(vec!["gene_a".to_string(), "gene_b".to_string()]),
    Some(vec![
        "sample_1".to_string(),
        "sample_2".to_string(),
        "sample_3".to_string(),
    ]),
).unwrap();

assert_eq!(exprs.get(0, 1), Some(&2.0));
```

## Wrap It In An ExpressionSet

An `ExpressionSet` is an `ESet` whose assay data contains an `exprs` matrix.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::{expression_set, exprs};

let matrix = Matrix::new(2, 2, vec![10.0, 20.0, 11.0, 21.0]).unwrap();
let eset = expression_set(matrix);

let assay = exprs(&eset).unwrap();
assert_eq!(assay.rows(), 2);
assert_eq!(assay.columns(), 2);
```

## Compute Row Summaries

The current Rust port includes direct row operations rather than a general `esApply` environment-injection mechanism.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::expression_set;
use biobase_rs::r::row_op_methods::expression_set_row_medians;

let matrix = Matrix::new(2, 3, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]).unwrap();
let eset = expression_set(matrix);

let medians = expression_set_row_medians(&eset, false).unwrap();
assert_eq!(medians, vec![2.0, 5.0]);
```

## Use Embedded Data

The sample expression matrix from Biobase is available as typed Rust data.

```rust
use biobase_rs::data::gene_data;

let data = gene_data();
assert_eq!(data.rows(), 500);
assert_eq!(data.columns(), 26);
assert_eq!(data.get_by_names("AFFX-MurIL2_at", "A"), Some(192.742));
```

Next, read [ExpressionSet Concepts](concepts/expression-set.html) and the [First Analysis](tutorials/first-analysis.html) tutorial.
