//! Native Rust translation of Biobase's `R/` folder.
//!
//! The original R files used S4 classes and generic dispatch. This module keeps
//! the original file boundaries as Rust modules and models the same concepts
//! with structs, traits, and ordinary functions.

pub mod all_generics;
pub mod any_missing;
pub mod data_classes;
pub mod environment;
pub mod methods_aggregator;
pub mod methods_annotated_data_frame;
pub mod methods_assay_data;
pub mod methods_container;
pub mod methods_e_set;
pub mod methods_expression_set;
pub mod methods_miame;
pub mod methods_miaxe;
pub mod methods_multi_set;
pub mod methods_n_channel_set;
pub mod methods_scalar_object;
pub mod methods_snp_set;
pub mod methods_versioned_class;
pub mod methods_versions_null;
pub mod packages;
pub mod row_op_methods;
pub mod strings;
pub mod tools;
pub mod update_object_to;
pub mod versioned_classes;
pub mod versions_class;
pub mod vignettes;
pub mod zzz;

pub use data_classes::*;
pub use versions_class::{Version, VersionParseError, Versions};
