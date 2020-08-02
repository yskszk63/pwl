use crate::segment::Group;
use git2::BranchType;
use super::*;

pub fn render(env: &Environment) -> anyhow::Result<Option<Segment>> {
    if let Some(repo) = &env.repo {
        let head = repo.head()?;
        let local = head.target();
        let name = head.shorthand();
        if let (Some(local), Some(name)) = (local, name) {
            let branch = repo.find_branch(name, BranchType::Local)?;
            if let Ok(upstream) = branch.upstream() {
                if let Some(upstream) = upstream.get().target() {
                    let (ahead, behind) = repo.graph_ahead_behind(local, upstream)?;
                    let sym = match (ahead, behind) {
                        (ahead, behind) if ahead > 0 && behind > 0 => Some(""),
                        (ahead, _) if ahead > 0 => Some(""),
                        (_, behind) if behind > 0 => Some(""),
                        _ => None,
                    };
                    if let Some(sym) = sym {
                        return Ok(Some(Segment::builder(Color::GitAheadBehind, sym).group(Group::GitStatus).build()))
                    }
                }
            }
        }
    }
    Ok(None)
}
