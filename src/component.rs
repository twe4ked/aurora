pub mod character;
pub mod color;
pub mod cwd;
pub mod git_branch;
pub mod git_commit;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(char),
    Color(color::Color),
    Cwd { style: cwd::CwdStyle },
    GitBranch,
    GitCommit,
}
