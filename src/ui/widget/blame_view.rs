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

        // Calculate min/max timestamps for color gradient
        let (min_ts, max_ts) = self
            .frame
            .entries
            .iter()
            .fold((i64::MAX, i64::MIN), |(min, max), e| {
                (min.min(e.timestamp), max.max(e.timestamp))
            });

        for (i, entry) in self.frame.entries[start..end].iter().enumerate() {
            let y = inner.y + i as u16;
            let line_index = start + i;
            let is_selected = line_index == self.frame.selected_line;

            let base_style = if is_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            // Fill entire line with REVERSED background first (for selected line)
            if is_selected {
                let reversed_style = Style::default().add_modifier(Modifier::REVERSED);
                for x in inner.x..inner.x + inner.width {
                    buf[(x, y)].set_style(reversed_style);
                }
            }

            // Commit hash with age-based color (newer = brighter yellow, older = darker)
            let hash_color = age_to_color(entry.timestamp, min_ts, max_ts);
            let hash_span = Span::styled(
                format!("{} ", entry.commit_hash.short()),
                base_style.fg(hash_color),
            );

            // Author (blue, truncated to 12 chars)
            let author = truncate(&entry.author, 12);
            let author_span = Span::styled(format!("{:>12} ", author), base_style.fg(Color::Blue));

            // Timestamp (green)
            let timestamp_span = Span::styled(
                format!("{} ", format_timestamp(entry.timestamp)),
                base_style.fg(Color::Green),
            );

            // Line number (dark gray)
            let line_num_span = Span::styled(
                format!("{:>5} ", entry.line_number),
                base_style.fg(Color::DarkGray),
            );

            // Content
            let content_span = Span::styled(&entry.content, base_style);

            let line = Line::from(vec![
                hash_span,
                author_span,
                timestamp_span,
                line_num_span,
                content_span,
            ]);
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

fn format_timestamp(timestamp: i64) -> String {
    use std::time::{Duration, UNIX_EPOCH};

    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let secs = datetime
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Convert to date components
    let days = secs / 86400;
    let mut year = 1970;
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

    let mut month = 1;
    for days_in_month in days_in_months {
        if remaining_days < days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }

    let day = remaining_days + 1;

    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// Convert timestamp age to color using terminal colors (newer = brighter)
fn age_to_color(timestamp: i64, min_ts: i64, max_ts: i64) -> Color {
    let range = max_ts - min_ts;
    let ratio = if range == 0 {
        1.0
    } else {
        (timestamp - min_ts) as f64 / range as f64
    };

    // Use terminal colors: DarkGray -> Gray -> White -> Yellow
    if ratio < 0.25 {
        Color::DarkGray
    } else if ratio < 0.5 {
        Color::Gray
    } else if ratio < 0.75 {
        Color::White
    } else {
        Color::Yellow
    }
}
