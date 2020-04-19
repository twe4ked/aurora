//! Tokens are parsed from the users provided configuration.

use crate::component::cwd;

#[derive(Debug, PartialEq)]
pub enum StyleToken {
    Black,
    DarkGrey,
    Blue,
    DarkBlue,
    Green,
    DarkGreen,
    Red,
    DarkRed,
    Cyan,
    DarkCyan,
    Magenta,
    DarkMagenta,
    Yellow,
    DarkYellow,
    White,
    Reset,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Char(char),
    Style(StyleToken),
    Cwd { style: cwd::CwdStyle },
    GitBranch,
    GitCommit,
    GitStash,
    Jobs,
}
