use std::env::var_os;
use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;

pub fn write_virtualenv(p: &mut impl SegmentTarget) -> Result<()> {
    let env = match var_os("VIRTUAL_ENV") {
        Some(env) => {
            let mut path = PathBuf::from(&env);
            if path.file_name() == Some(OsStr::new(".venv")) {
                path.pop();
                Some(path)
            } else {
                Some(path)
            }
        }
        None => var_os("CONDA_ENV_PATH")
            .or_else(|| var_os("CONDA_DEFAULT_ENV"))
            .map(|env| PathBuf::from(&env)),
    };
    if let Some(env_name) = env.and_then(|env| env.file_name().map(OsStr::to_os_string)) {
        let (fg, bg) = (Color::VirtualenvFg, Color::VirtualenvBg);
        p.append(Segment::new(&env_name.to_string_lossy(), fg, bg))
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
        set_var("VIRTUAL_ENV", "x");
        write_virtualenv(&mut |seg: Segment<'_>| {
            if let (SegmentContent::Text(text), fg, bg) = seg.parts() {
                assert_eq!(text, "x");
                assert_eq!(fg, Color::VirtualenvFg);
                assert_eq!(bg, Color::VirtualenvBg);
            } else {
                unreachable!()
            }
        })
        .unwrap();
    }

    #[test]
    pub fn test_dot_venv() {
        set_var("VIRTUAL_ENV", "x/.venv");
        write_virtualenv(&mut |seg: Segment<'_>| {
            if let (SegmentContent::Text(text), fg, bg) = seg.parts() {
                assert_eq!(text, "x");
                assert_eq!(fg, Color::VirtualenvFg);
                assert_eq!(bg, Color::VirtualenvBg);
            } else {
                unreachable!()
            }
        })
        .unwrap();
    }

    #[test]
    pub fn test_none() {
        remove_var("VIRTUAL_ENV");
        write_virtualenv(&mut |_seg: Segment<'_>| panic!()).unwrap();
    }
}
