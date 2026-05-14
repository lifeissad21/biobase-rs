---
title: Versioning
---

# Versioning

The original Biobase versioning docs explained how serialized objects could record class versions and later be checked or updated. `biobase-rs` currently ports version records and comparisons, but not full object migration.

## Version

```rust
use biobase_rs::r::versions_class::Version;

let a = Version::parse("1.0").unwrap();
let b = Version::parse("1.0.0").unwrap();
let c = Version::parse("1.0.1").unwrap();

assert_eq!(a, b);
assert!(c > b);
```

## Versions

```rust
use biobase_rs::r::versions_class::Versions;

let versions = Versions::from_pairs([
    ("eSet", "1.3.0"),
    ("ExpressionSet", "1.0.0"),
]).unwrap();

assert_eq!(versions.len(), 2);
assert!(versions.get("eSet").is_some());
```

## Versioned

```rust
use biobase_rs::r::versioned_classes::Versioned;
use biobase_rs::r::versions_class::Versions;
use biobase_rs::r::methods_versioned_class::versioned_is_current;

let object = Versioned::new(Versions::from_pairs([("ExpressionSet", "1.0.0")]).unwrap());
let current = Versions::from_pairs([("ExpressionSet", "1.0")]).unwrap();

assert!(versioned_is_current(&object, &current));
```

## Not Yet Implemented

`updateObject`, `updateObjectTo`, and serialized object upgrade workflows are not implemented as full Rust migrations. The versioning API should be treated as infrastructure for future compatibility.
