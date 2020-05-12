use crate::component::Component;
use crate::Context;

use git2::Repository;

pub fn display(context: &mut Context) -> Option<Component> {
    let repository = context.git_repository_mut()?;
    let count = stash_count(repository)?;

    if count == 0 {
        None
    } else {
        Some(Component::Computed(format!("{}+", count)))
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
