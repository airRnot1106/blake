use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInfo {
    pub hash: CommitHash,
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
