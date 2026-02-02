mod application;
mod config;
mod domain;
mod infrastructure;
mod ui;

use std::io::stdout;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use crossterm::ExecutableCommand;
use crossterm::event::Event;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;
use ratatui::widgets::StatefulWidget;

use crate::config::ConfigLoader;
use crate::infrastructure::{DeltaFormatter, Git2Gateway};
use crate::ui::app::{App, LayoutState};
use crate::ui::event::{EventHandler, key_to_action};
use crate::ui::layout;
use crate::ui::mode::Mode;
use crate::ui::widget::{
    BlameView, BlameViewState, DiffView, DiffViewState, HelpView, HelpViewState, StatusBar,
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: blake <file>");
        std::process::exit(1);
    }

    let file_path = PathBuf::from(&args[1]);
    if !file_path.exists() {
        eprintln!("File not found: {}", file_path.display());
        std::process::exit(1);
    }

    run(file_path)
}

fn run(file_path: PathBuf) -> Result<()> {
    // Load config
    let config = ConfigLoader::load()?;

    // Create dependencies
    let git = Git2Gateway::open_current()?;
    let formatter = DeltaFormatter::new();

    // Create app
    let mut app = App::new(git, formatter, config, file_path)?;

    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Widget states
    let mut blame_state = BlameViewState { scroll_offset: 0 };
    let mut diff_state = DiffViewState {
        scroll_offset: 0,
        selected_line: 0,
    };
    let mut help_state = HelpViewState { scroll_offset: 0 };

    // Event handler
    let event_handler = EventHandler::new(Duration::from_millis(100));

    // Main loop
    loop {
        // Render
        terminal.draw(|frame| {
            render(
                &app,
                frame,
                &mut blame_state,
                &mut diff_state,
                &mut help_state,
            );
        })?;

        // Handle events
        if let Some(event) = event_handler.poll()? {
            if let Event::Key(key) = event {
                let action = key_to_action(key, &app.mode, &app.config.keymap);
                app.dispatch(action)?;
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn render(
    app: &App<Git2Gateway, DeltaFormatter>,
    frame: &mut Frame,
    blame_state: &mut BlameViewState,
    diff_state: &mut DiffViewState,
    help_state: &mut HelpViewState,
) {
    let area = frame.area();

    match &app.layout {
        LayoutState::FullScreen => {
            let layout = layout::full_screen(area);

            // Blame view
            if let Some(blame_frame) = app.blame_stack.current() {
                let blame_view = BlameView::new(blame_frame);
                blame_view.render(layout.main, frame.buffer_mut(), blame_state);

                // Status bar
                let status_bar = StatusBar::new(
                    app.mode.clone(),
                    &blame_frame.file_path.to_string_lossy(),
                    blame_frame.selected_line,
                    blame_frame.entries.len(),
                    app.blame_stack.hash_chain(),
                )
                .with_message(app.status_message.as_deref());
                frame.render_widget(status_bar, layout.status_bar);
            }
        }
        LayoutState::Split { ratio } => {
            let split = layout::split(area, *ratio);

            // Blame view
            if let Some(blame_frame) = app.blame_stack.current() {
                let blame_view = BlameView::new(blame_frame);
                blame_view.render(split.blame, frame.buffer_mut(), blame_state);

                // Status bar
                let status_bar = StatusBar::new(
                    app.mode.clone(),
                    &blame_frame.file_path.to_string_lossy(),
                    blame_frame.selected_line,
                    blame_frame.entries.len(),
                    app.blame_stack.hash_chain(),
                )
                .with_message(app.status_message.as_deref());
                frame.render_widget(status_bar, split.status_bar);
            }

            // Diff view
            if let Some(lines) = &app.diff_lines {
                diff_state.selected_line = app.diff_selected_line;
                let diff_view = DiffView::new(lines, app.diff_commit_info.as_ref());
                diff_view.render(split.diff, frame.buffer_mut(), diff_state);
            }
        }
    }

    // Help overlay
    if matches!(app.mode, Mode::Help) {
        help_state.scroll_offset = app.help_scroll;
        let help_view = HelpView::new();
        help_view.render(area, frame.buffer_mut(), help_state);
    }
}
