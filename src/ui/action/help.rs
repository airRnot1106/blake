use serde::{Deserialize, Serialize};

/// Actions for Help mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum HelpAction {
    ScrollUp,
    ScrollDown,
    Close,
}
