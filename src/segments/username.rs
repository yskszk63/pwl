use std::io::Result;

use nix::unistd::Uid;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::symbol::Symbol;

pub fn write_username(p: &mut impl SegmentTarget) -> Result<()> {
    let (fg, bg) = if Uid::current().is_root() {
        (Color::UsernameFg, Color::UsernameRootBg)
    } else {
        (Color::UsernameFg, Color::UsernameBg)
    };
    p.append(Segment::symbol(Symbol::Username, fg, bg))
}
