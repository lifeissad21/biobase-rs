---
title: Object Model
---

# Object Model

Biobase organized high-throughput experiments around a compact rule: assay values, sample metadata, feature metadata, and experiment metadata must travel together. `biobase-rs` keeps that rule, but expresses it with Rust structs, maps, options, and explicit validation helpers rather than S4 classes and replacement methods.

## Core Types

| Biobase idea | Rust type |
|---|---|
| Assay matrix | `Matrix<f64>` |
| Named assay collection | `AssayData = BTreeMap<String, Matrix<f64>>` |
| Common experiment container | `ESet` |
| Expression container | `ExpressionSet` |
| Sample or feature metadata table | `AnnotatedDataFrame` |
| Experiment description | `Miame` |
| Object version metadata | `Version`, `Versions`, `Versioned` |

## ESet Composition

`ESet` is a concrete struct with public fields:

```rust
use biobase_rs::r::data_classes::{AssayData, ESet};

let assay_data = AssayData::new();
let eset = ESet::new(assay_data);

assert_eq!(eset.annotation, "");
assert_eq!(eset.pheno_data.dim_labels(), ["sampleNames", "sampleColumns"]);
assert_eq!(eset.feature_data.dim_labels(), ["featureNames", "featureColumns"]);
```

The original R `eSet` was virtual. The Rust port currently uses `ESet` as a reusable base field inside `ExpressionSet`, `MultiSet`, `NChannelSet`, and `SnpSet`.

## Matrix Layout

`Matrix<T>` is column-major. The index calculation is:

```text
index = column * rows + row
```

This preserves the original R matrix layout and makes translated tests compare directly against Biobase data.

## Validation

Validation is implemented in focused helper functions today:

- `Matrix::new` checks that the value count equals `rows * columns`.
- `Matrix::set_dimnames` checks row and column name lengths.
- `assay_data_valid_members` checks required assay members such as `exprs`, `call`, or `callProbability`.
- Row operations return `Result` when inputs can be invalid, such as an out-of-range quantile index.

Full cross-slot validation, such as checking that phenotype row names match assay column names, is not implemented yet.

## Rust Differences

The Rust port intentionally avoids several R-specific mechanisms:

- No S4 dispatch. Methods are inherent methods or module functions.
- No replacement functions such as `exprs<-`; use `set_exprs` or mutable access.
- No R environments or locked environments for assay storage; assay data is a `BTreeMap`.
- No implicit variable lookup in statistical functions; row operations are explicit Rust functions.
