#[derive(Debug, Clone)]
pub enum Color {
    Cwd,
    CmdSuccess,
    CmdFailure,
    GitBranch,
    GitAheadBehind,
    GitStaged,
    GitNotStaged,
    GitUntracked,
    GitConflicted,
}
