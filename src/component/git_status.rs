use crate::Context;
use anyhow::Result;
use git2::{Status, StatusEntry};

// + New file added to the working tree
// * File modified in the working tree
// - File deleted from the working tree
// ^ A change is staged
pub fn display(context: &Context) -> Result<Option<String>> {
    if let Some(repo_status) = repo_status(context)? {
        let mut output = String::new();

        if repo_status.is_wt_modified() {
            output.push('*');
        }

        if repo_status.is_wt_new() {
            output.push('+');
        }

        if repo_status.is_wt_deleted() {
            output.push('-');
        }

        if repo_status.intersects(
            Status::INDEX_NEW
                | Status::INDEX_MODIFIED
                | Status::INDEX_DELETED
                | Status::INDEX_RENAMED
                | Status::INDEX_TYPECHANGE,
        ) {
            output.push('^');
        }

        if !output.is_empty() {
            return Ok(Some(output));
        }
    }
    Ok(None)
}

fn repo_status(context: &Context) -> Result<Option<Status>> {
    if let Some(ref r) = context.git_repository() {
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
