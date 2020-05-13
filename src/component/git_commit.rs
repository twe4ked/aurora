use crate::component::Component;
use crate::Context;

pub fn display(context: &Context) -> Option<Component> {
    let repository = context.git_repository()?;
    repository.head().ok().and_then(|head| {
        head.peel_to_commit()
            .ok()
            .map(|commit| Component::Computed(format!("{}", commit.id())[0..7].to_owned()))
    })
}
