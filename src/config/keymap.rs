use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::BitOr;

use crate::ui::action::{BlameAction, DiffAction, GlobalAction, HelpAction};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Char(char),
    Enter,
    Escape,
    Backspace,
    Tab,
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    F(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyBinding {
    pub fn new(key: KeyCode) -> Self {
        // For uppercase letters, force shift to true
        let shift = matches!(key, KeyCode::Char(c) if c.is_ascii_uppercase());
        Self {
            key,
            modifiers: KeyModifiers {
                shift,
                ..KeyModifiers::default()
            },
        }
    }

    pub fn with_modifiers(mut self, modifiers: KeyModifiers) -> Self {
        self.modifiers = modifiers;
        self
    }
}

impl KeyModifiers {
    pub const CTRL: Self = Self {
        ctrl: true,
        alt: false,
        shift: false,
    };
    pub const ALT: Self = Self {
        ctrl: false,
        alt: true,
        shift: false,
    };
    pub const SHIFT: Self = Self {
        ctrl: false,
        alt: false,
        shift: true,
    };
}

impl BitOr for KeyModifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ctrl: self.ctrl || rhs.ctrl,
            alt: self.alt || rhs.alt,
            shift: self.shift || rhs.shift,
        }
    }
}

fn parse_key_binding(s: &str) -> Result<KeyBinding, String> {
    let parts: Vec<&str> = s.split('+').collect();
    let mut modifiers = KeyModifiers::default();
    let mut key_part = "";

    for part in &parts {
        match part.to_lowercase().as_str() {
            "ctrl" => modifiers.ctrl = true,
            "alt" => modifiers.alt = true,
            "shift" => modifiers.shift = true,
            _ => key_part = part,
        }
    }

    let key = match key_part.to_lowercase().as_str() {
        "enter" => KeyCode::Enter,
        "escape" | "esc" => KeyCode::Escape,
        "backspace" => KeyCode::Backspace,
        "tab" => KeyCode::Tab,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
        s if s.starts_with('f') && s.len() > 1 => {
            let num: u8 = s[1..]
                .parse()
                .map_err(|_| format!("Invalid F key: {}", s))?;
            KeyCode::F(num)
        }
        _ => return Err(format!("Unknown key: {}", key_part)),
    };

    Ok(KeyBinding { key, modifiers })
}

pub fn key_binding_to_string(binding: &KeyBinding) -> String {
    let mut parts = Vec::new();

    if binding.modifiers.ctrl {
        parts.push("Ctrl".to_string());
    }
    if binding.modifiers.alt {
        parts.push("Alt".to_string());
    }
    if binding.modifiers.shift {
        parts.push("Shift".to_string());
    }

    let key_str = match binding.key {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Escape => "Escape".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::F(n) => format!("F{}", n),
    };
    parts.push(key_str);

    parts.join("+")
}

impl Serialize for KeyBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&key_binding_to_string(self))
    }
}

impl<'de> Deserialize<'de> for KeyBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_key_binding(&s).map_err(serde::de::Error::custom)
    }
}

impl From<crossterm::event::KeyEvent> for KeyBinding {
    fn from(event: crossterm::event::KeyEvent) -> Self {
        let key = match event.code {
            crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
            crossterm::event::KeyCode::Enter => KeyCode::Enter,
            crossterm::event::KeyCode::Esc => KeyCode::Escape,
            crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
            crossterm::event::KeyCode::Tab => KeyCode::Tab,
            crossterm::event::KeyCode::Up => KeyCode::Up,
            crossterm::event::KeyCode::Down => KeyCode::Down,
            crossterm::event::KeyCode::Left => KeyCode::Left,
            crossterm::event::KeyCode::Right => KeyCode::Right,
            crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
            crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
            crossterm::event::KeyCode::Home => KeyCode::Home,
            crossterm::event::KeyCode::End => KeyCode::End,
            crossterm::event::KeyCode::F(n) => KeyCode::F(n),
            _ => KeyCode::Char('\0'),
        };

        // For uppercase letters, force shift to true
        let is_uppercase_char =
            matches!(event.code, crossterm::event::KeyCode::Char(c) if c.is_ascii_uppercase());

        let modifiers = KeyModifiers {
            ctrl: event
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL),
            alt: event
                .modifiers
                .contains(crossterm::event::KeyModifiers::ALT),
            shift: is_uppercase_char
                || event
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::SHIFT),
        };

        Self { key, modifiers }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct KeymapConfig {
    #[serde(default)]
    pub global: HashMap<KeyBinding, GlobalAction>,
    #[serde(default)]
    pub blame: HashMap<KeyBinding, BlameAction>,
    #[serde(default)]
    pub diff: HashMap<KeyBinding, DiffAction>,
    #[serde(default)]
    pub help: HashMap<KeyBinding, HelpAction>,
}

impl KeymapConfig {
    /// Find all keys bound to a specific global action
    pub fn keys_for_global(&self, action: GlobalAction) -> Vec<&KeyBinding> {
        self.global
            .iter()
            .filter(|(_, a)| **a == action)
            .map(|(k, _)| k)
            .collect()
    }

    /// Find all keys bound to a specific blame action
    pub fn keys_for_blame(&self, action: BlameAction) -> Vec<&KeyBinding> {
        self.blame
            .iter()
            .filter(|(_, a)| **a == action)
            .map(|(k, _)| k)
            .collect()
    }

    /// Find all keys bound to a specific diff action
    pub fn keys_for_diff(&self, action: DiffAction) -> Vec<&KeyBinding> {
        self.diff
            .iter()
            .filter(|(_, a)| **a == action)
            .map(|(k, _)| k)
            .collect()
    }

    /// Find all keys bound to a specific help action
    pub fn keys_for_help(&self, action: HelpAction) -> Vec<&KeyBinding> {
        self.help
            .iter()
            .filter(|(_, a)| **a == action)
            .map(|(k, _)| k)
            .collect()
    }

    pub fn with_defaults() -> Self {
        let mut config = Self::default();

        // Global
        config
            .global
            .insert(KeyBinding::new(KeyCode::Char('q')), GlobalAction::Quit);
        config.global.insert(
            KeyBinding::new(KeyCode::Char('c')).with_modifiers(KeyModifiers::CTRL),
            GlobalAction::Quit,
        );
        config
            .global
            .insert(KeyBinding::new(KeyCode::Char('?')), GlobalAction::ShowHelp);

        // Blame
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Char('j')), BlameAction::CursorDown);
        config.blame.insert(
            KeyBinding::new(KeyCode::Char('J')),
            BlameAction::Cursor10Down,
        );
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Char('k')), BlameAction::CursorUp);
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Char('K')), BlameAction::Cursor10Up);
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Down), BlameAction::CursorDown);
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Up), BlameAction::CursorUp);
        config.blame.insert(
            KeyBinding::new(KeyCode::Char('d')).with_modifiers(KeyModifiers::CTRL),
            BlameAction::CursorPageDown,
        );
        config.blame.insert(
            KeyBinding::new(KeyCode::Char('u')).with_modifiers(KeyModifiers::CTRL),
            BlameAction::CursorPageUp,
        );
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Char('g')), BlameAction::CursorTop);
        config.blame.insert(
            KeyBinding::new(KeyCode::Char('G')),
            BlameAction::CursorBottom,
        );
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Char(',')), BlameAction::DrillDown);
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Char('u')), BlameAction::GoBack);
        config
            .blame
            .insert(KeyBinding::new(KeyCode::Enter), BlameAction::ShowDiff);

        // Diff
        config
            .diff
            .insert(KeyBinding::new(KeyCode::Char('j')), DiffAction::ScrollDown);
        config.diff.insert(
            KeyBinding::new(KeyCode::Char('J')),
            DiffAction::Scroll10Down,
        );
        config
            .diff
            .insert(KeyBinding::new(KeyCode::Char('k')), DiffAction::ScrollUp);
        config
            .diff
            .insert(KeyBinding::new(KeyCode::Char('K')), DiffAction::Scroll10Up);
        config
            .diff
            .insert(KeyBinding::new(KeyCode::Down), DiffAction::ScrollDown);
        config
            .diff
            .insert(KeyBinding::new(KeyCode::Up), DiffAction::ScrollUp);
        config.diff.insert(
            KeyBinding::new(KeyCode::Char('d')).with_modifiers(KeyModifiers::CTRL),
            DiffAction::ScrollPageDown,
        );
        config.diff.insert(
            KeyBinding::new(KeyCode::Char('u')).with_modifiers(KeyModifiers::CTRL),
            DiffAction::ScrollPageUp,
        );
        config
            .diff
            .insert(KeyBinding::new(KeyCode::Char('q')), DiffAction::Close);
        config
            .diff
            .insert(KeyBinding::new(KeyCode::Escape), DiffAction::Close);

        // Help
        config
            .help
            .insert(KeyBinding::new(KeyCode::Char('j')), HelpAction::ScrollDown);
        config
            .help
            .insert(KeyBinding::new(KeyCode::Char('k')), HelpAction::ScrollUp);
        config.help.insert(
            KeyBinding::new(KeyCode::Char('J')),
            HelpAction::Scroll10Down,
        );
        config
            .help
            .insert(KeyBinding::new(KeyCode::Char('K')), HelpAction::Scroll10Up);
        config
            .help
            .insert(KeyBinding::new(KeyCode::Char('q')), HelpAction::Close);
        config
            .help
            .insert(KeyBinding::new(KeyCode::Escape), HelpAction::Close);

        config
    }
}
