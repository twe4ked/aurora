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
