//! Tokens are parsed from the users provided configuration.

use std::collections::HashMap;
use std::convert::TryFrom;

use crossterm::style::Color;

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
    Env,
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
            "env" => Ok(Component::Env),
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
    Color(Color),
    Reset,
    Conditional {
        condition: Condition,
        left: Vec<Token>,
        right: Option<Vec<Token>>,
    },
}
