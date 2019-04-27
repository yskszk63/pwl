use std::io::{Result, Write};

use crate::symbol::Symbol;

#[derive(Debug)]
pub enum Shell {
    Bash,
}

impl Shell {
    pub fn write_fg(&self, w: &mut (impl Write), color: u8) -> Result<()> {
        match self {
            Shell::Bash => write!(w, "\\[\\e[38;5;{}m\\]", color),
        }
    }

    pub fn write_bg(&self, w: &mut (impl Write), color: u8) -> Result<()> {
        match self {
            Shell::Bash => write!(w, "\\[\\e[48;5;{}m\\]", color),
        }
    }

    pub fn write_reset(&self, w: &mut (impl Write)) -> Result<()> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_bash() {
        let mut buf = vec![];
        Shell::Bash.write_fg(&mut buf, 0).unwrap();
        // TODO assert

        let mut buf = vec![];
        Shell::Bash.write_bg(&mut buf, 0).unwrap();
        // TODO assert

        let mut buf = vec![];
        Shell::Bash.write_reset(&mut buf).unwrap();
        // TODO assert

        Shell::Bash.symbol(&Symbol::Separator);
        Shell::Bash.symbol(&Symbol::SeparatorThin);
        Shell::Bash.symbol(&Symbol::Root);
        Shell::Bash.symbol(&Symbol::Hostname);
        Shell::Bash.symbol(&Symbol::Username);
        Shell::Bash.symbol(&Symbol::GitAhead);
        Shell::Bash.symbol(&Symbol::GitBehind);
        Shell::Bash.symbol(&Symbol::GitStaged);
        Shell::Bash.symbol(&Symbol::GitNotstaged);
        Shell::Bash.symbol(&Symbol::GitUntracked);
        Shell::Bash.symbol(&Symbol::GitConflicted);
    }
}
