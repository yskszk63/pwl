use std::io;

pub use segment::Segment;
pub use color::Color;
use module::{Environment, Module};
use theme::Theme;
use prompt::BashPromptWrite;

mod segment;
mod color;
mod prompt;
mod module;
mod theme;

fn main() -> anyhow::Result<()> {
    let modules = [
        Module::Cwd,
        Module::GitBranch,
        Module::GitAheadBehind,
        Module::GitStaged,
        Module::GitNotStaged,
        Module::GitUntracked,
        Module::GitConflicted,
        Module::Root,
    ];
    let theme = Theme::Light;
    let mut env = Environment::new(0);

    let stdout = io::stdout();
    let mut prompt = BashPromptWrite::new(io::BufWriter::new(stdout.lock()));

    let mut previous = None;
    let mut previous_group = None;
    for module in &modules[..] {
        if let Some(segment) = module.render(&mut env)? {
            let (fg, bg) = theme.resolve(&segment.color());
            if segment.group().is_none() || segment.group() != previous_group.as_ref() {
                if let Some((_, previous)) = previous {
                    prompt.write_fg(previous)?;
                    prompt.write_bg(bg)?;
                    prompt.write_text("")?;
                }
            }

            prompt.write_fg(fg)?;
            prompt.write_bg(bg)?;
            prompt.write_text(&format!("{} ", segment.text()))?;

            previous = Some((fg, bg));
            previous_group = segment.group().cloned();
        }
    }

    if let Some((_, previous)) = previous {
        prompt.write_reset()?;
        prompt.write_fg(previous)?;
        prompt.write_text(" ")?;
    }
    prompt.write_reset()?;
    Ok(())
}
