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
                .peel_to_commit()
                .ok()
                .map(|commit| Component::Computed(format!("{}", commit.id())[0..7].to_owned()))
        }
        None => None,
    }
}
