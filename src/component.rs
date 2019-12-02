pub mod cwd;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(char),
    Cwd { style: cwd::CwdStyle },
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::Char(c) => write!(f, "{}", c),
            Component::Cwd { style } => cwd::display(f, style),
        }
    }
}
