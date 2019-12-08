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
            Component::Color(color) => write!(f, "{}", color.display().unwrap_or(String::new())),
            Component::Cwd { style } => write!(f, "{}", style.display().unwrap_or(String::new())),
        }
    }
}
