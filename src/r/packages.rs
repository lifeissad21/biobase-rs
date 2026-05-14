#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
}

pub fn package_version(pkg: &PackageInfo) -> &str {
    &pkg.version
}

pub fn create_package_info(name: impl Into<String>, version: impl Into<String>) -> PackageInfo {
    PackageInfo {
        name: name.into(),
        version: version.into(),
    }
}
