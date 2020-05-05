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
pub enum Condition {
    LastCommandStatus,
}

impl std::str::FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "last_command_status" => Ok(Condition::LastCommandStatus),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Component {
    Cwd,
    GitBranch,
    GitCommit,
    Jobs,
    GitStash,
}

impl std::str::FromStr for Component {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cwd" => Ok(Component::Cwd),
            "git_branch" => Ok(Component::GitBranch),
            "git_commit" => Ok(Component::GitCommit),
            "git_stash" => Ok(Component::GitStash),
            "jobs" => Ok(Component::Jobs),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Component {
        name: Component,
        options: HashMap<String, String>,
    },
    Static(String),
    Style(StyleToken),
    Conditional {
        condition: Condition,
        left: Vec<Token>,
        right: Option<Vec<Token>>,
    },
}
