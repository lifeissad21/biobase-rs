use super::versioned_classes::Versioned;
use super::versions_class::{Version, Versions};

pub fn is_versioned<T>(_object: &T) -> bool {
    false
}

pub fn versioned_is_current(object: &Versioned, current: &Versions) -> bool {
    current.iter().all(|(name, version)| {
        object
            .class_version()
            .get(name)
            .is_some_and(|object_version| object_version == version)
    })
}

pub fn class_version(object: &Versioned) -> &Versions {
    object.class_version()
}

pub fn is_valid_version(object: &Versioned, name: &str, expected: &Version) -> bool {
    object
        .class_version()
        .get(name)
        .is_some_and(|version| version == expected)
}
