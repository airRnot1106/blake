mod blame;
mod diff;
mod global;
mod help;

pub use blame::BlameAction;
pub use diff::DiffAction;
pub use global::GlobalAction;
pub use help::HelpAction;

/// Unified action type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Global(GlobalAction),
    Blame(BlameAction),
    Diff(DiffAction),
    Help(HelpAction),
    None,
}
