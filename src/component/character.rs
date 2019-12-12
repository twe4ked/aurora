use crate::component::Component;
use crate::error::Error;

pub fn display(c: &char) -> Component {
    Component::Char(Some(format!("{}", c)))
}
