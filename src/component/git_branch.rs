use crate::error::Error;
use git2::Repository;

pub fn display(repository: Option<&mut Repository>) -> Result<String, Error> {
    if repository.is_none() {
        return Ok(String::new());
    }
    let head = repository.unwrap().head()?;
    let shorthand = head.shorthand();
    Ok(shorthand
        .map(std::string::ToString::to_string)
        .unwrap_or(String::new()))
}
