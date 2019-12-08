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
            Component::Char(c) => format!("{}", c),
            Component::Color(color) => color.display().unwrap_or(String::new()),
            Component::Cwd { style } => style.display().unwrap_or(String::new()),
        };

        write!(f, "{}", component)
    }
}
