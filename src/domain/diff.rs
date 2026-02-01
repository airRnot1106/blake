#[derive(Debug, Clone)]
pub struct Diff(String);

impl Diff {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
