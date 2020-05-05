use crate::component::Component;

pub fn display(jobs: Option<&str>) -> Option<Component> {
    match jobs {
        Some(jobs) => Some(Component::Computed(jobs.to_owned())),
        None => None,
    }
}
