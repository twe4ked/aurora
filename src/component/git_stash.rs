use crate::Context;

use git2::Repository;

pub fn display(context: &mut Context) -> Option<String> {
    let repository = context.git_repository_mut()?;
    let count = stash_count(repository)?;

    if count == 0 {
        None
    } else {
        Some(format!("{}+", count))
    }
}

fn stash_count(repository: &mut Repository) -> Option<usize> {
    let mut count = 0;
    repository
        .stash_foreach(|_, _, _| {
            count += 1;
            true
        })
        .ok()?;
    Some(count)
}
