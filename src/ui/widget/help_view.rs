use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, StatefulWidget, Widget},
};

use crate::config::{KeymapConfig, key_binding_to_string};
use crate::ui::action::{BlameAction, DiffAction, GlobalAction};

pub struct HelpView<'a> {
    keymap: &'a KeymapConfig,
}

pub struct HelpViewState {
    pub scroll_offset: usize,
}

impl<'a> HelpView<'a> {
    pub fn new(keymap: &'a KeymapConfig) -> Self {
        Self { keymap }
    }

    fn format_keys(&self, keys: Vec<&crate::config::KeyBinding>) -> String {
        if keys.is_empty() {
            return "-".to_string();
        }
        keys.iter()
            .map(|k| key_binding_to_string(k))
            .collect::<Vec<_>>()
            .join(" / ")
    }

    fn help_lines(&self) -> Vec<Line<'a>> {
        let mut lines = vec![
            Line::from(Span::styled(
                "Keybindings",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled("Blame Mode", Style::default().fg(Color::Blue))),
        ];

        // Blame mode keybindings
        let blame_bindings = [
            (BlameAction::CursorDown, "Cursor down"),
            (BlameAction::CursorUp, "Cursor up"),
            (BlameAction::Cursor10Down, "Cursor 10 down"),
            (BlameAction::Cursor10Up, "Cursor 10 up"),
            (BlameAction::CursorPageDown, "Page down"),
            (BlameAction::CursorPageUp, "Page up"),
            (BlameAction::CursorTop, "Go to top"),
            (BlameAction::CursorBottom, "Go to bottom"),
            (BlameAction::DrillDown, "Drill down (blame at parent)"),
            (BlameAction::GoBack, "Go back"),
            (BlameAction::ShowDiff, "Show diff"),
        ];

        for (action, desc) in blame_bindings {
            let keys = self.format_keys(self.keymap.keys_for_blame(action));
            lines.push(Line::from(format!("  {:15} {}", keys, desc)));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Diff Mode",
            Style::default().fg(Color::Blue),
        )));

        // Diff mode keybindings
        let diff_bindings = [
            (DiffAction::ScrollDown, "Scroll down"),
            (DiffAction::ScrollUp, "Scroll up"),
            (DiffAction::Scroll10Down, "Scroll 10 down"),
            (DiffAction::Scroll10Up, "Scroll 10 up"),
            (DiffAction::ScrollPageDown, "Page down"),
            (DiffAction::ScrollPageUp, "Page up"),
            (DiffAction::ScrollTop, "Scroll to top"),
            (DiffAction::ScrollBottom, "Scroll to bottom"),
            (DiffAction::Close, "Close diff"),
        ];

        for (action, desc) in diff_bindings {
            let keys = self.format_keys(self.keymap.keys_for_diff(action));
            lines.push(Line::from(format!("  {:15} {}", keys, desc)));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Global",
            Style::default().fg(Color::Blue),
        )));

        // Global keybindings
        let global_bindings = [
            (GlobalAction::ShowHelp, "Show this help"),
            (GlobalAction::Quit, "Quit"),
        ];

        for (action, desc) in global_bindings {
            let keys = self.format_keys(self.keymap.keys_for_global(action));
            lines.push(Line::from(format!("  {:15} {}", keys, desc)));
        }

        lines
    }
}

impl<'a> StatefulWidget for HelpView<'a> {
    type State = HelpViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Center the help popup
        let width = 50.min(area.width.saturating_sub(4));
        let height = 30.min(area.height.saturating_sub(4));
        let x = area.x + (area.width - width) / 2;
        let y = area.y + (area.height - height) / 2;
        let popup_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Help (press q to close) ");

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let lines = self.help_lines();
        let visible_lines = inner.height as usize;
        let total_lines = lines.len();

        // Clamp scroll
        if state.scroll_offset > total_lines.saturating_sub(visible_lines) {
            state.scroll_offset = total_lines.saturating_sub(visible_lines);
        }

        let start = state.scroll_offset;
        let end = (start + visible_lines).min(total_lines);

        for (i, line) in lines[start..end].iter().enumerate() {
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }
    }
}
