use super::*;
use git2::ErrorCode;

pub fn render(env: &Environment) -> anyhow::Result<Option<Segment>> {
    if let Some(repo) = &env.repo {
        if repo.head_detached()? {
            return Ok(Some(
                Segment::builder(Color::GitBranch, format!("{}", "?")).build(),
            ));
        }
        let name = match repo.head() {
            Ok(head) => {
                if let Some(name) = head.shorthand() {
                    name.to_owned()
                } else {
                    return Ok(None);
                }
            }
            Err(err) if err.code() == ErrorCode::UnbornBranch => "?".to_owned(),
            Err(err) => Err(err)?,
        };
        return Ok(Some(
            Segment::builder(Color::GitBranch, format!("{}", name)).build(),
        ));
    }
    Ok(None)
}
