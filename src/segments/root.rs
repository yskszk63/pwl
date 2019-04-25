use std::io::Result;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::shell::Shell;

pub fn write_root(p: &mut impl SegmentTarget) -> Result<()> {
    let t = match p.shell() {
        Shell::Bash => "\\$",
    };
    let (fg, bg) = match p.last_exit_status() {
        Some(0) | None => (Color::CmdPassedFg, Color::CmdPassedBg),
        _ => (Color::CmdFailedFg, Color::CmdFailedBg),
    };
    p.append(Segment::new(t, fg, bg))
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
