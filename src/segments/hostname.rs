use std::io::Result;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::symbol::Symbol;

pub fn write_hostname(p: &mut impl SegmentTarget) -> Result<()> {
    let (fg, bg) = (Color::HostnameFg, Color::HostnameBg);
    p.append(Segment::symbol(Symbol::Hostname, fg, bg))
}
