use crate::component::Component;

pub fn display() -> Option<Component> {
    let repository = crate::GIT_REPOSITORY.lock().expect("poisoned");
    match &*repository {
        Some(r) => {
            let head = r.head();
            if head.is_err() {
                return None;
            }
            head.unwrap()
                .shorthand()
                .map(|shorthand| Component::Computed(shorthand.to_string()))
        }
        None => None,
    }
}
