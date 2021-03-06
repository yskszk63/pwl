use super::*;
use crate::segment::Group;
use git2::{BranchType, ErrorCode};

pub fn render(env: &Environment) -> anyhow::Result<Option<Segment>> {
    if let Some(repo) = &env.repo {
        if repo.head_detached()? {
            return Ok(None);
        }
        let head = match repo.head() {
            Ok(head) => head,
            Err(err) if err.code() == ErrorCode::UnbornBranch => return Ok(None),
            Err(err) => return Err(err.into()),
        };
        let local = head.target();
        let name = head.shorthand();
        if let (Some(local), Some(name)) = (local, name) {
            let branch = repo.find_branch(name, BranchType::Local)?;
            if let Ok(upstream) = branch.upstream() {
                if let Some(upstream) = upstream.get().target() {
                    let (ahead, behind) = repo.graph_ahead_behind(local, upstream)?;
                    let sym = match (ahead, behind) {
                        (ahead, behind) if ahead > 0 && behind > 0 => Some("\u{21e3}\u{21e1}"),
                        (ahead, _) if ahead > 0 => Some("\u{21e1}"),
                        (_, behind) if behind > 0 => Some("\u{21e3}"),
                        _ => None,
                    };
                    if let Some(sym) = sym {
                        return Ok(Some(
                            Segment::builder(Color::GitAheadBehind, sym)
                                .group(Group::GitStatus)
                                .build(),
                        ));
                    }
                }
            }
        }
    }
    Ok(None)
}
