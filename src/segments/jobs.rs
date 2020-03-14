use std::fs::File;
use std::io::{Read, Result};

use nix::unistd::getppid;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;

fn write_jobs_from(children: &mut impl Read, p: &mut impl SegmentTarget) -> Result<()> {
    let mut data = String::new();
    children.read_to_string(&mut data)?;
    let num_children = data.chars().filter(|x| x == &' ').count() - 1;

    if num_children > 0 {
        p.append(Segment::new(
            &format!("{}", num_children),
            Color::JobsFg,
            Color::JobsBg,
        ))
    } else {
        Ok(())
    }
}

pub fn write_jobs(p: &mut impl SegmentTarget) -> Result<()> {
    let ppid = getppid();
    let mut children = File::open(format!("/proc/{}/task/{}/children", ppid, ppid))?;
    write_jobs_from(&mut children, p)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::segments::SegmentContent;

    #[test]
    pub fn test_self_only() {
        let mut children = &b"100 101"[..]; // 100->shell, 101->self

        write_jobs_from(&mut children, &mut |_seg: Segment<'_>| unreachable!()).unwrap();
    }

    #[test]
    pub fn test_any() {
        let mut children = &b"100 101 102"[..]; // 100->shell, 101->other, 102->self

        write_jobs_from(&mut children, &mut |seg: Segment<'_>| {
            if let (SegmentContent::Text(text), fg, bg) = seg.parts() {
                assert_eq!(text, "1");
                assert_eq!(fg, Color::JobsFg);
                assert_eq!(bg, Color::JobsBg);
            } else {
                unreachable!()
            }
        })
        .unwrap();
    }

    #[test]
    pub fn test_none() {
        write_jobs(&mut |_seg: Segment<'_>| panic!()).unwrap();
    }
}
