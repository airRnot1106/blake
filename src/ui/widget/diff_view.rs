use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

pub struct DiffView<'a> {
    lines: &'a [String],
}

pub struct DiffViewState {
    pub scroll_offset: usize,
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

        // Clamp scroll offset
        if state.scroll_offset > total_lines.saturating_sub(visible_lines) {
            state.scroll_offset = total_lines.saturating_sub(visible_lines);
        }

        let start = state.scroll_offset;
        let end = (start + visible_lines).min(total_lines);

        for (i, line_content) in self.lines[start..end].iter().enumerate() {
            let y = inner.y + i as u16;

            // Lines contain ANSI escape codes from delta, render as-is
            let line = Line::styled(line_content.as_str(), Style::default());
            buf.set_line(inner.x, y, &line, inner.width);
        }
    }
}
