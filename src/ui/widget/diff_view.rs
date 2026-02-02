use ansi_to_tui::IntoText;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    text::Line,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

pub struct DiffView<'a> {
    lines: &'a [String],
}

pub struct DiffViewState {
    pub scroll_offset: usize,
    pub selected_line: usize,
}

impl<'a> DiffView<'a> {
    pub fn new(lines: &'a [String]) -> Self {
        Self { lines }
    }
}

impl<'a> StatefulWidget for DiffView<'a> {
    type State = DiffViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().borders(Borders::ALL).title(" Diff ");

        let inner = block.inner(area);
        block.render(area, buf);

        let visible_lines = inner.height as usize;
        let total_lines = self.lines.len();

        // Clamp selected line
        if state.selected_line >= total_lines {
            state.selected_line = total_lines.saturating_sub(1);
        }

        // Adjust scroll to keep selected line visible
        if state.selected_line < state.scroll_offset {
            state.scroll_offset = state.selected_line;
        } else if state.selected_line >= state.scroll_offset + visible_lines {
            state.scroll_offset = state.selected_line - visible_lines + 1;
        }

        let start = state.scroll_offset;
        let end = (start + visible_lines).min(total_lines);

        for (i, line_content) in self.lines[start..end].iter().enumerate() {
            let y = inner.y + i as u16;
            let line_index = start + i;
            let is_selected = line_index == state.selected_line;

            // Convert ANSI to styled Line
            let text = line_content
                .as_bytes()
                .into_text()
                .unwrap_or_else(|_| line_content.as_str().into());

            let mut line: Line = if text.lines.is_empty() {
                Line::raw("")
            } else {
                text.lines.into_iter().next().unwrap_or_default()
            };

            // Apply REVERSED modifier for selected line
            if is_selected {
                for span in &mut line.spans {
                    span.style = span.style.add_modifier(Modifier::REVERSED);
                }
            }

            buf.set_line(inner.x, y, &line, inner.width);
        }
    }
}
