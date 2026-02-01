use serde::{Deserialize, Serialize};

/// Actions available in all modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum GlobalAction {
    Quit,
    ShowHelp,
}
