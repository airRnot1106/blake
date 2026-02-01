use crate::config::{KeyBinding, KeymapConfig};
use crate::ui::action::Action;

use super::{Mode, ModeHandler};

pub struct BlameModeHandler;

impl ModeHandler for BlameModeHandler {
    fn handle_key(&self, key: KeyBinding, keymap: &KeymapConfig) -> Action {
        // Check blame-specific keymap first
        if let Some(action) = keymap.blame.get(&key) {
            return Action::Blame(*action);
        }

        // Then check global keymap
        if let Some(action) = keymap.global.get(&key) {
            return Action::Global(*action);
        }

        Action::None
    }

    fn mode(&self) -> Mode {
        Mode::Blame
    }
}
