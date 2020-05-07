use crate::component::Component;
use crate::Context;

pub fn display(context: &mut Context) -> Option<Component> {
    match context.git_repository_mut() {
        Some(ref mut r) => {
            let mut count = 0;
            let x = r.stash_foreach(|_, _, _| {
                count += 1;
                true
            });
            if x.is_err() || count == 0 {
                return None;
            }
            Some(Component::Computed(format!("{}+", count)))
        }
        None => None,
    }
}
