use super::versions_class::{VersionParseError, Versions};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Versioned {
    class_version: Versions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionedBiobase {
    base: Versioned,
}

impl Versioned {
    pub fn new(class_version: Versions) -> Self {
        Self { class_version }
    }

    pub fn class_version(&self) -> &Versions {
        &self.class_version
    }

    pub fn class_version_mut(&mut self) -> &mut Versions {
        &mut self.class_version
    }
}

impl Default for Versioned {
    fn default() -> Self {
        Self::new(Versions::new())
    }
}

impl VersionedBiobase {
    pub fn new(biobase_version: &str) -> Result<Self, VersionParseError> {
        Ok(Self {
            base: Versioned::new(Versions::from_pairs([("Biobase", biobase_version)])?),
        })
    }

    pub fn versioned(&self) -> &Versioned {
        &self.base
    }
}
