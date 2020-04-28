use crate::component::Component;

pub fn display(jobs: Option<String>) -> Option<Component> {
    // https://github.com/clap-rs/clap/issues/1740
    match jobs {
        Some(jobs) => Some(Component::Jobs(jobs)),
        None => None,
    }
}
