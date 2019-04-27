use std::io::Result;
use std::path::Path;
use std::string::ToString;

use git2::{BranchType, ObjectType, Oid, Repository, StatusOptions, StatusShow};

use super::Segment;
use crate::color::Color;
use crate::powerline::SegmentTarget;
use crate::symbol::Symbol;

type HeadInfo = (Option<Oid>, Option<Oid>, Option<String>);

fn head_info(git: &Repository) -> std::result::Result<HeadInfo, String> {
    let branches = match git.branches(Some(BranchType::Local)) {
        Ok(branches) => branches,
        Err(e) => return Err(e.message().to_owned()),
    };

    for branch in branches.filter_map(|b| if let Ok((b, _)) = b { Some(b) } else { None }) {
        if branch.is_head() {
            let local = branch.get().target();
            let upstream = branch.upstream().ok().and_then(|b| b.get().target());

            if let Ok(name) = branch.name() {
                return Ok((local, upstream, name.map(ToString::to_string)));
            }
        }
    }

    if let Ok(head) = git.head() {
        if let Some(target) = head.target() {
            let name = git
                .find_object(target, Some(ObjectType::Any))
                .ok()
                .and_then(|o| o.short_id().ok())
                .and_then(|b| b.as_str().map(ToString::to_string));
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
            if status.is_index_new()
                || status.is_index_modified()
                || status.is_index_typechange()
                || status.is_index_renamed()
                || status.is_index_deleted()
            {
                staged += 1
            }
            if status.is_wt_modified() || status.is_wt_typechange() || status.is_wt_deleted() {
                notstaged += 1
            }
            if status.is_wt_new() {
                untracked += 1
            }
            if status.is_conflicted() {
                conflicted += 1
            }
        }

        Some((staged, notstaged, untracked, conflicted))
    } else {
        None
    }
}

fn write_git_with_repo(p: &mut impl SegmentTarget, path: impl AsRef<Path>) -> Result<()> {
    let git = match Repository::discover(path) {
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
        p.append(Segment::new(&branch_name, fg, bg))?;

        if let (Some(local), Some(upstream)) = (local, upstream) {
            if let Ok((ahead, behind)) = git.graph_ahead_behind(local, upstream) {
                if ahead > 0 {
                    let (fg, bg) = (Color::GitAheadFg, Color::GitAheadBg);
                    p.append(Segment::text_sym(
                        &format!("{}", ahead),
                        Symbol::GitAhead,
                        fg,
                        bg,
                    ))?;
                };
                if behind > 0 {
                    let (fg, bg) = (Color::GitBehindFg, Color::GitBehindBg);
                    p.append(Segment::text_sym(
                        &format!("{}", behind),
                        Symbol::GitBehind,
                        fg,
                        bg,
                    ))?;
                };
            }
        };

        if let Some((staged, notstaged, untracked, conflicted)) = count_stage(&git) {
            if staged > 0 {
                let (fg, bg) = (Color::GitStagedFg, Color::GitStagedBg);
                p.append(Segment::text_sym(
                    &format!("{}", staged),
                    Symbol::GitStaged,
                    fg,
                    bg,
                ))?;
            }
            if notstaged > 0 {
                let (fg, bg) = (Color::GitNotstagedFg, Color::GitNotstagedBg);
                p.append(Segment::text_sym(
                    &format!("{}", notstaged),
                    Symbol::GitNotstaged,
                    fg,
                    bg,
                ))?;
            }
            if untracked > 0 {
                let (fg, bg) = (Color::GitUntrackedFg, Color::GitUntrackedBg);
                p.append(Segment::text_sym(
                    &format!("{}", untracked),
                    Symbol::GitUntracked,
                    fg,
                    bg,
                ))?;
            }
            if conflicted > 0 {
                let (fg, bg) = (Color::GitConflictedFg, Color::GitConflictedBg);
                p.append(Segment::text_sym(
                    &format!("{}", conflicted),
                    Symbol::GitConflicted,
                    fg,
                    bg,
                ))?;
            }
        }
    } else {
        let (fg, bg) = (Color::RepoDirtyFg, Color::RepoDirtyBg);
        p.append(Segment::new("Big Bang", fg, bg))?;
    }

    Ok(())
}

pub fn write_git(p: &mut impl SegmentTarget) -> Result<()> {
    write_git_with_repo(p, ".")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::segments::SegmentContent;
    use std::env::temp_dir;
    use std::fs::{create_dir, remove_dir_all};
    use std::io::{ErrorKind, Write};
    use std::path::{Path, PathBuf};
    use std::process::{self, Command, Stdio};

    #[derive(Debug)]
    struct TmpDir(PathBuf);
    impl TmpDir {
        fn new() -> TmpDir {
            let tmp = temp_dir();
            let id = process::id();
            let mut n = 0;

            let path = loop {
                let mut path = tmp.clone();
                path.push(format!("pwltest-{}-{}", id, n));
                match create_dir(&path) {
                    Ok(()) => break path,
                    Err(e) => match e.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => panic!(),
                    },
                };
                n = n + 1;
            };
            TmpDir(path)
        }
    }

    impl Drop for TmpDir {
        fn drop(&mut self) {
            remove_dir_all(&self.0).ok();
        }
    }

    impl AsRef<Path> for TmpDir {
        fn as_ref(&self) -> &Path {
            &self.0
        }
    }

    fn run(p: impl AsRef<Path>, script: &[&str]) {
        let script = script.join("\n");
        let mut proc = Command::new("bash")
            .current_dir(p)
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        {
            let stdin = proc.stdin.as_mut().unwrap();
            stdin.write_all(script.as_bytes()).unwrap();
        }
        if !proc.wait().unwrap().success() {
            panic!()
        }
    }

    #[test]
    pub fn test() {
        write_git(&mut |_seg: Segment<'_>| {}).unwrap();
    }

    #[test]
    pub fn test_non_git() {
        let repo = TmpDir::new();
        write_git_with_repo(&mut |_seg: Segment<'_>| panic!(), &repo).unwrap();
    }

    #[test]
    pub fn test_nocommit() {
        let repo = TmpDir::new();
        run(&repo, &["git init"]);

        let mut n = 0;
        write_git_with_repo(
            &mut |seg: Segment<'_>| match (n, seg.parts()) {
                (0, (SegmentContent::Text("Big Bang"), Color::RepoDirtyFg, Color::RepoDirtyBg)) => {
                    n = n + 1
                }
                x => panic!("{:?}", x),
            },
            &repo,
        )
        .unwrap();
    }

    #[test]
    pub fn test_clean() {
        let repo = TmpDir::new();
        run(
            &repo,
            &[
                "git init",
                "git checkout -b x",
                "git config user.email 'you@example.com'",
                "git config user.name 'Your Name'",
                "git commit -mx --allow-empty",
            ],
        );

        let mut n = 0;
        write_git_with_repo(
            &mut |seg: Segment<'_>| match (n, seg.parts()) {
                (0, (SegmentContent::Text("x"), Color::RepoCleanFg, Color::RepoCleanBg)) => {
                    n = n + 1
                }
                x => panic!("{:?}", x),
            },
            &repo,
        )
        .unwrap();
    }

    #[test]
    pub fn test_stage() {
        let tmp = TmpDir::new();
        let mut repoa = tmp.as_ref().to_path_buf();
        let mut repob = repoa.clone();
        repoa.push("a");
        repob.push("b");
        create_dir(&repoa).unwrap();
        create_dir(&repob).unwrap();
        println!("{:?} {:?}", repoa, repob);

        run(
            &repoa,
            &[
                "git init",
                "git checkout -b x",
                "touch mod rename del change wt_mod",
                "echo del > del",
                "echo rename > rename",
                "echo wt change > wt_change",
                "echo wt del > wt_del",
                "echo conflict > conflict",
                "git add mod rename del change wt_mod wt_change wt_del conflict",
                "git config user.email 'you@example.com'",
                "git config user.name 'Your Name'",
                "git commit -mx",
            ],
        );
        run(
            &repob,
            &[
                "git clone ../a .",
                "echo mod1 > conflict",
                "git config user.email 'you@example.com'",
                "git config user.name 'Your Name'",
                "git commit -amx",
            ],
        );
        run(&repoa, &["echo mod2 > conflict", "git commit -amx"]);
        run(
            &repob,
            &[
                "git pull",
                "touch new",
                "echo mod2 > mod",
                "rm change",
                "ln -s new change",
                "git add new mod change",
                "git rm del",
                "git mv rename rename2",
                "echo ok > wt_mod",
                "rm wt_change",
                "ln -s new wt_change",
                "rm wt_del",
                "git status",
                "echo wt new > wt_new",
            ],
        );

        let mut n = 0;
        write_git_with_repo(
            &mut |seg: Segment<'_>| match (n, seg.parts()) {
                (0, (SegmentContent::Text("x"), Color::RepoDirtyFg, Color::RepoDirtyBg)) => {
                    n = n + 1
                }
                (
                    1,
                    (
                        SegmentContent::TextSym("1", Symbol::GitAhead),
                        Color::GitAheadFg,
                        Color::GitAheadBg,
                    ),
                ) => n = n + 1,
                (
                    2,
                    (
                        SegmentContent::TextSym("1", Symbol::GitBehind),
                        Color::GitBehindFg,
                        Color::GitBehindBg,
                    ),
                ) => n = n + 1,
                (
                    3,
                    (
                        SegmentContent::TextSym("5", Symbol::GitStaged),
                        Color::GitStagedFg,
                        Color::GitStagedBg,
                    ),
                ) => n = n + 1,
                (
                    4,
                    (
                        SegmentContent::TextSym("3", Symbol::GitNotstaged),
                        Color::GitNotstagedFg,
                        Color::GitNotstagedBg,
                    ),
                ) => n = n + 1,
                (
                    5,
                    (
                        SegmentContent::TextSym("1", Symbol::GitUntracked),
                        Color::GitUntrackedFg,
                        Color::GitUntrackedBg,
                    ),
                ) => n = n + 1,
                (
                    6,
                    (
                        SegmentContent::TextSym("1", Symbol::GitConflicted),
                        Color::GitConflictedFg,
                        Color::GitConflictedBg,
                    ),
                ) => n = n + 1,
                x => panic!("{:?}", x),
            },
            &repob,
        )
        .unwrap();
    }

    #[test]
    pub fn test_a() {
        let a = TmpDir::new();
        let b = TmpDir::new();
        run(&a, &["git init"]);
        println!("{:?} {:?}", a, b)
    }

}
