use crate::component::Component;
use crate::error::Error;
use git2::Repository;

pub fn display(repository: Option<&Repository>) -> Component {
    if repository.is_none() {
        return Component::GitBranch(None);
    }
    let head = repository.unwrap().head();
    if head.is_err() {
        return Component::GitBranch(None);
    }
    let head = head.unwrap();
    let shorthand = head.shorthand();
    let x = shorthand.map(std::string::ToString::to_string);
    Component::GitBranch(x)
}
