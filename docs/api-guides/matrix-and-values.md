---
title: Matrix and Values
---

# Matrix And Values

## `Matrix<T>`

`Matrix<T>` is the crate's basic two-dimensional assay storage type.

```rust
use biobase_rs::r::data_classes::Matrix;

let matrix = Matrix::new(2, 2, vec![1, 3, 2, 4]).unwrap();
assert_eq!(matrix.rows(), 2);
assert_eq!(matrix.columns(), 2);
assert_eq!(matrix.get(1, 0), Some(&3));
```

### Important Methods

| Method | Purpose |
|---|---|
| `Matrix::new(rows, columns, values)` | Construct and validate value count |
| `rows()` / `columns()` | Return dimensions |
| `values()` / `values_mut()` | Borrow raw column-major storage |
| `get(row, column)` | Read a value by zero-based row and column |
| `set_dimnames(row_names, column_names)` | Attach optional row and column names |
| `row_names()` / `column_names()` | Borrow attached names |

## `Value`

`Value` is a small dynamic value enum used by metadata tables and some utilities.

```rust
use biobase_rs::r::data_classes::Value;

assert!(Value::Null.is_missing());
assert!(Value::Number(f64::NAN).is_missing());
assert!(!Value::Text("case".to_string()).is_missing());
```

`Value` variants are `Null`, `Bool`, `Integer`, `Number`, `Text`, and `Vector`.
