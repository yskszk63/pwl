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

    pub fn write_symbol(&self, w: &mut (impl Write), symbol: &Symbol, padding: bool) -> Result<()> {
        match self {
            Shell::Bash if padding => write!(w, " {} ", bash_symbol(symbol)),
            Shell::Bash => write!(w, "{}", bash_symbol(symbol)),
        }
    }
}

fn bash_symbol(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Separator => "\u{E0B0}",
        Symbol::SeparatorThin => "\u{E0B1}",

        Symbol::Root => "\\$",

        Symbol::Hostname => "\\h",
        Symbol::Username => "\\u",
    }
}
