---
title: Migrating From R
---

# Migrating From R

This guide maps common Biobase patterns onto the current Rust APIs.

## Object Mapping

| R/Biobase | Rust |
|---|---|
| `ExpressionSet(assayData = exprs)` | `expression_set(exprs_matrix)` |
| `exprs(object)` | `exprs(&object)` or `object.exprs()` |
| `exprs(object) <- value` | `set_exprs(&mut object, value)` |
| `assayData(object)` | `object.base.assay_data` |
| `assayDataElement(object, "x")` | `assay_data_element(&object.base, "x")` |
| `AnnotatedDataFrame(data, varMetadata)` | `annotated_data_frame(data, var_metadata)` |
| `pData(object)` | `p_data(&object)` |
| `varMetadata(object)` | `var_metadata(&object)` |
| `rowMedians(exprs(object))` | `expression_set_row_medians(&object, na_rm)` |
| `rowQ(matrix, which)` | `row_q(&matrix, which)` |
| `data(geneData)` | `biobase_rs::data::gene_data()` |

## Constructor Translation

R:

```r
ExpressionSet(assayData = exprs)
```

Rust:

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::expression_set;

let exprs = Matrix::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]).unwrap();
let eset = expression_set(exprs);
```

## Accessor Translation

R replacement functions mutate through assignment syntax. Rust uses explicit mutable functions.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::{expression_set, exprs, set_exprs};

let original = Matrix::new(1, 2, vec![1.0, 2.0]).unwrap();
let mut eset = expression_set(original);

let replacement = Matrix::new(1, 2, vec![10.0, 20.0]).unwrap();
set_exprs(&mut eset, replacement);

assert_eq!(exprs(&eset).unwrap().values(), &[10.0, 20.0]);
```

## Matrix Indexing Pitfall

Biobase and R store matrices column-major. `Matrix::new` expects the same order.

```rust
use biobase_rs::r::data_classes::Matrix;

let matrix = Matrix::new(2, 3, vec![
    1.0, 4.0,
    2.0, 5.0,
    3.0, 6.0,
]).unwrap();

assert_eq!(matrix.get(0, 0), Some(&1.0));
assert_eq!(matrix.get(1, 0), Some(&4.0));
assert_eq!(matrix.get(0, 1), Some(&2.0));
```

## Semantic Differences

| Area | Difference |
|---|---|
| S4 classes | Replaced by Rust structs and module functions |
| Replacement methods | Replaced by explicit setters or mutable references |
| R environments | Replaced by owned maps and values |
| `esApply` | Replaced by explicit row operations; no environment injection |
| Subsetting | `ExpressionSet[i, j]` is not implemented yet |
| Validation | Some local validation exists; full cross-slot validation is incomplete |
| RDA loading | Raw `.rda` bytes are embedded, but parsed object reconstruction is not implemented |

## Common Migration Strategy

1. Convert expression matrices into `Matrix<f64>` with column-major values.
2. Set row and column names with `set_dimnames`.
3. Wrap the matrix with `expression_set`.
4. Represent sample metadata as `AnnotatedDataFrame`.
5. Use row operation helpers for currently ported analyses.
6. Keep Biobase object parity tests around when porting behavior.
