use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct FullScreenLayout {
    pub main: Rect,
    pub status_bar: Rect,
}

pub struct SplitLayout {
    pub blame: Rect,
    pub diff: Rect,
    pub status_bar: Rect,
}

/// Build full-screen layout (blame only)
pub fn full_screen(area: Rect) -> FullScreenLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    FullScreenLayout {
        main: chunks[0],
        status_bar: chunks[1],
    }
}

/// Build split layout (blame + diff)
pub fn split(area: Rect, ratio: u16) -> SplitLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(ratio),
            Constraint::Percentage(100 - ratio),
        ])
        .split(chunks[0]);

    SplitLayout {
        blame: main_chunks[0],
        diff: main_chunks[1],
        status_bar: chunks[1],
    }
}
