use crate::component::Component;

pub fn display() -> Option<Component> {
    let mut repository = aurora_prompt::GIT_REPOSITORY.lock().expect("poisoned");
    match &mut *repository {
        Some(ref mut r) => {
            let mut count = 0;
            let x = r.stash_foreach(|_, _, _| {
                count += 1;
                true
            });
            if x.is_err() || count == 0 {
                return None;
            }
            Some(Component::GitStash(format!("{}+", count)))
        }
        None => None,
    }
}
