use std::env::current_dir;
use std::io::Result;
use std::path::{Component, PathBuf};

use dirs::home_dir;

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;

struct Env {
    cwd: PathBuf,
    home: Option<PathBuf>,
}

impl Env {
    fn new() -> Option<Env> {
        let cwd = current_dir();
        let home = home_dir();
        let (cwd, home) = match (cwd, home) {
            (Ok(cwd), home) => (cwd, home),
            _ => return None,
        };
        Some(Env { cwd, home })
    }
}

pub fn write_cwd(p: &mut impl SegmentTarget, short: bool) -> Result<()> {
    let env = match Env::new() {
        Some(env) => env,
        _ => return Ok(()),
    };

    if short {
        write_cwd_short(p, env)
    } else {
        write_cwd_full(p, env)
    }
}

fn write_cwd_short(p: &mut impl SegmentTarget, env: Env) -> Result<()> {
    let cwd = env.cwd.as_path();

    if let Some(home) = env.home {
        if home == cwd {
            let (fg, bg) = (Color::HomeFg, Color::HomeBg);
            p.append(Segment::new("~", fg, bg))?;
            return Ok(());
        }
    }

    if let Some(path) = cwd.file_name() {
        let path = path.to_string_lossy();
        let (fg, bg) = (Color::CwdFg, Color::PathBg);
        p.append(Segment::new(&path, fg, bg))?;
    } else {
        let path = format!("{}", cwd.display());
        let (fg, bg) = (Color::CwdFg, Color::PathBg);
        p.append(Segment::new(&path, fg, bg))?;
    }

    Ok(())
}

fn write_cwd_full(p: &mut impl SegmentTarget, env: Env) -> Result<()> {
    let mut cwd = env.cwd.as_path();

    if let Some(home) = env.home {
        if cwd.starts_with(&home) {
            if let Ok(striped) = cwd.strip_prefix(home) {
                cwd = &striped;
                let (fg, bg) = (Color::HomeFg, Color::HomeBg);
                p.append(Segment::new("~", fg, bg))?;
            }
        }
    };

    let mut iter = cwd.components().peekable();
    while let Some(item) = iter.next() {
        match item {
            Component::Prefix(prefix) => {
                let prefix = prefix.as_os_str().to_string_lossy();
                let (fg, bg) = match iter.peek() {
                    Some(_) => (Color::PathFg, Color::PathBg),
                    None => (Color::CwdFg, Color::PathBg),
                };
                p.append(Segment::new(&prefix, fg, bg))?;
            }
            Component::RootDir => {}
            Component::CurDir => {}
            Component::ParentDir => {
                let (fg, bg) = match iter.peek() {
                    Some(_) => (Color::PathFg, Color::PathBg),
                    None => (Color::CwdFg, Color::PathBg),
                };
                p.append(Segment::new("..", fg, bg))?;
            }
            Component::Normal(path) => {
                let path = path.to_string_lossy();
                let (fg, bg) = match iter.peek() {
                    Some(_) => (Color::PathFg, Color::PathBg),
                    None => (Color::CwdFg, Color::PathBg),
                };
                p.append(Segment::new(&path, fg, bg))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::segments::SegmentContent;

    #[test]
    pub fn test() {
        write_cwd(&mut |_seg: Segment<'_>| {}, true).unwrap();
        write_cwd(&mut |_seg: Segment<'_>| {}, false).unwrap();
    }

    #[test]
    pub fn test_home_full() {
        let home = home_dir();
        let env = Env {
            cwd: home.clone().unwrap(),
            home: home,
        };

        write_cwd_full(
            &mut |seg: Segment<'_>| {
                if let (SegmentContent::Text(text), fg, bg) = seg.parts() {
                    assert_eq!(text, "~");
                    assert_eq!(fg, Color::HomeFg);
                    assert_eq!(bg, Color::HomeBg);
                } else {
                    panic!()
                }
            },
            env,
        )
        .unwrap();
    }

    #[test]
    pub fn test_home_short() {
        let home = home_dir();
        let env = Env {
            cwd: home.clone().unwrap(),
            home: home,
        };

        write_cwd_short(
            &mut |seg: Segment<'_>| {
                if let (SegmentContent::Text(text), fg, bg) = seg.parts() {
                    assert_eq!(text, "~");
                    assert_eq!(fg, Color::HomeFg);
                    assert_eq!(bg, Color::HomeBg);
                } else {
                    panic!()
                }
            },
            env,
        )
        .unwrap();
    }

    #[test]
    pub fn test_short_nofilename() {
        let cwd = PathBuf::from("/foo/..");
        let home = home_dir();
        let env = Env {
            cwd: cwd,
            home: home,
        };

        write_cwd_short(
            &mut |seg: Segment<'_>| {
                if let (SegmentContent::Text(text), fg, bg) = seg.parts() {
                    assert_eq!(text, "/foo/..");
                    assert_eq!(fg, Color::CwdFg);
                    assert_eq!(bg, Color::PathBg);
                } else {
                    panic!()
                }
            },
            env,
        )
        .unwrap();
    }

    #[test]
    pub fn test_full_withouthome() {
        let cwd = PathBuf::from("/foo/../hoge");
        let home = home_dir();
        let env = Env {
            cwd: cwd,
            home: home,
        };

        let mut n = 0;
        write_cwd_full(
            &mut |seg: Segment<'_>| match (n, seg.parts()) {
                (0, (SegmentContent::Text("foo"), Color::PathFg, Color::PathBg)) => n = n + 1,
                (1, (SegmentContent::Text(".."), Color::PathFg, Color::PathBg)) => n = n + 1,
                (2, (SegmentContent::Text("hoge"), Color::CwdFg, Color::PathBg)) => n = n + 1,
                x => panic!("{:?}", x),
            },
            env,
        )
        .unwrap();
    }

}
