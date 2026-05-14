use super::data_classes::{AssayData, ESet, MultiSet};

pub fn multi_set(assay_data: AssayData) -> MultiSet {
    MultiSet {
        base: ESet::new(assay_data),
    }
}
