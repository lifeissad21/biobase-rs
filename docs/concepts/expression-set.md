---
title: ExpressionSet
---

# ExpressionSet

The original Biobase `ExpressionSet` taught a durable data model: rows are features, columns are samples, and the matrix named `exprs` is the primary expression assay. `biobase-rs` preserves that model in `ExpressionSet`.

## Purpose

Use `ExpressionSet` when one expression matrix is the main assay and the matrix should be carried with Biobase-style metadata fields.

```rust
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_expression_set::{expression_set, exprs};

let exprs_matrix = Matrix::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]).unwrap();
let eset = expression_set(exprs_matrix);

assert!(exprs(&eset).is_some());
```

## What It Contains

An `ExpressionSet` contains `base: ESet`. The base `ESet` contains:

- `assay_data`: a map of assay names to matrices
- `pheno_data`: sample metadata
- `feature_data`: feature metadata
- `experiment_data`: `Miame`
- `annotation`: a string label for platform or annotation source
- `protocol_data`: protocol metadata

The constructor `expression_set(matrix)` inserts the matrix under the key `exprs`.

## Current Constraints

The Rust port currently guarantees that:

- the `ExpressionSet` constructor creates an assay member named `exprs`
- `exprs(&set)` returns that matrix when present
- `set_exprs(&mut set, matrix)` replaces the `exprs` matrix
- `Matrix::new` and `set_dimnames` perform local matrix validation

The following Biobase checks are not complete yet:

- phenotype row names versus assay column names
- feature metadata row names versus assay row names
- all assay matrices having identical dimensions and dimnames

Use `assay_data_valid_members`, `assay_data_dim`, and `assay_data_dims` when you need explicit checks.

## Semantic Difference From R

R Biobase allowed `ExpressionSet[features, samples]`, `$`, `[[`, and many S4 accessors. The current Rust crate does not implement subsetting operators or metadata column selection. Work directly with the underlying `Matrix`, `AnnotatedDataFrame`, and maps.
