use git2::Repository;
use std::path::Path;
use thiserror::Error;

use crate::domain::{BlameEntry, BlameFrame, CommitHash, CommitInfo, Diff, GitGateway};

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Git error: {0}")]
    Git2(#[from] git2::Error),
}

pub struct Git2Gateway {
    repo: Repository,
}

impl Git2Gateway {
    #[allow(dead_code)]
    pub fn open(path: &Path) -> Result<Self, GitError> {
        let repo = Repository::discover(path)?;
        Ok(Self { repo })
    }

    pub fn open_current() -> Result<Self, GitError> {
        let repo = Repository::discover(".")?;
        Ok(Self { repo })
    }
}

impl GitGateway for Git2Gateway {
    type Error = GitError;

    fn blame(&self, file_path: &Path, commit: &CommitHash) -> Result<BlameFrame, Self::Error> {
        let spec = commit.as_str();
        let commit_obj = self.repo.revparse_single(spec)?.peel_to_commit()?;
        let commit_oid = commit_obj.id();

        let mut opts = git2::BlameOptions::new();
        opts.newest_commit(commit_oid);

        let blame = self.repo.blame_file(file_path, Some(&mut opts))?;

        let mut entries = Vec::new();
        for hunk in blame.iter() {
            let sig = hunk.final_signature();
            let author = sig.name().unwrap_or("Unknown").to_string();
            let timestamp = sig.when().seconds();
            let hunk_commit = hunk.final_commit_id();

            let blob = commit_obj
                .tree()?
                .get_path(file_path)?
                .to_object(&self.repo)?;
            let content = blob
                .as_blob()
                .map(|b| String::from_utf8_lossy(b.content()).to_string())
                .unwrap_or_default();

            let lines: Vec<&str> = content.lines().collect();
            let start_line = hunk.final_start_line();
            let line_count = hunk.lines_in_hunk();

            for line_offset in 0..line_count {
                let line_number = start_line + line_offset;
                // Convert 1-based line number to 0-based index
                let line_content = lines
                    .get(line_number.saturating_sub(1))
                    .unwrap_or(&"")
                    .to_string();

                entries.push(BlameEntry {
                    line_number,
                    commit_hash: CommitHash::new(hunk_commit.to_string()),
                    author: author.clone(),
                    timestamp,
                    content: line_content,
                })
            }
        }

        entries.sort_by_key(|e| e.line_number);

        Ok(BlameFrame {
            file_path: file_path.to_path_buf(),
            commit_hash: CommitHash::new(commit_oid.to_string()),
            entries,
            selected_line: 0,
        })
    }

    fn diff(&self, commit: &CommitHash) -> Result<Diff, Self::Error> {
        let spec = commit.as_str();
        let commit_obj = self.repo.revparse_single(spec)?.peel_to_commit()?;

        // Get first parent (index 0); merge commits may have multiple parents
        let parent = commit_obj.parent(0).ok();
        let parent_tree = parent.as_ref().map(|p| p.tree()).transpose()?;
        let commit_tree = commit_obj.tree()?;

        let diff = self
            .repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&commit_tree), None)?;

        let mut diff_text = Vec::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            // Include the origin character (+, -, space, etc.) for diff lines
            let origin = line.origin();
            if origin == '+' || origin == '-' || origin == ' ' {
                diff_text.push(origin as u8);
            }
            diff_text.extend_from_slice(line.content());
            true
        })?;

        let content = String::from_utf8_lossy(&diff_text).to_string();
        Ok(Diff::new(content))
    }

    fn commit_info(&self, commit: &CommitHash) -> Result<CommitInfo, Self::Error> {
        let spec = commit.as_str();
        let commit_obj = self.repo.revparse_single(spec)?.peel_to_commit()?;

        let sig = commit_obj.author();
        let author = sig.name().unwrap_or("Unknown").to_string();
        let timestamp = sig.when().seconds();
        let message = commit_obj.message().unwrap_or("").to_string();
        let parent = commit_obj
            .parent(0)
            .ok()
            .map(|p| CommitHash::new(p.id().to_string()));

        Ok(CommitInfo {
            hash: CommitHash::new(commit_obj.id().to_string()),
            parent,
            author,
            timestamp,
            message,
        })
    }

    fn github_commit_url(&self, commit: &CommitHash) -> Option<String> {
        // Try to find a GitHub remote
        let remote = self.repo.find_remote("origin").ok()?;
        let url = remote.url()?;

        // Parse GitHub URL from remote
        // Supports: git@github.com:user/repo.git, https://github.com/user/repo.git
        let github_base = if url.starts_with("git@github.com:") {
            let path = url.strip_prefix("git@github.com:")?;
            let path = path.strip_suffix(".git").unwrap_or(path);
            format!("https://github.com/{}", path)
        } else if url.starts_with("https://github.com/") {
            let path = url.strip_prefix("https://github.com/")?;
            let path = path.strip_suffix(".git").unwrap_or(path);
            format!("https://github.com/{}", path)
        } else {
            return None;
        };

        Some(format!("{}/commit/{}", github_base, commit.as_str()))
    }
}
