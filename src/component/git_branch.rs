use crate::Context;

pub fn display(context: &Context) -> Option<String> {
    let repository = context.git_repository()?;
    repository
        .head()
        .ok()
        .and_then(|head| head.shorthand().map(|shorthand| shorthand.to_string()))
}
