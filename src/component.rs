pub mod character;
pub mod color;
pub mod cwd;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(char),
    Color(color::Color),
    Cwd { style: cwd::CwdStyle },
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let component = match self {
            Component::Char(c) => character::display(c),
            Component::Color(color) => color.display(),
            Component::Cwd { style } => style.display(),
        };

        write!(f, "{}", component.unwrap_or(String::new()))
    }
}
