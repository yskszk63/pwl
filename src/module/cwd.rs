use std::path::Path;

use super::*;

fn workdir(env: &Environment) -> Option<&Path> {
    let repo = if let Some(repo) = &env.repo {
        repo
    } else {
        return None;
    };
    repo.workdir()
}

fn relative_repo_path<'a>(cwd: &'a Path, wd: &Path) -> Option<&'a Path> {
    let parent = if let Some(parent) = wd.parent() {
        parent
    } else {
        return None;
    };

    cwd.strip_prefix(parent).ok()
}

fn display(path: &Path) -> String {
    if let Some(homedir) = home::home_dir() {
        if homedir == path {
            return "~".into();
        }
    }
    if let Some(name) = path.file_name() {
        return name.to_string_lossy().into();
    }
    path.to_string_lossy().into()
}

pub fn render(env: &Environment) -> anyhow::Result<Option<Segment>> {
    Ok(if let Some(cwd) = &env.cwd {
        if let Some(wd) = workdir(&env) {
            if let Some(relative) = relative_repo_path(&cwd, &wd) {
                return Ok(Some(
                    Segment::builder(Color::Cwd, relative.display()).build(),
                ));
            }
        }
        Some(Segment::builder(Color::Cwd, display(cwd)).build())
    } else {
        Some(Segment::builder(Color::Cwd, "\\w").build())
    })
}
