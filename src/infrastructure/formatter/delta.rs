use std::io::Write;
use std::process::{Command, Stdio};

use thiserror::Error;

use crate::application::port::DiffFormatter;
use crate::domain::Diff;

#[derive(Debug, Error)]
pub enum DeltaError {
    #[error("delta not found in PATH")]
    NotFound,

    #[error("delta execution failed: {0}")]
    Execution(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct DeltaFormatter {
    delta_path: Option<String>,
}

impl DeltaFormatter {
    pub fn new() -> Self {
        let delta_path = which::which("delta")
            .ok()
            .map(|p| p.to_string_lossy().to_string());

        Self { delta_path }
    }
}

impl Default for DeltaFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffFormatter for DeltaFormatter {
    type Error = DeltaError;

    fn format(&self, diff: &Diff) -> Result<Vec<String>, Self::Error> {
        let delta_path = self.delta_path.as_ref().ok_or(DeltaError::NotFound)?;

        let mut child = Command::new(delta_path)
            .args(["--color-only", "--paging=never"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(diff.as_str().as_bytes())?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            return Err(DeltaError::Execution(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let formatted = String::from_utf8_lossy(&output.stdout);
        let lines = formatted.lines().map(|s| s.to_string()).collect();

        Ok(lines)
    }

    fn is_available(&self) -> bool {
        self.delta_path.is_some()
    }
}
