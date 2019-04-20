use std::io::{Result, Write};

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
}
