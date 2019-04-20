use crate::color::Color;

pub use cwd::*;
pub use git::*;
pub use hostname::*;
pub use jobs::*;
pub use root::*;
pub use ssh::*;
pub use username::*;
pub use virtualenv::*;

mod cwd;
mod git;
mod hostname;
mod jobs;
mod root;
mod ssh;
mod username;
mod virtualenv;

#[derive(Debug)]
pub enum Segments {
    Root,
    Cwd,
    Jobs,
    Virtualenv,
    Username,
    Hostname,
    Ssh,
    Git,
}

#[derive(Debug)]
pub struct Segment<'a> {
    fg: Color,
    bg: Color,
    text: &'a str,
}

impl<'a> Segment<'a> {
    pub fn new(text: &'a str, fg: Color, bg: Color) -> Segment<'a> {
        Segment { text, fg, bg }
    }

    pub fn parts(self) -> (&'a str, Color, Color) {
        (self.text, self.fg, self.bg)
    }
}
