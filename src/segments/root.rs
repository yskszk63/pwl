use std::io::{Result, Write};

use super::Segment;
use crate::color::Color;
use crate::powerline::Powerline;
use crate::shell::Shell;

pub fn write_root<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
    let t = match p.shell() {
        Shell::Bash => "\\$",
    };
    let (fg, bg) = match p.last_exit_status() {
        Some(0) | None => (Color::CmdPassedFg, Color::CmdPassedBg),
        _ => (Color::CmdFailedFg, Color::CmdFailedBg),
    };
    p.add(Segment::new(t, fg, bg))
}
