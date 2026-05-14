---
title: Working With Expression Data
---

# Working With Expression Data

This tutorial adapts the original `esApply` vignette. Biobase used `esApply` to make expression rows and phenotype variables visible inside an R function environment. Rust does not use dynamic function environments, so the current crate exposes explicit row operations.

## Row Medians

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::row_op_methods::row_medians;

let matrix = Matrix::new(2, 3, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]).unwrap();
let medians = row_medians(&matrix, false).unwrap();

assert_eq!(medians, vec![2.0, 5.0]);
```

## Missing Values

`Value::Number(f64::NAN)` is treated as missing by `Value::is_missing`. Row medians also understand `NaN`.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::row_op_methods::row_medians;

let matrix = Matrix::new(1, 3, vec![1.0, f64::NAN, 3.0]).unwrap();

assert!(row_medians(&matrix, false).unwrap()[0].is_nan());
assert_eq!(row_medians(&matrix, true).unwrap(), vec![2.0]);
```

## Quantiles, Min, And Max

`row_q` uses a one-based `which` argument to preserve Biobase behavior.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::row_op_methods::{row_max, row_min, row_q};

let matrix = Matrix::new(2, 3, vec![3.0, 9.0, 1.0, 7.0, 2.0, 8.0]).unwrap();

assert_eq!(row_min(&matrix).unwrap(), vec![1.0, 7.0]);
assert_eq!(row_q(&matrix, 2).unwrap(), vec![2.0, 8.0]);
assert_eq!(row_max(&matrix).unwrap(), vec![3.0, 9.0]);
```

## ExpressionSet Row Operations

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::expression_set;
use biobase_rs::r::row_op_methods::expression_set_row_medians;

let eset = expression_set(Matrix::new(2, 3, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]).unwrap());

assert_eq!(expression_set_row_medians(&eset, false).unwrap(), vec![2.0, 5.0]);
```

## Not Yet Implemented

There is no general `es_apply` function that accepts an arbitrary closure with phenotype metadata injected into scope. Write ordinary Rust iteration code when you need custom row-wise statistics.
