use std::path::Path;

use crate::domain::{BlameFrame, CommitHash, CommitInfo, Diff};

pub trait GitGateway {
    type Error: std::error::Error + Send + Sync + 'static;

    fn blame(&self, file_path: &Path, commit: &CommitHash) -> Result<BlameFrame, Self::Error>;

    fn diff(&self, commit: &CommitHash) -> Result<Diff, Self::Error>;

    fn commit_info(&self, commit: &CommitHash) -> Result<CommitInfo, Self::Error>;
}
