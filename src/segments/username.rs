use std::io::Result;

use nix::unistd::Uid;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::shell::Shell;

pub fn write_username(p: &mut impl SegmentTarget) -> Result<()> {
    let t = match p.shell() {
        Shell::Bash => "\\u",
    };
    let (fg, bg) = if Uid::current().is_root() {
        (Color::UsernameFg, Color::UsernameRootBg)
    } else {
        (Color::UsernameFg, Color::UsernameBg)
    };
    p.append(Segment::new(t, fg, bg))
}
