---
title: Installation
---

# Installation

This crate currently lives in the repository as a local Rust package. Add it to another workspace with a path dependency:

```toml
[dependencies]
biobase-rs = { path = "../biobase-rs" }
```

Inside Rust code, the crate name uses underscores:

```rust
use biobase_rs::r::data_classes::Matrix;
```

The crate has no external dependencies at the moment and uses Rust 2024:

```toml
[package]
edition = "2024"
```

## Verify The Crate

Run the test suite from the repository root:

```sh
cargo test
```

The tests are also useful as executable documentation. They cover matrix layout, expression sets, annotated data frames, row operations, datasets, version handling, and translated helpers from the original Biobase codebase.

## Build This Documentation Site

The `docs/` directory is a Jekyll site. From `docs/`, run:

```sh
bundle exec jekyll serve
```

If Jekyll is installed globally, this is enough:

```sh
jekyll serve
```

The site is static Markdown plus a small layout. It does not require a Rust build step.
