pub mod character;
pub mod color;
pub mod cwd;
pub mod git_branch;
pub mod git_commit;
pub mod git_stash;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(Option<String>),
    Color(Option<String>),
    Cwd(Option<String>),
    GitBranch(Option<String>),
    GitCommit(Option<String>),
    GitStash(Option<String>),
}
