//! Native Rust translation of Biobase's `src/` C folder.
//!
//! The original files are represented as Rust modules named after their C or
//! header counterparts.

pub mod envir;
pub mod matchpt;
pub mod rinit;
pub mod row_medians;
pub mod row_medians_type_template;
pub mod sublist_extract;
pub mod templates_types;
pub mod templates_types_undef;

pub use envir::{SlotObject, lc_prefix, row_q, unsafe_set_slot};
pub use matchpt::{MatchPointResult, matchpt};
pub use rinit::{RegisteredCallMethod, registered_call_methods};
pub use row_medians::{MatrixView, row_medians};
pub use sublist_extract::{SimplifiedValue, sublist_extract};
