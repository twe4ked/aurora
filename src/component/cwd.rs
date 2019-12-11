use crate::error::Error;
use git2::Repository;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum CwdStyle {
    Default,
    Long,
    Short,
}

impl CwdStyle {
    pub fn display(
        &self,
        current_dir: &PathBuf,
        repository: Option<&mut Repository>,
    ) -> Result<String, Error> {
        Ok(format!("{}", inner(current_dir, repository, self)))
    }
}

fn inner(current_dir: &PathBuf, repository: Option<&mut Repository>, style: &CwdStyle) -> String {
    match style {
        CwdStyle::Default => {
            let home_dir = dirs::home_dir().unwrap_or(PathBuf::new());
            replace_home_dir(current_dir, home_dir)
        }
        CwdStyle::Short => {
            let home_dir = dirs::home_dir().unwrap_or(PathBuf::new());
            match repository {
                Some(repository) => {
                    short(&current_dir, &home_dir, &repository.path().to_path_buf())
                }
                // TODO: We want to contract up to the current dir if we don't have a git root.
                None => replace_home_dir(current_dir, home_dir),
            }
        }
        CwdStyle::Long => format!("{}", current_dir.display()),
    }
}

/// Replace the home directory portion of the path with "~/"
fn replace_home_dir(current_dir: &PathBuf, home_dir: PathBuf) -> String {
    match current_dir.strip_prefix(home_dir) {
        Ok(current_dir) => format!("~/{}", current_dir.display()),
        // Unable to strip the prefix, fall back to full path
        Err(_) => format!("{}", current_dir.display()),
    }
}

fn short(current_dir: &PathBuf, home_dir: &PathBuf, git_root: &Path) -> String {
    match current_dir.strip_prefix(&home_dir) {
        Ok(current_dir) => {
            // Remove repo/.git
            let git_root = git_root.parent().unwrap().parent().unwrap();
            // Remove the home_dir from the git_root.
            let git_root = git_root
                .strip_prefix(&home_dir)
                .expect("unable to remove home dir");

            let short_repo = git_root.iter().fold(PathBuf::new(), |acc, part| {
                acc.join(format!("{}", part.to_string_lossy().chars().nth(0).unwrap()).as_str())
            });

            let rest = current_dir
                .strip_prefix(&git_root)
                .expect("unable to remove non-home-dir git_root from dir");
            format!("~/{}/{}", short_repo.display(), rest.display())
        }
        // Unable to strip the prefix, fall back to full path
        Err(_) => format!("{}", current_dir.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_home_dir() {
        let current_dir = PathBuf::from("/home/foo/bar/baz");
        let home_dir = PathBuf::from("/home/foo");

        assert_eq!(
            replace_home_dir(&current_dir, home_dir),
            "~/bar/baz".to_string()
        );
    }

    #[test]
    fn short_test() {
        let current_dir = PathBuf::from("/home/foo/axx/bxx/repo/cxx/dxx");
        let home_dir = PathBuf::from("/home/foo");
        let git_root = Path::new("/home/foo/axx/bxx/repo/.git");

        assert_eq!(
            short(&current_dir, &home_dir, &git_root),
            "~/a/b/repo/cxx/dxx".to_string()
        );

        let current_dir = PathBuf::from("/home/foo/axx/bxx/repo");
        assert_eq!(
            short(&current_dir, &home_dir, &git_root),
            "~/a/b/repo".to_string()
        );
    }
}
