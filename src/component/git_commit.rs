use crate::component::Component;
use crate::Context;

pub fn display(context: &Context) -> Option<Component> {
    match context.git_repository() {
        Some(r) => {
            let head = r.head();
            if head.is_err() {
                return None;
            }
            head.unwrap().peel_to_commit().ok().map(|commit| {
                Component::Computed(
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
