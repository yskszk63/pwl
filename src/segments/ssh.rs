use std::env::var_os;
use std::io::Result;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;

pub fn write_ssh(p: &mut impl SegmentTarget) -> Result<()> {
    if var_os("SSH_CLIENT").is_some() {
        let (fg, bg) = (Color::SshFg, Color::SshBg);
        p.append(Segment::new("SSH", fg, bg))
    } else {
        Ok(())
    }
}
