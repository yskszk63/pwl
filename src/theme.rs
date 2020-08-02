use crate::Color;

#[derive(Debug)]
pub enum Theme {
    Light,
}

impl Theme {
    pub fn resolve(&self, color: &Color) -> (u8, u8) {
        match self {
            Self::Light => resolve_light(color),
        }
    }
}

fn resolve_light(color: &Color) -> (u8, u8) {
    match color {
        Color::Cwd => (0, 7),
        Color::CmdSuccess => (15, 2),
        Color::CmdFailure => (15, 1),
        Color::GitBranch => (1, 15),
        Color::GitAheadBehind => (250, 240),
        Color::GitStaged => (15, 22),
        Color::GitNotStaged => (15, 130),
        Color::GitUntracked => (15, 52),
        Color::GitConflicted => (15, 9),
    }
}
