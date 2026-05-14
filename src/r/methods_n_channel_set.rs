use super::data_classes::{Matrix, NChannelSet};

pub fn channel_names(object: &NChannelSet) -> Vec<&str> {
    object.base.assay_data.keys().map(String::as_str).collect()
}

pub fn channel<'a>(object: &'a NChannelSet, name: &str) -> Option<&'a Matrix<f64>> {
    object.base.assay_data.get(name)
}

pub fn select_channels(object: &NChannelSet, names: &[&str]) -> NChannelSet {
    let assay_data = names
        .iter()
        .filter_map(|name| {
            object
                .base
                .assay_data
                .get(*name)
                .cloned()
                .map(|matrix| ((*name).to_string(), matrix))
        })
        .collect();
    let mut selected = object.clone();
    selected.base.assay_data = assay_data;
    selected
}
