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

#[cfg(test)]
mod test {
    use super::*;
    use crate::segments::SegmentContent;
    use std::env::{remove_var, set_var};

    #[test]
    pub fn test_any() {
        set_var("SSH_CLIENT", "x");
        write_ssh(&mut |seg: Segment<'_>| {
            if let (SegmentContent::Text(text), fg, bg) = seg.parts() {
                assert_eq!(text, "SSH");
                assert_eq!(fg, Color::SshFg);
                assert_eq!(bg, Color::SshBg);
            } else {
                unreachable!()
            }
        })
        .unwrap();
    }

    #[test]
    pub fn test_none() {
        remove_var("SSH_CLIENT");
        write_ssh(&mut |_seg: Segment<'_>| panic!()).unwrap();
    }
}
