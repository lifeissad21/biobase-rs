---
title: Specialized Sets
---

# Specialized Sets

Biobase included several `eSet` subclasses. The Rust port currently exposes lightweight equivalents backed by `ESet`.

## MultiSet

`MultiSet` is a flexible assay collection with no required assay member.

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::Matrix;
use biobase_rs::r::methods_multi_set::multi_set;

let assay_data = BTreeMap::from([
    ("channel_a".to_string(), Matrix::new(1, 2, vec![1.0, 2.0]).unwrap()),
]);
let multi = multi_set(assay_data);

assert!(multi.base.assay_data.contains_key("channel_a"));
```

## NChannelSet

`NChannelSet` models named channels and supports channel selection.

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::{ESet, Matrix, NChannelSet};
use biobase_rs::r::methods_n_channel_set::{channel, channel_names, select_channels};

let assay_data = BTreeMap::from([
    ("R".to_string(), Matrix::new(1, 2, vec![1.0, 2.0]).unwrap()),
    ("G".to_string(), Matrix::new(1, 2, vec![3.0, 4.0]).unwrap()),
]);
let set = NChannelSet { base: ESet::new(assay_data) };

assert_eq!(channel_names(&set), vec!["G", "R"]);
assert_eq!(channel(&set, "R").unwrap().values(), &[1.0, 2.0]);

let selected = select_channels(&set, &["G"]);
assert_eq!(channel_names(&selected), vec!["G"]);
```

## SnpSet

`SnpSet` expects assay members named `call` and `callProbability`.

```rust
use std::collections::BTreeMap;
use biobase_rs::r::data_classes::{ESet, Matrix, SnpSet};
use biobase_rs::r::methods_snp_set::{snp_call, snp_call_probability};

let assay_data = BTreeMap::from([
    ("call".to_string(), Matrix::new(1, 2, vec![0.0, 1.0]).unwrap()),
    ("callProbability".to_string(), Matrix::new(1, 2, vec![0.7, 0.9]).unwrap()),
]);
let set = SnpSet { base: ESet::new(assay_data) };

assert_eq!(snp_call(&set).unwrap().values(), &[0.0, 1.0]);
assert_eq!(snp_call_probability(&set).unwrap().values(), &[0.7, 0.9]);
```

The content of SNP calls is not validated by the class yet.
