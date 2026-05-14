---
title: ExpressionSet API
---

# ExpressionSet API

`ExpressionSet` is the Rust equivalent of the core Biobase expression assay container.

## Construction

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::expression_set;

let matrix = Matrix::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]).unwrap();
let eset = expression_set(matrix);
```

## Accessors

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::{expression_set, exprs, set_exprs, as_data_frame};

let original = Matrix::new(1, 2, vec![1.0, 2.0]).unwrap();
let mut eset = expression_set(original);

assert_eq!(exprs(&eset).unwrap().columns(), 2);

let replacement = Matrix::new(1, 2, vec![10.0, 20.0]).unwrap();
set_exprs(&mut eset, replacement);

assert_eq!(as_data_frame(&eset).len(), 2);
```

## Functions

| Function | Purpose |
|---|---|
| `expression_set(exprs)` | Construct an `ExpressionSet` with assay member `exprs` |
| `exprs(&object)` | Borrow the expression matrix |
| `set_exprs(&mut object, value)` | Replace the expression matrix |
| `as_data_frame(&object)` | Return rows by sample, equivalent to transposed expression values |

## Constraints

The constructor guarantees an `exprs` assay member. It does not yet validate phenotype or feature metadata alignment. Use matrix dimnames and explicit tests to maintain those invariants in application code.
