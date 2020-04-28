use crate::component::Component;

pub fn display() -> Component {
    let repository = crate::GIT_REPOSITORY.lock().expect("poisoned");
    match &*repository {
        Some(r) => {
            let head = r.head();
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
        None => Component::Empty,
    }
}
