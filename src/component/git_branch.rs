use crate::error::Error;
use crate::git_repo::GitRepo;

pub fn display(git_repo: &GitRepo) -> Result<String, Error> {
    let head = git_repo.repository()?.head()?;
    let shorthand = head.shorthand();
    Ok(shorthand
        .map(std::string::ToString::to_string)
        .unwrap_or(String::new()))
}
