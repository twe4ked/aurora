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
    let commit = head.unwrap().peel_to_commit();
    if commit.is_err() {
        return Component::Empty;
    }
    Component::GitCommit(
        commit
            .unwrap()
            .id()
            .as_bytes()
            .iter()
            .fold(String::new(), |acc, b| acc + &format!("{:02x}", b))[0..7]
            .to_string(),
    )
}
