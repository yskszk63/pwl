use crate::color::Color;

#[derive(Debug)]
pub struct Theme {
    separator: &'static str,
    separator_thin: &'static str,

    username_fg: u8,
    username_bg: u8,
    username_root_bg: u8,

    hostname_fg: u8,
    hostname_bg: u8,

    cmd_passed_fg: u8,
    cmd_passed_bg: u8,
    cmd_failed_fg: u8,
    cmd_failed_bg: u8,

    home_fg: u8,
    home_bg: u8,
    path_fg: u8,
    path_bg: u8,
    cwd_fg: u8,

    ssh_fg: u8,
    ssh_bg: u8,

    repo_clean_fg: u8,
    repo_clean_bg: u8,
    repo_dirty_fg: u8,
    repo_dirty_bg: u8,

    git_ahead_fg: u8,
    git_ahead_bg: u8,
    git_behind_fg: u8,
    git_behind_bg: u8,

    git_staged_fg: u8,
    git_staged_bg: u8,
    git_notstaged_fg: u8,
    git_notstaged_bg: u8,
    git_untracked_fg: u8,
    git_untracked_bg: u8,
    git_conflicted_fg: u8,
    git_conflicted_bg: u8,

    jobs_fg: u8,
    jobs_bg: u8,

    virtualenv_fg: u8,
    virtualenv_bg: u8,
}

impl Theme {
    pub fn get(&self, color: &Color) -> u8 {
        match color {
            Color::UsernameFg => self.username_fg,
            Color::UsernameBg => self.username_bg,
            Color::UsernameRootBg => self.username_root_bg,

            Color::HostnameFg => self.hostname_fg,
            Color::HostnameBg => self.hostname_bg,

            Color::CmdPassedFg => self.cmd_passed_fg,
            Color::CmdPassedBg => self.cmd_passed_bg,
            Color::CmdFailedFg => self.cmd_failed_fg,
            Color::CmdFailedBg => self.cmd_failed_bg,

            Color::HomeFg => self.home_fg,
            Color::HomeBg => self.home_bg,
            Color::PathFg => self.path_fg,
            Color::PathBg => self.path_bg,
            Color::CwdFg => self.cwd_fg,

            Color::SshFg => self.ssh_fg,
            Color::SshBg => self.ssh_bg,

            Color::RepoCleanFg => self.repo_clean_fg,
            Color::RepoCleanBg => self.repo_clean_bg,
            Color::RepoDirtyFg => self.repo_dirty_fg,
            Color::RepoDirtyBg => self.repo_dirty_bg,

            Color::GitAheadFg => self.git_ahead_fg,
            Color::GitAheadBg => self.git_ahead_bg,
            Color::GitBehindFg => self.git_behind_fg,
            Color::GitBehindBg => self.git_behind_bg,

            Color::GitStagedFg => self.git_staged_fg,
            Color::GitStagedBg => self.git_staged_bg,
            Color::GitNotstagedFg => self.git_notstaged_fg,
            Color::GitNotstagedBg => self.git_notstaged_bg,
            Color::GitUntrackedFg => self.git_untracked_fg,
            Color::GitUntrackedBg => self.git_untracked_bg,
            Color::GitConflictedFg => self.git_conflicted_fg,
            Color::GitConflictedBg => self.git_conflicted_bg,

            Color::JobsFg => self.jobs_fg,
            Color::JobsBg => self.jobs_bg,

            Color::VirtualenvFg => self.virtualenv_fg,
            Color::VirtualenvBg => self.virtualenv_bg,
        }
    }

    pub fn separator(&self) -> &str {
        &self.separator
    }

    pub fn separator_thin(&self) -> &str {
        &self.separator_thin
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            separator: "\u{E0B0}",
            separator_thin: "\u{E0B1}",

            username_fg: 250,
            username_bg: 240,
            username_root_bg: 124,

            hostname_fg: 250,
            hostname_bg: 238,

            cmd_passed_fg: 15,
            cmd_passed_bg: 236,
            cmd_failed_fg: 15,
            cmd_failed_bg: 161,

            home_fg: 15,
            home_bg: 31,
            path_fg: 250,
            path_bg: 237,
            cwd_fg: 254,

            ssh_fg: 39,
            ssh_bg: 238,

            repo_clean_fg: 0,
            repo_clean_bg: 148,
            repo_dirty_fg: 15,
            repo_dirty_bg: 161,

            git_ahead_fg: 250,
            git_ahead_bg: 240,
            git_behind_fg: 250,
            git_behind_bg: 240,

            git_staged_fg: 15,
            git_staged_bg: 22,
            git_notstaged_fg: 15,
            git_notstaged_bg: 130,
            git_untracked_fg: 15,
            git_untracked_bg: 52,
            git_conflicted_fg: 15,
            git_conflicted_bg: 9,

            jobs_fg: 39,
            jobs_bg: 238,

            virtualenv_fg: 0,
            virtualenv_bg: 35,
        }
    }
}

pub fn solarized_light() -> Theme {
    Theme {
        username_fg: 15,
        username_bg: 4,
        username_root_bg: 1,

        hostname_fg: 15,
        hostname_bg: 10,

        path_fg: 10,
        path_bg: 7,
        cwd_fg: 0,

        repo_clean_fg: 0,
        repo_clean_bg: 15,
        repo_dirty_fg: 1,
        repo_dirty_bg: 15,

        jobs_fg: 4,
        jobs_bg: 7,

        cmd_passed_fg: 15,
        cmd_passed_bg: 2,
        cmd_failed_fg: 15,
        cmd_failed_bg: 1,

        virtualenv_fg: 15,
        virtualenv_bg: 2,

        ..Default::default()
    }
}
