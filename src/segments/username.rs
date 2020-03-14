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

#[cfg(test)]
mod test {
    use super::*;
    use crate::segments::SegmentContent;

    #[test]
    pub fn test_none() {
        write_username(&mut |seg: Segment<'_>| {
            if let (SegmentContent::Symbol(sym), fg, bg) = seg.parts() {
                assert_eq!(sym, Symbol::Username);
                assert_eq!(fg, Color::UsernameFg);
                assert_eq!(bg, Color::UsernameBg);
            } else {
                panic!()
            }
        })
        .unwrap();
    }
}
