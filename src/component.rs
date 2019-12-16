pub mod character;
pub mod color;
pub mod cwd;
pub mod git_branch;
pub mod git_commit;
pub mod git_stash;

#[derive(Debug, PartialEq)]
pub enum ColorStartReset {
    Start(String),
    Reset(String),
}

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(String),
    Color(ColorStartReset),
    Cwd(String),
    GitBranch(String),
    GitCommit(String),
    GitStash(String),
    Empty,
}
