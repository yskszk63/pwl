use std::io::Result;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::shell::Shell;

pub fn write_hostname(p: &mut impl SegmentTarget) -> Result<()> {
    let t = match p.shell() {
        Shell::Bash => "\\h",
    };
    let (fg, bg) = (Color::HostnameFg, Color::HostnameBg);
    p.append(Segment::new(t, fg, bg))
}
