use std::io::Result;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::symbol::Symbol;

pub fn write_hostname(p: &mut impl SegmentTarget) -> Result<()> {
    let (fg, bg) = (Color::HostnameFg, Color::HostnameBg);
    p.append(Segment::symbol(Symbol::Hostname, fg, bg))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::segments::SegmentContent;

    #[test]
    pub fn test_none() {
        write_hostname(&mut |seg: Segment<'_> | {
            if let (SegmentContent::Symbol(sym), fg, bg) = seg.parts() {
                assert_eq!(sym, Symbol::Hostname);
                assert_eq!(fg, Color::HostnameFg);
                assert_eq!(bg, Color::HostnameBg);
            } else {
                panic!()
            }
        }).unwrap();
    }

}
