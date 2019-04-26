use std::io::Result;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::symbol::Symbol;

pub fn write_root(p: &mut impl SegmentTarget, last_exit_status: &Option<i32>) -> Result<()> {
    let (fg, bg) = match last_exit_status {
        Some(0) | None => (Color::CmdPassedFg, Color::CmdPassedBg),
        _ => (Color::CmdFailedFg, Color::CmdFailedBg),
    };
    p.append(Segment::symbol(Symbol::Root, fg, bg))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shell::Shell;

    #[test]
    pub fn test() {
        let t = crate::theme::Theme::default();
    }
}
