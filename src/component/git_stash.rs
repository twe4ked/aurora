use crate::component::Component;
use git2::Repository;

pub fn display(repository: Option<&mut Repository>) -> Component {
    if repository.is_none() {
        return Component::Empty;
    }
    let mut count = 0;
    let x = repository.unwrap().stash_foreach(|_, _, _| {
        count += 1;
        true
    });
    if x.is_err() || count == 0 {
        return Component::Empty;
    }
    Component::GitStash(format!("{}+", count))
}
