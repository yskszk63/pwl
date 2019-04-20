use std::io::{Result, Write};

use git2::{BranchType, ObjectType, Oid, Repository, Status, StatusOptions, StatusShow};

use super::Segment;
use crate::color::Color;
use crate::powerline::Powerline;

fn head_info(
    git: &Repository,
) -> std::result::Result<(Option<Oid>, Option<Oid>, Option<String>), &'static str> {
    let branches = match git.branches(Some(BranchType::Local)) {
        Ok(branches) => branches,
        Err(_) => return Err("err"),
    };

    for branch in branches.filter_map(|b| if let Ok((b, _)) = b { Some(b) } else { None }) {
        if branch.is_head() {
            let local = branch.get().target();
            let upstream = branch.upstream().ok().and_then(|b| b.get().target());

            if let Ok(name) = branch.name() {
                return Ok((local, upstream, name.map(|s| s.to_string())));
            }
        }
    }

    if let Ok(head) = git.head() {
        if let Some(target) = head.target() {
            let name = git
                .find_object(target, Some(ObjectType::Any))
                .ok()
                .and_then(|o| o.short_id().ok())
                .and_then(|b| b.as_str().map(|s| s.to_string()));
            return Ok((None, None, name));
        }
    };

    Ok((None, None, None))
}

fn count_stage(git: &Repository) -> Option<(usize, usize, usize, usize)> {
    let statuses = git.statuses(Some(
        StatusOptions::new()
            .show(StatusShow::IndexAndWorkdir)
            .include_untracked(true)
            .renames_from_rewrites(true)
            .renames_head_to_index(true),
    ));
    if let Ok(statuses) = statuses {
        let mut staged = 0;
        let mut notstaged = 0;
        let mut untracked = 0;
        let mut conflicted = 0;

        for status in statuses.iter().map(|entry| entry.status()) {
            match status {
                Status::INDEX_NEW
                | Status::INDEX_MODIFIED
                | Status::INDEX_TYPECHANGE
                | Status::INDEX_RENAMED
                | Status::INDEX_DELETED => staged += 1,

                Status::WT_MODIFIED | Status::WT_TYPECHANGE | Status::WT_DELETED => notstaged += 1,

                Status::WT_NEW => untracked += 1,

                Status::CONFLICTED => conflicted += 1,

                Status::CURRENT | Status::WT_RENAMED => {}

                Status::IGNORED => {}

                _ => {}
            }
        }

        Some((staged, notstaged, untracked, conflicted))
    } else {
        None
    }
}

pub fn write_git<'a, W: Write>(p: &mut Powerline<'a, W>) -> Result<()> {
    let git = match Repository::discover(".") {
        Ok(git) => git,
        Err(_) => return Ok(()),
    };

    let (local, upstream, branch_name) = match head_info(&git) {
        Ok(item) => item,
        Err(_) => return Ok(()),
    };

    if let Some(branch_name) = branch_name {
        let has_status = git
            .statuses(Some(
                StatusOptions::new()
                    .show(StatusShow::IndexAndWorkdir)
                    .include_untracked(true)
                    .renames_from_rewrites(true)
                    .renames_head_to_index(true),
            ))
            .ok()
            .map(|statuses| statuses.iter().next().is_some())
            .unwrap_or(false);
        let (fg, bg) = if !has_status {
            (Color::RepoCleanFg, Color::RepoCleanBg)
        } else {
            (Color::RepoDirtyFg, Color::RepoDirtyBg)
        };
        p.add(Segment::new(&branch_name, fg, bg))?;

        if let (Some(local), Some(upstream)) = (local, upstream) {
            if let Ok((ahead, behind)) = git.graph_ahead_behind(local, upstream) {
                if ahead > 0 {
                    let (fg, bg) = (Color::GitAheadFg, Color::GitAheadBg);
                    p.add(Segment::new(&format!("{}\u{2B06}", ahead), fg, bg))?;
                };
                if behind > 0 {
                    let (fg, bg) = (Color::GitBehindFg, Color::GitBehindBg);
                    p.add(Segment::new(&format!("{}\u{2B07}", behind), fg, bg))?;
                };
            }
        };

        if let Some((staged, notstaged, untracked, conflicted)) = count_stage(&git) {
            if staged > 0 {
                let (fg, bg) = (Color::GitStagedFg, Color::GitStagedBg);
                p.add(Segment::new(&format!("{}\u{2714}", staged), fg, bg))?;
            }
            if notstaged > 0 {
                let (fg, bg) = (Color::GitNotstagedFg, Color::GitNotstagedBg);
                p.add(Segment::new(&format!("{}\u{270E}", notstaged), fg, bg))?;
            }
            if untracked > 0 {
                let (fg, bg) = (Color::GitUntrackedFg, Color::GitUntrackedBg);
                p.add(Segment::new(&format!("{}+", untracked), fg, bg))?;
            }
            if conflicted > 0 {
                let (fg, bg) = (Color::GitConflictedFg, Color::GitConflictedBg);
                p.add(Segment::new(&format!("{}*", conflicted), fg, bg))?;
            }
        }
    } else {
        let (fg, bg) = (Color::RepoDirtyFg, Color::RepoDirtyBg);
        p.add(Segment::new("Big Bang", fg, bg))?;
    }

    Ok(())
}
