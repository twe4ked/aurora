use crate::component::Component;

pub fn display(c: char) -> Component {
    Component::Char(format!("{}", c))
}
