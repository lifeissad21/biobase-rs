---
title: Annotations
---

# Annotations

In Biobase, `annotation` usually named a Bioconductor annotation package or chip platform. The Rust port keeps the field as a plain string on `ESet`.

```rust
use biobase_rs::r::data_classes::{AssayData, ESet};

let mut eset = ESet::new(AssayData::new());
eset.annotation = "hgu95av2".to_string();

assert_eq!(eset.annotation, "hgu95av2");
```

## What To Store

Use `annotation` for a short platform or external annotation identifier:

- array platform name
- genome build label
- probe annotation source
- internal reference data version

Use `feature_data` for experiment-specific feature annotations that should travel with the dataset.

## What Is Not Ported

The crate does not integrate with Bioconductor annotation packages, `AnnotationDbi`, GO, KEGG, or external biological databases. Treat `annotation` as a stable label, not as an active lookup mechanism.
