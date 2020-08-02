use super::*;

pub fn render(env: &Environment) -> anyhow::Result<Option<Segment>> {
    let color = if env.exit_code == 0 {
        Color::CmdSuccess
    } else {
        Color::CmdFailure
    };
    Ok(Some(Segment::builder(color, "\\$").build()))
}
