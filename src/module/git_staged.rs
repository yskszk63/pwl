use crate::segment::Group;
use super::*;

pub fn render(env: &mut Environment) -> anyhow::Result<Option<Segment>> {
    if let Some(summary) = env.summarize_git() {
        let n = summary.staged();
        if n > 0 {
            Ok(Some(Segment::builder(Color::GitStaged, format!("✔{}", n)).group(Group::GitStatus).build()))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
