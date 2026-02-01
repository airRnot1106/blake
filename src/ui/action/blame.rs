use serde::{Deserialize, Serialize};

/// Actions for Blame mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum BlameAction {
    // Cursor movement
    CursorUp,
    CursorDown,
    Cursor10Up,
    Cursor10Down,
    CursorPageUp,
    CursorPageDown,
    CursorTop,
    CursorBottom,

    // Blame navigation
    DrillDown,
    GoBack,

    // Show diff
    ShowDiff,
}
