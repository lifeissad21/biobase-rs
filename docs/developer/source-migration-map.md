---
title: Source Migration Map
---

# Source Migration Map

This page records how the original Biobase documentation sources informed the Rust documentation.

| Source | Rust documentation outcome |
|---|---|
| `man/class.eSet.Rd` | [Object Model](../concepts/object-model.html), `ESet` discussion |
| `man/class.ExpressionSet.Rd` | [ExpressionSet concept](../concepts/expression-set.html), [ExpressionSet API](../api-guides/expressionset.html) |
| `man/class.AnnotatedDataFrame.Rd` | [Metadata](../concepts/metadata.html), [AnnotatedDataFrame API](../api-guides/annotateddataframe.html) |
| `man/class.AssayData.Rd`, `assayData.Rd` | [AssayData API](../api-guides/assaydata.html) |
| `man/class.NChannelSet.Rd`, `channel.Rd`, `channelNames.Rd`, `selectChannels.Rd` | [Specialized Sets](../api-guides/specialized-sets.html) |
| `man/class.SnpSet.Rd`, `snpCall.Rd` | [Specialized Sets](../api-guides/specialized-sets.html) |
| `man/rowMedians.Rd`, `rowQ.Rd` | [Row Operations](../api-guides/row-operations.html), [Working With Expression Data](../tutorials/working-with-expression-data.html) |
| `man/class.Versioned.Rd`, `class.Versions.Rd`, `isCurrent.Rd` | [Versioning](../api-guides/versioning.html) |
| `vignettes/ExpressionSetIntroduction.Rnw` | [First Analysis](../tutorials/first-analysis.html) |
| `vignettes/esApply.Rmd` | [Working With Expression Data](../tutorials/working-with-expression-data.html) |
| `vignettes/BiobaseDevelopment.Rmd` | [Architecture](architecture.html), [Implementation Notes](implementation-notes.html) |
| `man/data.*.Rd`, `data/`, `inst/extdata/` | [Datasets](../datasets/index.html) |
| `DESCRIPTION`, `NAMESPACE`, `NEWS` | Site overview, implementation scope, migration caveats |

Sources tied to R package installation, Windows menus, PDF viewers, and Bioconductor repository tooling were intentionally not converted into user-facing Rust docs.
