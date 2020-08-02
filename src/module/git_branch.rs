use super::*;

pub fn render(env: &Environment) -> anyhow::Result<Option<Segment>> {
    if let Some(repo) = &env.repo {
        let head = repo.head()?;
        if let Some(name) = head.shorthand() {
            return Ok(Some(Segment::builder(Color::GitBranch, format!("{}", name)).build()))
        }
    }
    Ok(None)
}
