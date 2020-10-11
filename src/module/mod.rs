use std::env;
use std::path::PathBuf;

use git2::Repository;

use crate::Color;
use crate::Segment;
use git_util::{summarize as summarize_git, Summary as GitSummary};

mod cwd;
mod git_ahead_behind;
mod git_branch;
mod git_conflicted;
mod git_not_staged;
mod git_staged;
mod git_untracked;
mod git_util;
mod root;

pub struct Environment {
    exit_code: i32,
    cwd: Option<PathBuf>,
    repo: Option<Repository>,
    summary: Option<GitSummary>,
}

impl Environment {
    pub fn new(exit_code: i32) -> Self {
        let cwd = env::current_dir().ok();
        let repo = if let Some(cwd) = &cwd {
            Repository::discover(cwd).ok()
        } else {
            None
        };
        Self {
            exit_code,
            cwd,
            repo,
            summary: None,
        }
    }

    fn summarize_git(&mut self) -> Option<&GitSummary> {
        if self.summary.is_some() {
            return self.summary.as_ref();
        }

        let summary = if let Some(repo) = &self.repo {
            summarize_git(repo)
        } else {
            None
        };
        self.summary = summary;
        self.summary.as_ref()
    }
}

pub enum Module {
    Cwd,
    GitBranch,
    GitAheadBehind,
    GitStaged,
    GitNotStaged,
    GitUntracked,
    GitConflicted,
    Root,
}

impl Module {
    pub fn render(&self, env: &mut Environment) -> anyhow::Result<Option<Segment>> {
        let segment = match self {
            Self::Cwd => cwd::render(env)?,
            Self::GitBranch => git_branch::render(env)?,
            Self::GitAheadBehind => git_ahead_behind::render(env)?,
            Self::GitStaged => git_staged::render(env)?,
            Self::GitNotStaged => git_not_staged::render(env)?,
            Self::GitUntracked => git_untracked::render(env)?,
            Self::GitConflicted => git_conflicted::render(env)?,
            Self::Root => root::render(env)?,
        };
        Ok(segment)
    }
}
