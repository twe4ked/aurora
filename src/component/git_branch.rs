use crate::component::Component;
use git2::Repository;

pub fn display(repository: Option<&Repository>) -> Component {
    if repository.is_none() {
        return Component::Empty;
    }
    let head = repository.unwrap().head();
    if head.is_err() {
        return Component::Empty;
    }
    let head = head.unwrap();
    match head.shorthand() {
        Some(shorthand) => Component::GitBranch(shorthand.to_string()),
        None => Component::Empty,
    }
}
