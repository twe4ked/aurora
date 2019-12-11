use crate::error::Error;
use git2::Repository;

pub fn display(repository: Option<&Repository>) -> Result<String, Error> {
    if repository.is_none() {
        return Ok(String::new());
    }
    let head = repository.unwrap().head()?;
    let commit = head.peel_to_commit()?;
    Ok(commit
        .id()
        .as_bytes()
        .iter()
        .fold(String::new(), |acc, b| acc + &format!("{:02x}", b))[0..7]
        .to_string())
}
