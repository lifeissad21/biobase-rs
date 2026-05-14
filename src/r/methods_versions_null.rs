use super::versions_class::Versions;

pub fn versions_null() -> Versions {
    Versions::new()
}

pub fn show(object: &Versions) -> String {
    if object.is_empty() {
        "Versioned; no version string".to_string()
    } else {
        object
            .iter()
            .map(|(name, version)| format!("{name}: {version}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
