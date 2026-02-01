use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::domain::BlameFrame;

pub struct BlameView<'a> {
    frame: &'a BlameFrame,
}

pub struct BlameViewState {
    pub scroll_offset: usize,
}

impl<'a> BlameView<'a> {
    pub fn new(frame: &'a BlameFrame) -> Self {
        Self { frame }
    }
}

impl<'a> StatefulWidget for BlameView<'a> {
    type State = BlameViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.frame.file_path.display()));

        let inner = block.inner(area);
        block.render(area, buf);

        let visible_lines = inner.height as usize;
        let total_lines = self.frame.entries.len();

        // Adjust scroll to keep selected line visible
        if self.frame.selected_line < state.scroll_offset {
            state.scroll_offset = self.frame.selected_line;
        } else if self.frame.selected_line >= state.scroll_offset + visible_lines {
            state.scroll_offset = self.frame.selected_line - visible_lines + 1;
        }

        let start = state.scroll_offset;
        let end = (start + visible_lines).min(total_lines);

        for (i, entry) in self.frame.entries[start..end].iter().enumerate() {
            let y = inner.y + i as u16;
            let line_index = start + i;
            let is_selected = line_index == self.frame.selected_line;

            let base_style = if is_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            // Commit hash (yellow)
            let hash_span = Span::styled(
                format!("{} ", entry.commit_hash.short()),
                base_style.fg(Color::Yellow),
            );

            // Author (blue, truncated to 12 chars)
            let author = truncate(&entry.author, 12);
            let author_span = Span::styled(format!("{:>12} ", author), base_style.fg(Color::Blue));

            // Line number (dark gray)
            let line_num_span = Span::styled(
                format!("{:>5} ", entry.line_number),
                base_style.fg(Color::DarkGray),
            );

            // Content
            let content_span = Span::styled(&entry.content, base_style);

            let line = Line::from(vec![hash_span, author_span, line_num_span, content_span]);
            buf.set_line(inner.x, y, &line, inner.width);
        }
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 2).collect();
        format!("{}.. ", truncated)
    }
}
