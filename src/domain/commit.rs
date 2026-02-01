use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInfo {
    pub hash: CommitHash,
    pub parent: Option<CommitHash>,
    pub author: String,
    pub timestamp: i64,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitHash(String);
impl CommitHash {
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn short(&self) -> &str {
        &self.0[..7.min(self.0.len())]
    }

    pub fn head() -> Self {
        Self("HEAD".to_string())
    }
}

impl fmt::Display for CommitHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for CommitHash {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for CommitHash {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn short_is_at_most_7_chars(s in "[0-9a-f]{40}") {
            let hash = CommitHash::new(&s);
            prop_assert!(hash.short().len() <= 7);
        }

        #[test]
        fn short_is_prefix_of_original(s in "[0-9a-f]{40}") {
            let hash = CommitHash::new(&s);
            prop_assert!(hash.as_str().starts_with(hash.short()));
        }

        #[test]
        fn display_equals_original(s in "[0-9a-f]{40}") {
            let hash = CommitHash::new(&s);
            prop_assert_eq!(format!("{}", hash), hash.as_str());
        }

        #[test]
        fn from_str_roundtrip(s in "[0-9a-f]{40}") {
            let hash: CommitHash = s.as_str().into();
            prop_assert_eq!(hash.as_str(), s);
        }
    }
}
