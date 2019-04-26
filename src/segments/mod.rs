use crate::color::Color;

use crate::symbol::Symbol;
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
pub enum SegmentContent<'a> {
    Symbol(Symbol),
    Text(&'a str),
}

#[derive(Debug)]
pub struct Segment<'a> {
    fg: Color,
    bg: Color,
    content: SegmentContent<'a>,
}

impl<'a> Segment<'a> {
    pub fn new(text: &'a str, fg: Color, bg: Color) -> Segment<'a> {
        let content = SegmentContent::Text(text);
        Segment { content, fg, bg }
    }

    pub fn symbol(symbol: Symbol, fg: Color, bg: Color) -> Segment<'a> {
        let content = SegmentContent::Symbol(symbol);
        Segment { content, fg, bg }
    }

    pub fn parts(self) -> (SegmentContent<'a>, Color, Color) {
        (self.content, self.fg, self.bg)
    }
}
