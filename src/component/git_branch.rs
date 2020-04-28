use crate::component::Component;

pub fn display() -> Option<Component> {
    let repository = aurora_prompt::GIT_REPOSITORY.lock().expect("poisoned");
    match &*repository {
        Some(r) => {
            let head = r.head();
            if head.is_err() {
                return None;
            }
            head.unwrap()
                .shorthand()
                .map(|shorthand| Component::GitBranch(shorthand.to_string()))
        }
        None => None,
    }
}
