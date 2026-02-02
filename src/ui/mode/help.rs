use crate::config::{KeyBinding, KeymapConfig};
use crate::ui::action::Action;

use super::ModeHandler;

pub struct HelpModeHandler;

impl ModeHandler for HelpModeHandler {
    fn handle_key(&self, key: KeyBinding, keymap: &KeymapConfig) -> Action {
        // Check help-specific keymap first
        if let Some(action) = keymap.help.get(&key) {
            return Action::Help(*action);
        }

        // Then check global keymap
        if let Some(action) = keymap.global.get(&key) {
            return Action::Global(*action);
        }

        Action::None
    }
}
