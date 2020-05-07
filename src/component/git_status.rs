use crate::component::Component;
use anyhow::Result;
use git2::{Status, StatusEntry};

pub fn display() -> Result<Option<Component>> {
    if let Some(repo_status) = repo_status()? {
        if repo_status.is_wt_modified() {
            return Ok(Some(Component::Computed("*".to_string())));
        }
    }
    Ok(None)
}

fn repo_status() -> Result<Option<Status>> {
    let repository = crate::GIT_REPOSITORY.lock().expect("poisoned");
    if let Some(ref r) = &*repository {
        let status_options = None;
        let statuses = r
            .statuses(status_options)?
            .iter()
            .fold(Status::empty(), add_status);
        return Ok(Some(statuses));
    }
    Ok(None)
}

fn add_status(mut s: Status, x: StatusEntry) -> Status {
    s.insert(x.status());
    s
}
