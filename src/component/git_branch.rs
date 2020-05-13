use crate::component::Component;
use crate::Context;

pub fn display(context: &Context) -> Option<Component> {
    let repository = context.git_repository()?;
    repository.head().ok().and_then(|head| {
        head.shorthand()
            .map(|shorthand| Component::Computed(shorthand.to_string()))
    })
}
