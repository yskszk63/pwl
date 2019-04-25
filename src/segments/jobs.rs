use std::fs::File;
use std::io::{Read, Result};

use nix::unistd::getppid;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;

pub fn write_jobs(p: &mut impl SegmentTarget) -> Result<()> {
    let ppid = getppid();
    let mut children = File::open(format!("/proc/{}/task/{}/children", ppid, ppid))?;
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
