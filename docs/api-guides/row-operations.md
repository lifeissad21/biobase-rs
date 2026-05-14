---
title: Row Operations
---

# Row Operations

The row operation API ports the behavior of Biobase helpers such as `rowMedians`, `rowQ`, `rowMin`, and `rowMax`.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::row_op_methods::{row_medians, row_q, row_min, row_max};

let matrix = Matrix::new(2, 3, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]).unwrap();

assert_eq!(row_medians(&matrix, false).unwrap(), vec![2.0, 5.0]);
assert_eq!(row_q(&matrix, 2).unwrap(), vec![2.0, 5.0]);
assert_eq!(row_min(&matrix).unwrap(), vec![1.0, 4.0]);
assert_eq!(row_max(&matrix).unwrap(), vec![3.0, 6.0]);
```

`row_q` uses one-based `which`, matching the original R API rather than Rust indexing.

## ExpressionSet Helper

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::expression_set;
use biobase_rs::r::row_op_methods::expression_set_row_medians;

let eset = expression_set(Matrix::new(1, 3, vec![1.0, 2.0, 3.0]).unwrap());
assert_eq!(expression_set_row_medians(&eset, false).unwrap(), vec![2.0]);
```
