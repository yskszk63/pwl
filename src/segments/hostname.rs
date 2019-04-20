use std::io::{Result, Write};

use super::Segment;
use crate::color::Color;
use crate::powerline::Powerline;
use crate::shell::Shell;

pub fn write_hostname<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
    let t = match p.shell() {
        Shell::Bash => "\\h",
    };
    let (fg, bg) = (Color::HostnameFg, Color::HostnameBg);
    p.add(Segment::new(t, fg, bg))
}
