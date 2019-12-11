use crate::error::Error;
use crate::git_repo::GitRepo;

pub fn display(git_repo: &GitRepo) -> Result<String, Error> {
    let head = git_repo.repository()?.head()?;
    let commit = head.peel_to_commit()?;
    Ok(commit
        .id()
        .as_bytes()
        .iter()
        .fold(String::new(), |acc, b| acc + &format!("{:02x}", b))[0..7]
        .to_string())
}
