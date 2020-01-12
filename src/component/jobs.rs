use crate::component::Component;

pub fn display(jobs: &Option<String>) -> Component {
    let jobs = jobs.as_ref().unwrap().to_string();
    if jobs.is_empty() {
        Component::Empty
    } else {
        Component::Jobs(jobs)
    }
}
