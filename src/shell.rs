use std::io::{Result, Write};

use crate::symbol::Symbol;

#[derive(Debug)]
pub enum Shell {
    Bash,
}

impl Shell {
    pub fn write_fg(&self, w: &mut (dyn Write), color: u8) -> Result<()> {
        match self {
            Shell::Bash => write!(w, "\\[\\e[38;5;{}m\\]", color),
        }
    }

    pub fn write_bg(&self, w: &mut (dyn Write), color: u8) -> Result<()> {
        match self {
            Shell::Bash => write!(w, "\\[\\e[48;5;{}m\\]", color),
        }
    }

    pub fn write_reset(&self, w: &mut (dyn Write)) -> Result<()> {
        match self {
            Shell::Bash => write!(w, "\\[\\e[0m\\]"),
        }
    }

    pub fn symbol(&self, symbol: &Symbol) -> &'static str {
        match self {
            Shell::Bash => bash_symbol(symbol),
        }
    }
}

fn bash_symbol(symbol: &Symbol) -> &'static str {
    match symbol {
        Symbol::Separator => "\u{E0B0}",
        Symbol::SeparatorThin => "\u{E0B1}",

        Symbol::Root => "\\$",

        Symbol::Hostname => "\\h",
        Symbol::Username => "\\u",

        Symbol::GitAhead => "\u{2B06}",
        Symbol::GitBehind => "\u{2B07}",

        Symbol::GitStaged => "\u{2714}",
        Symbol::GitNotstaged => "\u{270E}",
        Symbol::GitUntracked => "+",
        Symbol::GitConflicted => "*",
    }
}
