use serde::{Deserialize, Serialize};

/// Actions for Diff mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum DiffAction {
    // Vertical scroll
    ScrollUp,
    ScrollDown,
    Scroll10Up,
    Scroll10Down,
    ScrollPageUp,
    ScrollPageDown,
    ScrollTop,
    ScrollBottom,

    // Horizontal scroll
    ScrollLeft,
    ScrollRight,

    // Close diff view
    Close,

    // Open in GitHub
    OpenInGitHub,
}
