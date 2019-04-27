#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Separator,
    SeparatorThin,

    Root,

    Hostname,
    Username,

    GitAhead,
    GitBehind,

    GitStaged,
    GitNotstaged,
    GitUntracked,
    GitConflicted,
}
