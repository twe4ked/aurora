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

impl std::str::FromStr for StyleToken {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StyleToken::*;
        match s {
            "black" => Ok(Black),
            "dark_grey" => Ok(DarkGrey),
            "blue" => Ok(Blue),
            "dark_blue" => Ok(DarkBlue),
            "green" => Ok(Green),
            "dark_green" => Ok(DarkGreen),
            "red" => Ok(Red),
            "dark_red" => Ok(DarkRed),
            "cyan" => Ok(Cyan),
            "dark_cyan" => Ok(DarkCyan),
            "magenta" => Ok(Magenta),
            "dark_magenta" => Ok(DarkMagenta),
            "yellow" => Ok(Yellow),
            "dark_yellow" => Ok(DarkYellow),
            "white" => Ok(White),
            "reset" => Ok(Reset),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Component {
        name: String,
        options: HashMap<String, String>,
    },
    Char(char),
    Static(String),
    Style(StyleToken),
}
