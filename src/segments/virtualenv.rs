use std::env::var_os;
use std::ffi::OsStr;
use std::io::{Result, Write};
use std::path::PathBuf;

use super::Segment;
use crate::color::Color;
use crate::powerline::Powerline;

pub fn write_virtualenv<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
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
        p.add(Segment::new(&env_name.to_string_lossy(), fg, bg))
    } else {
        Ok(())
    }
}
