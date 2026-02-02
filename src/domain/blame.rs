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

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Get the chain of commit hashes as "hash1 -> hash2 -> ..."
    pub fn hash_chain(&self) -> Option<String> {
        if self.frames.len() <= 1 {
            return None;
        }

        let chain: Vec<String> = self
            .frames
            .iter()
            .map(|f| f.commit_hash.short().to_string())
            .collect();

        Some(chain.join(" -> "))
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

    #[test]
    fn hash_chain_empty_stack_returns_none() {
        let stack = BlameStack::new();
        assert!(stack.hash_chain().is_none());
    }

    #[test]
    fn hash_chain_single_frame_returns_none() {
        let mut stack = BlameStack::new();
        stack.push(BlameFrame {
            file_path: "test.rs".into(),
            commit_hash: CommitHash::new("abc123def456789012345678901234567890abcd".to_string()),
            entries: vec![],
            selected_line: 0,
        });
        assert!(stack.hash_chain().is_none());
    }

    #[test]
    fn hash_chain_two_frames_returns_chain() {
        let mut stack = BlameStack::new();
        stack.push(BlameFrame {
            file_path: "test.rs".into(),
            commit_hash: CommitHash::new("1111111111111111111111111111111111111111".to_string()),
            entries: vec![],
            selected_line: 0,
        });
        stack.push(BlameFrame {
            file_path: "test.rs".into(),
            commit_hash: CommitHash::new("2222222222222222222222222222222222222222".to_string()),
            entries: vec![],
            selected_line: 0,
        });

        let chain = stack.hash_chain().unwrap();
        assert_eq!(chain, "1111111 -> 2222222");
    }

    #[test]
    fn hash_chain_multiple_frames_returns_full_chain() {
        let mut stack = BlameStack::new();
        let hashes = [
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "cccccccccccccccccccccccccccccccccccccccc",
            "dddddddddddddddddddddddddddddddddddddddd",
        ];

        for hash in hashes {
            stack.push(BlameFrame {
                file_path: "test.rs".into(),
                commit_hash: CommitHash::new(hash.to_string()),
                entries: vec![],
                selected_line: 0,
            });
        }

        let chain = stack.hash_chain().unwrap();
        assert_eq!(chain, "aaaaaaa -> bbbbbbb -> ccccccc -> ddddddd");
    }

    proptest! {
        #[test]
        fn hash_chain_contains_all_hashes(frames in proptest::collection::vec(arbitrary_frame(), 2..10)) {
            let mut stack = BlameStack::new();
            for frame in &frames {
                stack.push(frame.clone());
            }

            let chain = stack.hash_chain().unwrap();

            // Verify all short hashes are in the chain
            for frame in &frames {
                let short_hash = frame.commit_hash.short();
                prop_assert!(chain.contains(short_hash));
            }

            // Verify the chain has the correct number of arrows
            let arrow_count = chain.matches(" -> ").count();
            prop_assert_eq!(arrow_count, frames.len() - 1);
        }
    }
}
