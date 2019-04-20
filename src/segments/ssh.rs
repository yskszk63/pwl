use std::env::var_os;
use std::io::{Result, Write};

use super::Segment;
use crate::color::Color;
use crate::powerline::Powerline;

pub fn write_ssh<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
    if var_os("SSH_CLIENT").is_some() {
        let (fg, bg) = (Color::SshFg, Color::SshBg);
        p.add(Segment::new("SSH", fg, bg))
    } else {
        Ok(())
    }
}
