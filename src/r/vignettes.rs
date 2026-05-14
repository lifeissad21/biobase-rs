#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vignette {
    pub package: String,
    pub topic: String,
    pub path: String,
}

pub fn get_pkg_vigs(package: &str, vignettes: &[Vignette]) -> Vec<Vignette> {
    vignettes
        .iter()
        .filter(|vignette| vignette.package == package)
        .cloned()
        .collect()
}

pub fn open_vignette(package: &str, topic: &str, vignettes: &[Vignette]) -> Option<Vignette> {
    vignettes
        .iter()
        .find(|vignette| vignette.package == package && vignette.topic == topic)
        .cloned()
}
