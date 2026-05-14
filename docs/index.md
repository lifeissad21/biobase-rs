---
title: biobase-rs Documentation
---

# biobase-rs Documentation

`biobase-rs` is a native Rust port of core Biobase ideas: coordinated assay matrices, sample and feature metadata, experiment descriptions, version records, and small utility routines that support high-throughput biological data containers.

This site rewrites the useful parts of the original Biobase manual pages and vignettes for the Rust implementation that exists in this repository. It does not describe the entire R/Bioconductor package. Features that have not been ported are called out rather than documented as available APIs.

This Rust port and documentation were created with AI assistance. The original Biobase reference repository supplied the source concepts and behavior, while the Rust code and documentation were generated and adapted for the current crate API.

## What Is Implemented

The current crate includes:

| Area | Rust API |
|---|---|
| Matrix storage | `biobase_rs::r::data_classes::Matrix<T>` |
| Generic assay containers | `ESet`, `AssayData`, `AnnotatedDataFrame`, `Miame` |
| Expression data | `ExpressionSet`, `expression_set`, `exprs`, `set_exprs` |
| Specialized sets | `MultiSet`, `NChannelSet`, `SnpSet` |
| Row operations | `row_medians`, `row_q`, `row_min`, `row_max`, `expression_set_row_medians` |
| Version records | `Version`, `Versions`, `Versioned`, `VersionedBiobase` |
| Embedded sample data | `gene_data`, `GENE_COV`, `AA_MAP`, raw `.rda` assets |
| Compatibility utilities | string helpers, missing-value checks, container and environment-like helpers |

## Reading Path

Start with [Installation](installation.html), then [Getting Started](getting-started.html). For the object model behind the crate, read [Object Model](concepts/object-model.html) and [ExpressionSet](concepts/expression-set.html). If you are migrating R code, go directly to [Migrating From R](tutorials/migrating-from-r.html).

## Source Lineage

The documentation draws from the original Biobase materials:

- `man/class.eSet.Rd`, `class.ExpressionSet.Rd`, `class.AnnotatedDataFrame.Rd`, `class.AssayData.Rd`
- `vignettes/ExpressionSetIntroduction.Rnw`
- `vignettes/BiobaseDevelopment.Rmd`
- `vignettes/esApply.Rmd`
- dataset documentation such as `data.geneData.Rd`, `data.aaMap.Rd`, and `data.sample.ExpressionSet.Rd`
- package metadata, examples, unit tests, and embedded data files

The Rust API is the source of truth for examples on this site.
