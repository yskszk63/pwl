use std::io::Result;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::symbol::Symbol;

pub fn write_root(p: &mut impl SegmentTarget, last_exit_status: Option<i32>) -> Result<()> {
    let (fg, bg) = match last_exit_status {
        Some(0) | None => (Color::CmdPassedFg, Color::CmdPassedBg),
        _ => (Color::CmdFailedFg, Color::CmdFailedBg),
    };
    p.append(Segment::symbol(Symbol::Root, fg, bg))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::segments::SegmentContent;

    impl <F: FnMut(Segment)> SegmentTarget for F {
        fn append(&mut self, seg: Segment) -> Result<()> {
            self(seg);
            Ok(())
        }
    }

    #[test]
    pub fn test_none() {
        write_root(&mut |seg: Segment<'_> | {
            if let (SegmentContent::Symbol(sym), fg, bg) = seg.parts() {
                assert_eq!(sym, Symbol::Root);
                assert_eq!(fg, Color::CmdPassedFg);
                assert_eq!(bg, Color::CmdPassedBg);
            } else {
                panic!()
            }
        }, None).unwrap();
    }

    #[test]
    pub fn test_zero() {
        write_root(&mut |seg: Segment<'_> | {
            if let (SegmentContent::Symbol(sym), fg, bg) = seg.parts() {
                assert_eq!(sym, Symbol::Root);
                assert_eq!(fg, Color::CmdPassedFg);
                assert_eq!(bg, Color::CmdPassedBg);
            } else {
                panic!()
            }
        }, Some(0)).unwrap();
    }

    #[test]
    pub fn test_err() {
        write_root(&mut |seg: Segment<'_> | {
            if let (SegmentContent::Symbol(sym), fg, bg) = seg.parts() {
                assert_eq!(sym, Symbol::Root);
                assert_eq!(fg, Color::CmdFailedFg);
                assert_eq!(bg, Color::CmdFailedBg);
            } else {
                panic!()
            }
        }, Some(-1)).unwrap();
    }
}
