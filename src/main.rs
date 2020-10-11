use std::env;
use std::io;

pub use color::Color;
use module::{Environment, Module};
use prompt::BashPromptWrite;
pub use segment::Segment;
use theme::Theme;

mod color;
mod module;
mod prompt;
mod segment;
mod theme;

fn main() -> anyhow::Result<()> {
    let exit_code = env::args().nth(1).map_or(Ok(0), |v| v.parse()).unwrap_or(0);
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
    let mut env = Environment::new(exit_code);

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
            prompt.write_text(&format!(" {} ", segment.text()))?;

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
