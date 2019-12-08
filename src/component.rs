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
        match self {
            Component::Char(c) => write!(f, "{}", c),
            Component::Color(color) => color.display(f),
            Component::Cwd { style } => style.display(f),
        }
    }
}
