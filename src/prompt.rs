use std::io;

pub struct BashPromptWrite<W>
where
    W: io::Write,
{
    io: W,
    enter_non_printing: bool,
}

impl<W> BashPromptWrite<W>
where
    W: io::Write,
{
    pub fn new(io: W) -> Self {
        Self {
            io,
            enter_non_printing: false,
        }
    }

    pub fn write_text(&mut self, text: &str) -> io::Result<()> {
        self.ensure_exit_non_printing()?;
        self.io.write_all(text.as_bytes())
    }

    pub fn write_fg(&mut self, color: u8) -> io::Result<()> {
        self.ensure_begin_non_printing()?;
        write!(self.io, "\\e[38;5;{}m", color)?;
        Ok(())
    }

    pub fn write_bg(&mut self, color: u8) -> io::Result<()> {
        self.ensure_begin_non_printing()?;
        write!(self.io, "\\e[48;5;{}m", color)?;
        Ok(())
    }

    pub fn write_reset(&mut self) -> io::Result<()> {
        self.ensure_begin_non_printing()?;
        write!(self.io, "\\e[m")?;
        Ok(())
    }

    fn ensure_begin_non_printing(&mut self) -> io::Result<()> {
        if !self.enter_non_printing {
            self.io.write_all(b"\\[")?;
            self.enter_non_printing = true
        }
        Ok(())
    }

    fn ensure_exit_non_printing(&mut self) -> io::Result<()> {
        if self.enter_non_printing {
            self.io.write_all(b"\\]")?;
            self.enter_non_printing = false
        }
        Ok(())
    }
}

impl<W> Drop for BashPromptWrite<W>
where
    W: io::Write,
{
    fn drop(&mut self) {
        self.ensure_exit_non_printing().ok();
    }
}
