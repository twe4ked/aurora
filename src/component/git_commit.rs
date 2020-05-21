use crate::Context;

pub fn display(context: &Context) -> Option<String> {
    let repository = context.git_repository()?;
    repository.head().ok().and_then(|head| {
        head.peel_to_commit()
            .ok()
            .map(|commit| format!("{}", commit.id())[0..7].to_owned())
    })
}
