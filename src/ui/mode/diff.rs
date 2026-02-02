use crate::config::{KeyBinding, KeymapConfig};
use crate::ui::action::Action;

use super::ModeHandler;

pub struct DiffModeHandler;

impl ModeHandler for DiffModeHandler {
    fn handle_key(&self, key: KeyBinding, keymap: &KeymapConfig) -> Action {
        // Check diff-specific keymap first
        if let Some(action) = keymap.diff.get(&key) {
            return Action::Diff(*action);
        }

        // Then check global keymap
        if let Some(action) = keymap.global.get(&key) {
            return Action::Global(*action);
        }

        Action::None
    }
}
