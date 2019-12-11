use crate::error::Error;
use git2::Repository;

pub fn display(repository: Option<&mut Repository>) -> Result<String, Error> {
    if repository.is_none() {
        return Ok(String::new());
    }
    let mut count = 0;
    repository.unwrap().stash_foreach(|_, _, _| {
        count += 1;
        true
    })?;
    Ok(format!("{}+", count))
}
