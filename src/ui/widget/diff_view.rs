use ansi_to_tui::IntoText;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::domain::CommitInfo;

pub struct DiffView<'a> {
    lines: &'a [String],
    commit_info: Option<&'a CommitInfo>,
}

pub struct DiffViewState {
    pub scroll_offset: usize,
    pub selected_line: usize,
}

impl<'a> DiffView<'a> {
    pub fn new(lines: &'a [String], commit_info: Option<&'a CommitInfo>) -> Self {
        Self { lines, commit_info }
    }

    fn header_lines(&self) -> Vec<Line<'a>> {
        let Some(info) = self.commit_info else {
            return vec![];
        };

        let yellow = Style::default().fg(Color::Yellow);
        let normal = Style::default();

        vec![
            Line::from(vec![
                Span::styled("commit ", normal),
                Span::styled(info.hash.as_str(), yellow),
            ]),
            Line::from(vec![
                Span::styled("Author: ", normal),
                Span::raw(&info.author),
            ]),
            Line::from(vec![
                Span::styled("Date:   ", normal),
                Span::raw(format_timestamp(info.timestamp)),
            ]),
            Line::raw(""),
            Line::from(vec![Span::styled("    ", normal), Span::raw(&info.message)]),
            Line::raw(""),
        ]
    }
}

impl<'a> StatefulWidget for DiffView<'a> {
    type State = DiffViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().borders(Borders::ALL).title(" Diff ");

        let inner = block.inner(area);
        block.render(area, buf);

        let header = self.header_lines();
        let header_len = header.len();
        let visible_lines = inner.height as usize;
        let total_lines = header_len + self.lines.len();

        // Clamp selected line (only diff lines are selectable, not header)
        let selectable_lines = self.lines.len();
        if state.selected_line >= selectable_lines {
            state.selected_line = selectable_lines.saturating_sub(1);
        }

        // Adjust scroll to keep selected line visible (account for header)
        let selected_visual = header_len + state.selected_line;
        if selected_visual < state.scroll_offset {
            state.scroll_offset = selected_visual;
        } else if selected_visual >= state.scroll_offset + visible_lines {
            state.scroll_offset = selected_visual - visible_lines + 1;
        }

        let start = state.scroll_offset;
        let end = (start + visible_lines).min(total_lines);

        for i in start..end {
            let y = inner.y + (i - start) as u16;

            if i < header_len {
                // Render header line
                let line = &header[i];
                buf.set_line(inner.x, y, line, inner.width);
            } else {
                // Render diff line
                let diff_index = i - header_len;
                let line_content = &self.lines[diff_index];
                let is_selected = diff_index == state.selected_line;

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
}

fn format_timestamp(timestamp: i64) -> String {
    use std::time::{Duration, UNIX_EPOCH};

    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let secs = datetime
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    let mut year = 1970u64;
    let mut remaining_days = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let days_in_months: [u64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1u64;
    for days_in_month in days_in_months {
        if remaining_days < days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }

    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
