use std::io::{Result, Write};

use crate::color::Color;
use crate::segments::{self as seg, Segment, SegmentContent, Segments};
use crate::shell::Shell;
use crate::symbol::Symbol;
use crate::theme::Theme;

pub trait SegmentTarget {
    fn append(&mut self, segment: Segment) -> Result<()>;
}

#[derive(Debug)]
pub struct Powerline<'a, W: Write> {
    last_exit_status: Option<i32>,
    cwd_short: bool,
    theme: Theme,
    shell: Shell,
    output: &'a mut W,
    last_color: Option<(Color, Color)>,
}

impl<'a, W: Write> Powerline<'a, W> {
    pub fn new(
        last_exit_status: Option<i32>,
        cwd_short: bool,
        theme: Theme,
        shell: Shell,
        output: &'a mut W,
    ) -> Powerline<'a, W> {
        Powerline {
            last_exit_status,
            cwd_short,
            theme,
            shell,
            output,
            last_color: None,
        }
    }

    pub fn draw(&mut self, segments: &(dyn AsRef<[Segments]>)) -> Result<()> {
        for seg in segments.as_ref() {
            match seg {
                Segments::Root => seg::write_root(self, self.last_exit_status)?,
                Segments::Cwd => seg::write_cwd(self, self.cwd_short)?,
                Segments::Jobs => seg::write_jobs(self)?,
                Segments::Virtualenv => seg::write_virtualenv(self)?,
                Segments::Username => seg::write_username(self)?,
                Segments::Hostname => seg::write_hostname(self)?,
                Segments::Ssh => seg::write_ssh(self)?,
                Segments::Git => seg::write_git(self)?,
            }
        }

        if let Some((_, last_bg)) = &self.last_color {
            self.shell.write_reset(self.output)?;
            self.shell.write_fg(self.output, self.theme.get(last_bg))?;
            write!(self.output, "{}", self.shell.symbol(&Symbol::Separator))?;
            self.shell.write_reset(self.output)?;
            write!(self.output, " ")?;
        }
        Ok(())
    }

    pub fn add(&mut self, segment: Segment) -> Result<()> {
        let (content, fg, bg) = segment.parts();

        if let Some((last_fg, last_bg)) = &self.last_color {
            if last_bg == &bg {
                self.shell.write_bg(self.output, self.theme.get(&bg))?;
                self.shell.write_fg(self.output, self.theme.get(last_fg))?;
                write!(self.output, "{}", self.shell.symbol(&Symbol::SeparatorThin))?
            } else {
                self.shell.write_bg(self.output, self.theme.get(&bg))?;
                self.shell.write_fg(self.output, self.theme.get(last_bg))?;
                write!(self.output, "{}", self.shell.symbol(&Symbol::Separator))?
            }
        };

        self.shell.write_fg(self.output, self.theme.get(&fg))?;
        self.shell.write_bg(self.output, self.theme.get(&bg))?;
        match content {
            SegmentContent::Text(text) => write!(self.output, " {} ", text)?,
            SegmentContent::Symbol(symbol) => {
                write!(self.output, " {} ", self.shell.symbol(&symbol))?
            }
            SegmentContent::TextSym(text, symbol) => {
                write!(self.output, " {}{} ", text, self.shell.symbol(&symbol))?
            }
        };

        self.last_color = Some((fg, bg));
        Ok(())
    }
}

impl<'a, W: Write> SegmentTarget for Powerline<'a, W> {
    fn append(&mut self, segment: Segment) -> Result<()> {
        self.add(segment)
    }
}
