use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, StatefulWidget, Widget},
};

pub struct HelpView;

pub struct HelpViewState {
    pub scroll_offset: usize,
}

impl HelpView {
    pub fn new() -> Self {
        Self
    }

    fn help_lines() -> Vec<Line<'static>> {
        vec![
            Line::from(Span::styled(
                "Keybindings",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled("Blame Mode", Style::default().fg(Color::Blue))),
            Line::from("  j / Down      Cursor down"),
            Line::from("  k / Up        Cursor up"),
            Line::from("  J             Cursor 10 down"),
            Line::from("  K             Cursor 10 up"),
            Line::from("  Ctrl+d        Page down"),
            Line::from("  Ctrl+u        Page up"),
            Line::from("  g / Home      Go to top"),
            Line::from("  G / End       Go to bottom"),
            Line::from("  Enter         Drill down (blame at parent)"),
            Line::from("  Backspace     Go back"),
            Line::from("  d             Show diff"),
            Line::from(""),
            Line::from(Span::styled("Diff Mode", Style::default().fg(Color::Blue))),
            Line::from("  j / Down      Scroll down"),
            Line::from("  k / Up        Scroll up"),
            Line::from("  J             Scroll 10 down"),
            Line::from("  K             Scroll 10 up"),
            Line::from("  Ctrl+d        Page down"),
            Line::from("  Ctrl+u        Page up"),
            Line::from("  g / Home      Scroll to top"),
            Line::from("  G / End       Scroll to bottom"),
            Line::from("  q / Esc       Close diff"),
            Line::from(""),
            Line::from(Span::styled("Global", Style::default().fg(Color::Blue))),
            Line::from("  ?             Show this help"),
            Line::from("  q             Quit"),
        ]
    }
}

impl Default for HelpView {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for HelpView {
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

        let lines = Self::help_lines();
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
