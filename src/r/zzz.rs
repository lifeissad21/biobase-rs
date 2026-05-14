pub fn on_load() -> bool {
    true
}

pub fn on_attach() -> &'static str {
    "Biobase native Rust module attached"
}

pub fn on_unload() -> bool {
    true
}
