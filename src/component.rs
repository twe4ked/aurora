use std::fmt;

pub mod character;
pub mod color;
pub mod cwd;
pub mod git_branch;
pub mod git_commit;
pub mod git_stash;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(String),
    Color(color::Color),
    Cwd(String),
    GitBranch(String),
    GitCommit(String),
    GitStash(String),
    Empty,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::Char(c)
            | Component::Color(color::Color::Black(c))
            | Component::Color(color::Color::Blue(c))
            | Component::Color(color::Color::Green(c))
            | Component::Color(color::Color::Red(c))
            | Component::Color(color::Color::Cyan(c))
            | Component::Color(color::Color::Magenta(c))
            | Component::Color(color::Color::Yellow(c))
            | Component::Color(color::Color::White(c))
            | Component::Color(color::Color::Reset(c))
            | Component::Cwd(c)
            | Component::GitBranch(c)
            | Component::GitCommit(c)
            | Component::GitStash(c) => write!(f, "{}", c),
            Component::Empty => write!(f, ""),
        }
    }
}
