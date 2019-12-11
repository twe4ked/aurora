use crate::error::Error;
use crate::CurrentDir;
use git2::Repository;

pub fn display(current_dir: &CurrentDir) -> Result<String, Error> {
    // TODO: Pass in &mut git_repo
    let mut repository = Repository::discover(current_dir.get())?;
    let mut count = 0;
    repository.stash_foreach(|_, _, _| {
        count += 1;
        true
    })?;
    Ok(format!("{}+", count))
}
