# biobase-rs

`biobase-rs` is an experimental native Rust port of core ideas from the Bioconductor Biobase package. It focuses on the parts that are useful outside the R runtime: coordinated assay matrices, expression-set style containers, sample and feature metadata, version records, row operations, and embedded reference datasets.

This repository was written and documented with AI assistance. The Rust implementation and the Jekyll documentation were generated, inspected, and iterated on by an AI coding agent using the original Biobase repository as the reference source. The result should be treated as a porting foundation that needs normal engineering review before production use.

## What Is Implemented

- `Matrix<T>` with Biobase/R-compatible column-major layout
- `ExpressionSet`, `ESet`, `AnnotatedDataFrame`, `Miame`, `MultiSet`, `NChannelSet`, and `SnpSet` structures
- assay-data helpers for required members, dimensions, and names
- row operations such as medians, quantiles, minima, and maxima
- version parsing and comparison utilities
- typed access to selected Biobase sample datasets
- embedded raw `.rda` reference assets for future compatibility work

The crate does not yet implement the complete Biobase API. Full cross-slot validation, expression-set subsetting, RDA parsing, S4-style dispatch, and object upgrade workflows are intentionally documented as incomplete.

## Documentation

The user-facing documentation lives in `docs/` as a Jekyll site. It reinterprets the original Biobase manuals and vignettes for the Rust implementation instead of mechanically translating R documentation. Documentation can also be accessed online via GitHub Pages.

Run it locally from the `docs/` directory:

```sh
bundle install
bundle exec jekyll serve
```

## Development

Run the Rust test suite from the repository root:

```sh
cargo test
```

The tests are the best executable reference for current behavior. They cover matrix indexing, expression-set accessors, metadata structures, row operations, datasets, version handling, and selected translated helpers from Biobase.

## Project Status

This is an early port, not an official Bioconductor project. API names and behavior may change as more of the original Biobase semantics are reviewed and translated into idiomatic Rust.
