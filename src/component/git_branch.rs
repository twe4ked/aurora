use crate::component::Component;

pub fn display() -> Component {
    let repository = crate::GIT_REPOSITORY.lock().expect("poisoned");
    match &*repository {
        Some(r) => {
            let head = r.head();
            if head.is_err() {
                return Component::Empty;
            }
            let head = head.unwrap();
            match head.shorthand() {
                Some(shorthand) => Component::GitBranch(shorthand.to_string()),
                None => Component::Empty,
            }
        }
        None => Component::Empty,
    }
}
