//! Tokens are parsed from the users provided configuration.

use std::collections::HashMap;
use std::convert::TryFrom;

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
}

impl TryFrom<&str> for StyleToken {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use StyleToken::*;
        match value {
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
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    LastCommandStatus,
    EnvironmentVariable(String),
}

impl TryFrom<&str> for Condition {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
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
    GitStash,
    GitStatus,
    Hostname,
    Jobs,
    User,
}

impl TryFrom<&str> for Component {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cwd" => Ok(Component::Cwd),
            "git_branch" => Ok(Component::GitBranch),
            "git_commit" => Ok(Component::GitCommit),
            "git_stash" => Ok(Component::GitStash),
            "git_status" => Ok(Component::GitStatus),
            "hostname" => Ok(Component::Hostname),
            "jobs" => Ok(Component::Jobs),
            "user" => Ok(Component::User),
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
    // TODO: Rename Color(Color),
    Style(StyleToken),
    Reset,
    Conditional {
        condition: Condition,
        left: Vec<Token>,
        right: Option<Vec<Token>>,
    },
}
