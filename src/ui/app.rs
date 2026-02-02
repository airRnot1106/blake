use std::path::PathBuf;

use anyhow::Result;

use crate::application::port::DiffFormatter;
use crate::config::AppConfig;
use crate::domain::{BlameStack, CommitHash, GitGateway};
use crate::ui::action::{Action, BlameAction, DiffAction, GlobalAction, HelpAction};
use crate::ui::mode::Mode;

/// Layout state
#[derive(Debug, Clone)]
pub enum LayoutState {
    FullScreen,
    Split { ratio: u16 },
}

/// Application state
pub struct App<G: GitGateway, F: DiffFormatter> {
    // Dependencies
    git: G,
    formatter: F,

    // Config
    pub config: AppConfig,

    // State
    pub mode: Mode,
    pub blame_stack: BlameStack,
    pub diff_lines: Option<Vec<String>>,

    // UI state
    pub layout: LayoutState,
    pub diff_selected_line: usize,
    pub help_scroll: usize,

    // Flags
    pub should_quit: bool,
}

impl<G: GitGateway, F: DiffFormatter> App<G, F> {
    pub fn new(git: G, formatter: F, config: AppConfig, file_path: PathBuf) -> Result<Self> {
        // Check if delta is available
        if !formatter.is_available() {
            anyhow::bail!(
                "delta is not installed. Please install delta: https://github.com/dandavison/delta"
            );
        }

        // Get initial blame
        let initial_frame = git.blame(&file_path, &CommitHash::head())?;
        let mut blame_stack = BlameStack::new();
        blame_stack.push(initial_frame);

        Ok(Self {
            git,
            formatter,
            config,
            mode: Mode::Blame,
            blame_stack,
            diff_lines: None,
            layout: LayoutState::FullScreen,
            diff_selected_line: 0,
            help_scroll: 0,
            should_quit: false,
        })
    }

    pub fn dispatch(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Global(ga) => self.handle_global(ga),
            Action::Blame(ba) => self.handle_blame(ba),
            Action::Diff(da) => self.handle_diff(da),
            Action::Help(ha) => self.handle_help(ha),
            Action::None => Ok(()),
        }
    }

    fn handle_global(&mut self, action: GlobalAction) -> Result<()> {
        match action {
            GlobalAction::Quit => {
                self.should_quit = true;
            }
            GlobalAction::ShowHelp => {
                self.mode = Mode::Help;
            }
        }
        Ok(())
    }

    fn handle_blame(&mut self, action: BlameAction) -> Result<()> {
        let frame = match self.blame_stack.current_mut() {
            Some(f) => f,
            None => return Ok(()),
        };

        let total = frame.entries.len();

        match action {
            BlameAction::CursorUp => {
                frame.selected_line = frame.selected_line.saturating_sub(1);
            }
            BlameAction::CursorDown => {
                if frame.selected_line < total.saturating_sub(1) {
                    frame.selected_line += 1;
                }
            }
            BlameAction::Cursor10Up => {
                frame.selected_line = frame.selected_line.saturating_sub(10);
            }
            BlameAction::Cursor10Down => {
                frame.selected_line = (frame.selected_line + 10).min(total.saturating_sub(1));
            }
            BlameAction::CursorPageUp => {
                frame.selected_line = frame.selected_line.saturating_sub(20);
            }
            BlameAction::CursorPageDown => {
                frame.selected_line = (frame.selected_line + 20).min(total.saturating_sub(1));
            }
            BlameAction::CursorTop => {
                frame.selected_line = 0;
            }
            BlameAction::CursorBottom => {
                frame.selected_line = total.saturating_sub(1);
            }
            BlameAction::DrillDown => {
                self.drill_down()?;
            }
            BlameAction::GoBack => {
                self.go_back();
            }
            BlameAction::ShowDiff => {
                self.show_diff()?;
            }
        }
        Ok(())
    }

    fn handle_diff(&mut self, action: DiffAction) -> Result<()> {
        let total = self.diff_lines.as_ref().map(|l| l.len()).unwrap_or(0);

        match action {
            DiffAction::ScrollUp => {
                self.diff_selected_line = self.diff_selected_line.saturating_sub(1);
            }
            DiffAction::ScrollDown => {
                if self.diff_selected_line < total.saturating_sub(1) {
                    self.diff_selected_line += 1;
                }
            }
            DiffAction::Scroll10Up => {
                self.diff_selected_line = self.diff_selected_line.saturating_sub(10);
            }
            DiffAction::Scroll10Down => {
                self.diff_selected_line =
                    (self.diff_selected_line + 10).min(total.saturating_sub(1));
            }
            DiffAction::ScrollPageUp => {
                self.diff_selected_line = self.diff_selected_line.saturating_sub(20);
            }
            DiffAction::ScrollPageDown => {
                self.diff_selected_line =
                    (self.diff_selected_line + 20).min(total.saturating_sub(1));
            }
            DiffAction::ScrollTop => {
                self.diff_selected_line = 0;
            }
            DiffAction::ScrollBottom => {
                self.diff_selected_line = total.saturating_sub(1);
            }
            DiffAction::ScrollLeft | DiffAction::ScrollRight => {
                // Horizontal scroll not implemented yet
            }
            DiffAction::Close => {
                self.diff_lines = None;
                self.layout = LayoutState::FullScreen;
                self.mode = Mode::Blame;
            }
        }
        Ok(())
    }

    fn handle_help(&mut self, action: HelpAction) -> Result<()> {
        match action {
            HelpAction::ScrollUp => {
                self.help_scroll = self.help_scroll.saturating_sub(1);
            }
            HelpAction::ScrollDown => {
                self.help_scroll += 1;
            }
            HelpAction::Close => {
                self.mode = Mode::Blame;
            }
        }
        Ok(())
    }

    fn drill_down(&mut self) -> Result<()> {
        let (file_path, commit_hash) = {
            let frame = match self.blame_stack.current() {
                Some(f) => f,
                None => return Ok(()),
            };

            let entry = match frame.entries.get(frame.selected_line) {
                Some(e) => e,
                None => return Ok(()),
            };

            (frame.file_path.clone(), entry.commit_hash.clone())
        };

        // Get parent commit from commit info
        let commit_info = self.git.commit_info(&commit_hash)?;
        let parent = match commit_info.parent {
            Some(p) => p,
            None => return Ok(()), // Initial commit, no parent to drill into
        };

        let new_frame = self.git.blame(&file_path, &parent)?;
        self.blame_stack.push(new_frame);

        Ok(())
    }

    fn go_back(&mut self) {
        if self.blame_stack.depth() > 1 {
            self.blame_stack.pop();
        }
    }

    fn show_diff(&mut self) -> Result<()> {
        let commit_hash = {
            let frame = match self.blame_stack.current() {
                Some(f) => f,
                None => return Ok(()),
            };

            let entry = match frame.entries.get(frame.selected_line) {
                Some(e) => e,
                None => return Ok(()),
            };

            entry.commit_hash.clone()
        };

        let diff = self.git.diff(&commit_hash)?;
        let lines = self.formatter.format(&diff)?;

        self.diff_lines = Some(lines);
        self.diff_selected_line = 0;
        self.layout = LayoutState::Split { ratio: 50 };
        self.mode = Mode::Diff;

        Ok(())
    }
}
