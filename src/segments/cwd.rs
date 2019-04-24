use std::env::current_dir;
use std::io::{Result, Write};
use std::path::Component;

use dirs::home_dir;

use super::Segment;
use crate::color::Color;
use crate::powerline::Powerline;

pub fn write_cwd<'a, W: Write>(p: &mut Powerline<'a, W>, short: bool) -> Result<()> {
    if short {
        write_cwd_short(p)
    } else {
        write_cwd_full(p)
    }
}

fn write_cwd_short<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
    let cwd = current_dir()?;
    let cwd = cwd.as_path();

    if let Some(home) = home_dir() {
        if home == cwd {
            let (fg, bg) = (Color::HomeFg, Color::HomeBg);
            p.add(Segment::new("~", fg, bg))?;
            return Ok(());
        }
    }

    if let Some(path) = cwd.file_name() {
        let path = path.to_string_lossy();
        let (fg, bg) = (Color::CwdFg, Color::PathBg);
        p.add(Segment::new(&path, fg, bg))?;
    } else {
        let path = format!("{}", cwd.display());
        let (fg, bg) = (Color::CwdFg, Color::PathBg);
        p.add(Segment::new(&path, fg, bg))?;
    }

    Ok(())
}

fn write_cwd_full<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
    let cwd = current_dir()?;
    let mut cwd = cwd.as_path();
    if let Some(home) = home_dir() {
        if cwd.starts_with(&home) {
            if let Ok(striped) = cwd.strip_prefix(home) {
                cwd = &striped;
                let (fg, bg) = (Color::HomeFg, Color::HomeBg);
                p.add(Segment::new("~", fg, bg))?;
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
                p.add(Segment::new(&prefix, fg, bg))?;
            }
            Component::RootDir => {}
            Component::CurDir => {}
            Component::ParentDir => {
                let (fg, bg) = match iter.peek() {
                    Some(_) => (Color::PathFg, Color::PathBg),
                    None => (Color::CwdFg, Color::PathBg),
                };
                p.add(Segment::new("..", fg, bg))?;
            }
            Component::Normal(path) => {
                let path = path.to_string_lossy();
                let (fg, bg) = match iter.peek() {
                    Some(_) => (Color::PathFg, Color::PathBg),
                    None => (Color::CwdFg, Color::PathBg),
                };
                p.add(Segment::new(&path, fg, bg))?;
            }
        }
    }
    Ok(())
}
