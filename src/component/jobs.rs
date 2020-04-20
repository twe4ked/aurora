use crate::component::Component;

pub fn display(jobs: Option<String>) -> Component {
    // https://github.com/clap-rs/clap/issues/1740
    match jobs {
        Some(jobs) => Component::Jobs(jobs),
        None => Component::Empty,
    }
}
