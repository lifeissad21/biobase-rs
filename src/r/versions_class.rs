use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Eq)]
pub struct Version {
    parts: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionParseError {
    Empty,
    InvalidPart(String),
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionParseError::Empty => write!(f, "version must contain at least one part"),
            VersionParseError::InvalidPart(part) => {
                write!(f, "invalid non-negative integer version part: {part}")
            }
        }
    }
}

impl Error for VersionParseError {}

impl Version {
    pub fn new(parts: impl Into<Vec<u32>>) -> Result<Self, VersionParseError> {
        let parts = parts.into();
        if parts.is_empty() {
            return Err(VersionParseError::Empty);
        }
        Ok(Self { parts })
    }

    pub fn parse(input: &str) -> Result<Self, VersionParseError> {
        input.parse()
    }

    pub fn parts(&self) -> &[u32] {
        &self.parts
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.trim().is_empty() {
            return Err(VersionParseError::Empty);
        }
        let parts = input
            .split('.')
            .map(|part| {
                part.parse::<u32>()
                    .map_err(|_| VersionParseError::InvalidPart(part.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Self::new(parts)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self
            .parts
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(".");
        f.write_str(&text)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_len = self.parts.len().max(other.parts.len());
        for idx in 0..max_len {
            let left = *self.parts.get(idx).unwrap_or(&0);
            let right = *other.parts.get(idx).unwrap_or(&0);
            match left.cmp(&right) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Versions {
    values: BTreeMap<String, Version>,
}

impl Versions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_pairs<I, K, V>(pairs: I) -> Result<Self, VersionParseError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: AsRef<str>,
    {
        let mut values = BTreeMap::new();
        for (name, version) in pairs {
            values.insert(name.into(), Version::parse(version.as_ref())?);
        }
        Ok(Self { values })
    }

    pub fn insert(
        &mut self,
        name: impl Into<String>,
        version: impl Into<Version>,
    ) -> Option<Version> {
        self.values.insert(name.into(), version.into())
    }

    pub fn get(&self, name: &str) -> Option<&Version> {
        self.values.get(name)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.values.keys().map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Version)> {
        self.values
            .iter()
            .map(|(name, version)| (name.as_str(), version))
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
