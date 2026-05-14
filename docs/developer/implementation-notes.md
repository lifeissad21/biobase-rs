---
title: Implementation Notes
---

# Implementation Notes

## Column-Major Storage

`Matrix<T>` uses column-major layout so that values copied from R data keep their meaning. This is visible in `Matrix::get` and the dataset tests:

```text
value_index = column * rows + row
```

## Validation Scope

The original Biobase classes performed substantial S4 validity checking. The Rust port currently implements local checks:

- matrix value count
- dimname lengths
- required assay members
- row-operation argument bounds
- version parsing

Cross-object validation is a future area: assay matrix dimname coordination, phenotype row count, feature row count, and channel-specific metadata rules.

## RDA Assets

Some R serialized files are embedded with `include_bytes!`. This preserves reference artifacts and supports future compatibility work, but these bytes are not parsed into Rust objects yet.

## `esApply`

Biobase `esApply` relied on R environments to expose phenotype columns while applying a function to expression rows. The current Rust port implements direct row operations instead. A future Rust-native design should probably accept closures with explicit metadata arguments rather than emulate R lexical scoping.

## Versioning

`Version` intentionally compares `1.0` and `1.0.0` as equal, matching Biobase/R package-version expectations. Full object upgrade routines are not implemented.
