use ansi_to_tui::IntoText;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
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

        // Join visible lines and convert ANSI to styled Text
        let visible_content = self.lines[start..end].join("\n");
        let text: Text = visible_content
            .as_bytes()
            .into_text()
            .unwrap_or_else(|_| Text::raw(&visible_content));

        let paragraph = Paragraph::new(text);
        paragraph.render(inner, buf);
    }
}
