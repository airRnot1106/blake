use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event};

use crate::config::{KeyBinding, KeymapConfig};
use crate::ui::action::Action;
use crate::ui::mode::{BlameModeHandler, DiffModeHandler, HelpModeHandler, Mode, ModeHandler};

/// Event handler for terminal input
pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Poll for the next event
    pub fn poll(&self) -> Result<Option<Event>> {
        if event::poll(self.tick_rate)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }
}

/// Convert crossterm key event to Action based on current mode
pub fn key_to_action(key: event::KeyEvent, mode: &Mode, keymap: &KeymapConfig) -> Action {
    let binding = KeyBinding::from(key);

    match mode {
        Mode::Blame => BlameModeHandler.handle_key(binding, keymap),
        Mode::Diff => DiffModeHandler.handle_key(binding, keymap),
        Mode::Help => HelpModeHandler.handle_key(binding, keymap),
    }
}
