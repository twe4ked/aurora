use crate::component::Component;

pub fn display() -> Option<Component> {
    let repository = crate::GIT_REPOSITORY.lock().expect("poisoned");
    match &*repository {
        Some(r) => {
            let head = r.head();
            if head.is_err() {
                return None;
            }
            head.unwrap().peel_to_commit().ok().map(|commit| {
                Component::GitCommit(
                    commit
                        .id()
                        .as_bytes()
                        .iter()
                        .fold(String::new(), |acc, b| acc + &format!("{:02x}", b))
                        [0..7]
                        .to_string(),
                )
            })
        }
        None => None,
    }
}
