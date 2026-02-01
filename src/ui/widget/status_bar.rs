use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::ui::mode::Mode;

pub struct StatusBar {
    mode: Mode,
    file_path: String,
    position: String,
    stack_depth: usize,
}

impl StatusBar {
    pub fn new(
        mode: Mode,
        file_path: &str,
        current_line: usize,
        total_lines: usize,
        stack_depth: usize,
    ) -> Self {
        Self {
            mode,
            file_path: file_path.to_string(),
            position: format!("{}/{}", current_line + 1, total_lines),
            stack_depth,
        }
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Fill background
        let style = Style::default().bg(Color::DarkGray).fg(Color::White);
        for x in area.x..area.x + area.width {
            buf[(x, area.y)].set_style(style);
        }

        // Mode indicator
        let mode_style = Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        let mode_span = Span::styled(format!(" {} ", self.mode.name()), mode_style);

        // Stack depth (if > 1)
        let depth_span = if self.stack_depth > 1 {
            Span::styled(
                format!(" [depth: {}] ", self.stack_depth),
                style.fg(Color::Yellow),
            )
        } else {
            Span::raw("")
        };

        // File path
        let file_span = Span::styled(format!(" {} ", self.file_path), style);

        // Position (right aligned)
        let pos_span = Span::styled(format!(" {} ", self.position), style);

        let left = Line::from(vec![mode_span, depth_span, file_span]);
        let right = Line::from(vec![pos_span]);

        buf.set_line(area.x, area.y, &left, area.width);

        // Right align position
        let right_x = area.x + area.width.saturating_sub(self.position.len() as u16 + 2);
        buf.set_line(right_x, area.y, &right, area.width);
    }
}
