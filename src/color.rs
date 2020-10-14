#[derive(Debug, Clone)]
pub enum Color {
    Cwd,
    CmdSuccess,
    CmdFailure,
    GitBranch,
    GitBranchDirty,
    GitAheadBehind,
}
