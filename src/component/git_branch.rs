use crate::component::Component;
use crate::Context;

pub fn display(context: &Context) -> Option<Component> {
    match context.git_repository() {
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
