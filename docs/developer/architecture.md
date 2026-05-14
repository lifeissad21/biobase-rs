---
title: Architecture
---

# Architecture

The crate mirrors the original Biobase source organization at a broad level but expresses the implementation as native Rust modules.

## Module Layout

| Module | Role |
|---|---|
| `src/r/data_classes.rs` | Core data structures translated from R classes |
| `src/r/methods_expression_set.rs` | ExpressionSet constructor and accessors |
| `src/r/methods_annotated_data_frame.rs` | AnnotatedDataFrame helpers |
| `src/r/methods_assay_data.rs` | AssayData validation and name helpers |
| `src/r/row_op_methods.rs` | Row summaries and quantiles |
| `src/r/versions_class.rs` | Version parsing and comparison |
| `src/data/` | Typed and embedded datasets |
| `src/C/` | Rust translations of selected C helpers |
| `tests/` | Ported behavior tests from Biobase examples and unit tests |

## Design Choices

The port favors explicit Rust data ownership over exact R runtime behavior:

- `BTreeMap` gives deterministic ordering for assay and metadata maps.
- `Matrix<T>` preserves column-major indexing for parity with R data.
- `Result<_, String>` is used for lightweight validation errors in current APIs.
- S4 generics become module functions or inherent methods.
- R package tooling is treated as source documentation, not a runtime feature.

## Documentation Strategy

This site is organized by Rust concepts, not by original `.Rd` file names. Each page maps original Biobase intent to the implemented Rust surface.
