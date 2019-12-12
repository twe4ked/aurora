use crate::component::Component;
use crate::error::Error;
use git2::Repository;

pub fn display(repository: Option<&mut Repository>) -> Component {
    if repository.is_none() {
        return Component::GitStash(None);
    }
    let mut count = 0;
    let x = repository.unwrap().stash_foreach(|_, _, _| {
        count += 1;
        true
    });
    if x.is_err() {
        return Component::GitStash(None);
    }
    Component::GitStash(Some(format!("{}+", count)))
}
