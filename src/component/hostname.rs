use crate::component::Component;

pub fn display() -> Option<Component> {
    gethostname::gethostname()
        .into_string()
        .ok()
        .map(Component::Computed)
}
