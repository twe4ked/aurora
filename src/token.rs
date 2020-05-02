//! Tokens are parsed from the users provided configuration.

use std::collections::HashMap;

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
    Component {
        name: String,
        options: HashMap<String, String>,
    },
    Char(char),
    Style(StyleToken),
}
