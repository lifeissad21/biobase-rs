use super::data_classes::Miaxe;

pub fn show(object: &Miaxe) -> String {
    format!(
        "MIAxE with {} version entrie(s)",
        object.versioned().class_version().len()
    )
}
