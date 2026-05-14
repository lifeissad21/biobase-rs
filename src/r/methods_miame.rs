use super::data_classes::{Miame, Value};

pub fn miame() -> Miame {
    Miame::default()
}

pub fn abstract_text(object: &Miame) -> &str {
    &object.abstract_text
}

pub fn samples(object: &Miame) -> &[Value] {
    &object.samples
}

pub fn hybridizations(object: &Miame) -> &[Value] {
    &object.hybridizations
}

pub fn norm_controls(object: &Miame) -> &[Value] {
    &object.norm_controls
}

pub fn preproc(object: &Miame) -> &[Value] {
    &object.preprocessing
}

pub fn pub_med_ids(object: &Miame) -> &[String] {
    &object.pub_med_ids
}

pub fn other_info(object: &Miame) -> &[Value] {
    &object.other
}
