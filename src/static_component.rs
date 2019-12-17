//! Static components are parsed from the users provided configuration.

use crate::component::cwd;

#[derive(Debug, PartialEq)]
pub enum Style {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    Reset,
}

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(char),
    Style(Style),
    Cwd { style: cwd::CwdStyle },
    GitBranch,
    GitCommit,
    GitStash,
}
