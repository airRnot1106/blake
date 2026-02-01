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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::CommitHash;
    use proptest::prelude::*;

    // テスト用のBlameFrameを生成する戦略
    fn arbitrary_frame() -> impl Strategy<Value = BlameFrame> {
        ("[a-z]{1,20}", "[0-9a-f]{40}", 0..1000usize).prop_map(|(path, hash, line)| BlameFrame {
            file_path: path.into(),
            commit_hash: CommitHash::new(hash),
            entries: vec![],
            selected_line: line,
        })
    }

    proptest! {
        #[test]
        fn push_increases_depth(frames in proptest::collection::vec(arbitrary_frame(), 1..10)) {
            let mut stack = BlameStack::new();

            for (i, frame) in frames.into_iter().enumerate() {
                prop_assert_eq!(stack.depth(), i);
                stack.push(frame);
                prop_assert_eq!(stack.depth(), i + 1);
            }
        }

        #[test]
        fn pop_decreases_depth(frames in proptest::collection::vec(arbitrary_frame(), 1..10)) {
            let mut stack = BlameStack::new();
            for frame in &frames {
                stack.push(frame.clone());
            }

            for i in (0..frames.len()).rev() {
                prop_assert_eq!(stack.depth(), i + 1);
                let popped = stack.pop();
                prop_assert!(popped.is_some());
                prop_assert_eq!(stack.depth(), i);
            }
        }

        #[test]
        fn current_returns_last_pushed(frames in proptest::collection::vec(arbitrary_frame(), 1..10)) {
            let mut stack = BlameStack::new();

            for frame in frames {
                stack.push(frame.clone());
                let current = stack.current().unwrap();
                prop_assert_eq!(&current.file_path, &frame.file_path);
                prop_assert_eq!(current.selected_line, frame.selected_line);
            }
        }

        #[test]
        fn pop_returns_in_lifo_order(frames in proptest::collection::vec(arbitrary_frame(), 1..10)) {
            let mut stack = BlameStack::new();
            for frame in &frames {
                stack.push(frame.clone());
            }

            // 逆順でpopされる（LIFO）
            for frame in frames.into_iter().rev() {
                let popped = stack.pop().unwrap();
                prop_assert_eq!(popped.file_path, frame.file_path);
            }
        }
    }

    #[test]
    fn empty_stack_pop_returns_none() {
        let mut stack = BlameStack::new();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn empty_stack_current_returns_none() {
        let stack = BlameStack::new();
        assert!(stack.current().is_none());
    }

    #[test]
    fn new_stack_is_empty() {
        let stack = BlameStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.depth(), 0);
    }
}
