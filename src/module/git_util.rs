use git2::{Repository, StatusOptions, StatusShow};

#[derive(Debug)]
pub struct Summary {
    staged: usize,
    notstaged: usize,
    untracked: usize,
    conflicted: usize,
}

impl Summary {
    pub fn staged(&self) -> usize {
        self.staged
    }

    pub fn notstaged(&self) -> usize {
        self.notstaged
    }

    pub fn untracked(&self) -> usize {
        self.untracked
    }

    pub fn conflicted(&self) -> usize {
        self.conflicted
    }
}

pub fn summarize(repo: &Repository) -> Option<Summary> {
    let statuses = repo.statuses(Some(
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

        Some(Summary {
            staged,
            notstaged,
            untracked,
            conflicted,
        })
    } else {
        None
    }
}
