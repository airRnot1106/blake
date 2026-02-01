use std::path::PathBuf;

use crate::domain::CommitHash;

#[derive(Debug, Clone)]
pub struct BlameEntry {
    pub line_number: usize,
    pub commit_hash: CommitHash,
    pub author: String,
    pub timestamp: i64,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct BlameFrame {
    pub file_path: PathBuf,
    pub commit_hash: CommitHash,
    pub entries: Vec<BlameEntry>,
    pub selected_line: usize,
}

#[derive(Debug, Default)]
pub struct BlameStack {
    frames: Vec<BlameFrame>,
}

impl BlameStack {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn push(&mut self, frame: BlameFrame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Option<BlameFrame> {
        self.frames.pop()
    }

    pub fn current(&self) -> Option<&BlameFrame> {
        self.frames.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut BlameFrame> {
        self.frames.last_mut()
    }

    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}
