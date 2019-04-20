#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    UsernameFg,
    UsernameBg,
    UsernameRootBg,

    HostnameFg,
    HostnameBg,

    CmdPassedFg,
    CmdPassedBg,
    CmdFailedFg,
    CmdFailedBg,

    HomeFg,
    HomeBg,
    PathFg,
    PathBg,
    CwdFg,

    SshFg,
    SshBg,

    RepoCleanFg,
    RepoCleanBg,
    RepoDirtyFg,
    RepoDirtyBg,

    GitAheadFg,
    GitAheadBg,
    GitBehindFg,
    GitBehindBg,

    GitStagedFg,
    GitStagedBg,
    GitNotstagedFg,
    GitNotstagedBg,
    GitUntrackedFg,
    GitUntrackedBg,
    GitConflictedFg,
    GitConflictedBg,

    JobsFg,
    JobsBg,

    VirtualenvFg,
    VirtualenvBg,
}
