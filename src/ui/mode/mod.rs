mod blame;
mod diff;
mod help;

pub use blame::BlameModeHandler;
pub use diff::DiffModeHandler;
pub use help::HelpModeHandler;

use crate::config::{KeyBinding, KeymapConfig};
use crate::ui::action::Action;

/// Application mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Blame,
    Diff,
    Help,
}

impl Mode {
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Blame => "BLAME",
            Mode::Diff => "DIFF",
            Mode::Help => "HELP",
        }
    }
}

/// Trait for handling key events per mode
pub trait ModeHandler {
    fn handle_key(&self, key: KeyBinding, keymap: &KeymapConfig) -> Action;
}
