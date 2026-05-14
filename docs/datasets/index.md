---
title: Datasets
---

# Datasets

The crate embeds selected data from Biobase's `data/` directory. Some datasets have typed Rust accessors; others are available as raw `.rda` bytes for compatibility and future parser work.

## Gene Expression Matrix

`gene_data()` exposes the sample expression matrix as a typed, column-major Rust matrix wrapper.

```rust
use biobase_rs::data::gene_data;

let data = gene_data();
assert_eq!(data.rows(), 500);
assert_eq!(data.columns(), 26);
assert_eq!(data.row_names()[0], "AFFX-MurIL2_at");
assert_eq!(data.column_names()[0], "A");
assert_eq!(data.get_by_names("AFFX-MurIL2_at", "A"), Some(192.742));
```

Origin: Biobase `geneData` sample expression matrix and `inst/extdata/exprsData.txt`. Intended use: examples and regression tests for matrix orientation and expression-data workflows.

## Gene Covariates

```rust
use biobase_rs::data::gene_covariate;

let cov = gene_covariate('A').unwrap();
assert_eq!(cov.cov1, 1);
assert_eq!(cov.cov2, 1);
assert_eq!(cov.cov3, 1);
```

Origin: Biobase `geneCov` sample covariates.

## Amino Acid Map

```rust
use biobase_rs::data::{SideChainProperty, amino_acid_by_three_letter};

let valine = amino_acid_by_three_letter("val").unwrap();
assert_eq!(valine.one_letter, 'V');
assert_eq!(valine.side_chain_property, SideChainProperty::Nonpolar);
```

Origin: Biobase `aaMap`. This dataset is useful as a small typed lookup example, not as part of the core expression-set model.

## Raw RDA Assets

The following `.rda` files are embedded as bytes:

- `geneCovariate.rda`
- `reporter.rda`
- `sample.ExpressionSet.rda`
- `sample.MultiSet.rda`
- `seD.rda`
- `SW.rda`

```rust
use biobase_rs::data::DataAsset;

for asset in DataAsset::ALL {
    assert!(!asset.file_name().is_empty());
    assert!(!asset.bytes().is_empty());
}
```

The crate does not yet parse these RDA payloads into Rust structs.
