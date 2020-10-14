use super::*;
use git2::ErrorCode;

pub fn render(env: &mut Environment) -> anyhow::Result<Option<Segment>> {
    if let Some(repo) = &env.repo {
        let name = match repo.head() {
            Ok(head) => {
                if repo.head_detached()? {
                    "?".to_owned()
                } else if let Some(name) = head.shorthand() {
                    name.to_owned()
                } else {
                    return Ok(None);
                }
            }
            Err(err) if err.code() == ErrorCode::UnbornBranch => "?".to_owned(),
            Err(err) => return Err(err.into()),
        };
        let summary = env.summarize_git();
        let staged = summary.filter(|s| s.staged() > 0).map(|_| "\u{f067}");
        let notstaged = summary.filter(|s| s.notstaged() > 0).map(|_| "\u{f444}");
        let untracked = summary.filter(|s| s.untracked() > 0).map(|_| "\u{f128}");
        let conflicted = summary.filter(|s| s.conflicted() > 0).map(|_| "\u{f069}");

        let color = match (staged, notstaged, untracked, conflicted) {
            (None, None, None, None) => Color::GitBranch,
            _ => Color::GitBranchDirty,
        };
        Ok(Some(
            Segment::builder(
                color,
                format!(
                    "\u{e0a0}{}{}{}{}{}",
                    name,
                    staged.unwrap_or_default(),
                    notstaged.unwrap_or_default(),
                    untracked.unwrap_or_default(),
                    conflicted.unwrap_or_default(),
                ),
            )
            .build(),
        ))
    } else {
        Ok(None)
    }
}
