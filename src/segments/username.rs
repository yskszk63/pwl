use std::io::{Result, Write};

use nix::unistd::Uid;

use super::Segment;
use crate::color::Color;
use crate::powerline::Powerline;
use crate::shell::Shell;

pub fn write_username<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
    let t = match p.shell() {
        Shell::Bash => "\\u",
    };
    let (fg, bg) = if Uid::current().is_root() {
        (Color::UsernameFg, Color::UsernameRootBg)
    } else {
        (Color::UsernameFg, Color::UsernameBg)
    };
    p.add(Segment::new(t, fg, bg))
}
