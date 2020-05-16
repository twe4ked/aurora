use crate::component::Component;

pub fn display() -> Option<Component> {
    std::env::var("USER").ok().map(Component::Computed)
}
