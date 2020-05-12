use crate::component::Component;

pub fn display(jobs: Option<&str>) -> Option<Component> {
    jobs.map(|jobs| Component::Computed(jobs.to_owned()))
}
