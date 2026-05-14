---
title: Utilities
---

# Utilities

Some Biobase utility behavior has been ported where it supports tests or low-level compatibility.

## Missing Values

```rust
use biobase_rs::r::data_classes::Value;
use biobase_rs::r::any_missing::any_missing;

let values = vec![Value::Integer(1), Value::Null];
assert!(any_missing(&values));
```

## String Helpers

```rust
use biobase_rs::r::strings::{lc_prefix, lc_suffix};

let values = ["sample_one", "sample_two"];
assert_eq!(lc_prefix(&values, false), "sample_");
assert_eq!(lc_suffix(&["left.fastq", "right.fastq"], false), ".fastq");
```

## Selection And Uniqueness

```rust
use biobase_rs::r::tools::{duplicated, is_unique, select_some};

assert_eq!(is_unique(&[1, 2, 1]), vec![false, true, false]);
assert_eq!(duplicated(&[1, 2, 1]), vec![false, false, true]);
assert_eq!(select_some(&[1, 2, 3, 4], 2), vec![1, 2]);
```

## Environment-Like Helpers

The Rust environment helpers use `BTreeMap`, not R environments. They are compatibility utilities rather than a recommended high-level data model.
