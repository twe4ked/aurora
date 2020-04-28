use crate::component::Component;

pub fn display(c: char) -> Option<Component> {
    Some(Component::Char(format!("{}", c)))
}
