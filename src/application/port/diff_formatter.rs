use crate::domain::Diff;

/// Trait for formatting diff output
pub trait DiffFormatter: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Format raw diff into displayable lines
    fn format(&self, diff: &Diff) -> Result<Vec<String>, Self::Error>;

    /// Check if this formatter is available
    fn is_available(&self) -> bool;
}
