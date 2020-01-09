use crate::component::Component;
use crate::error::Error;
use git2::Repository;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum CwdStyle {
    Default,
    Long,
    Short,
}

pub fn display(
    style: &CwdStyle,
    current_dir: &PathBuf,
    repository: Option<&Repository>,
) -> Component {
    Component::Cwd(cwd(style, current_dir, repository).unwrap_or(long(current_dir).unwrap()))
}

fn cwd(
    style: &CwdStyle,
    current_dir: &PathBuf,
    repository: Option<&Repository>,
) -> Result<String, Error> {
    match style {
        CwdStyle::Default => {
            let home_dir = dirs::home_dir().unwrap_or(PathBuf::new());
            Ok(replace_home_dir(current_dir, &home_dir))
        }
        CwdStyle::Short => {
            let home_dir = dirs::home_dir().unwrap_or(PathBuf::new());
            match repository {
                Some(repository) => {
                    short(&current_dir, &home_dir, &repository.path().to_path_buf())
                }
                // TODO: We want to contract up to the current dir if we don't have a git root.
                None => Ok(replace_home_dir(current_dir, &home_dir)),
            }
        }
        CwdStyle::Long => long(current_dir),
    }
}

/// Replace the home directory portion of the path with "~/"
fn replace_home_dir(current_dir: &PathBuf, home_dir: &PathBuf) -> String {
    format!("{}", current_dir.display()).replacen(&format!("{}", home_dir.display()), "~", 1)
}

fn short(current_dir: &PathBuf, home_dir: &PathBuf, git_root: &Path) -> Result<String, Error> {
    let current_dir = current_dir.strip_prefix(&home_dir)?;

    // Remove repo/.git
    let git_root = git_root.parent().unwrap().parent().unwrap();

    // Remove the home_dir from the git_root.
    let git_root = git_root.strip_prefix(&home_dir)?;

    let short_repo = git_root.iter().fold(PathBuf::new(), |acc, part| {
        acc.join(format!("{}", part.to_string_lossy().chars().nth(0).unwrap()).as_str())
    });

    let rest = current_dir.strip_prefix(&git_root)?;

    let mut output = PathBuf::new();
    output.push(&home_dir);
    output.push(short_repo);
    output.push(rest);

    Ok(replace_home_dir(&output, &home_dir.to_path_buf()))
}

fn long(current_dir: &PathBuf) -> Result<String, Error> {
    Ok(format!("{}", current_dir.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_home_dir() {
        let current_dir = PathBuf::from("/home/foo/bar/baz");
        let home_dir = PathBuf::from("/home/foo");

        assert_eq!(
            replace_home_dir(&current_dir, &home_dir),
            "~/bar/baz".to_string()
        );
    }

    #[test]
    fn test_replace_home_dir_in_home_dir() {
        let current_dir = PathBuf::from("/home/foo");
        let home_dir = PathBuf::from("/home/foo");

        assert_eq!(replace_home_dir(&current_dir, &home_dir), "~".to_string());
    }

    #[test]
    fn short_test() {
        let current_dir = PathBuf::from("/home/foo/axx/bxx/repo/cxx/dxx");
        let home_dir = PathBuf::from("/home/foo");
        let git_root = Path::new("/home/foo/axx/bxx/repo/.git");

        assert_eq!(
            short(&current_dir, &home_dir, &git_root).unwrap(),
            "~/a/b/repo/cxx/dxx".to_string()
        );

        let current_dir = PathBuf::from("/home/foo/axx/bxx/repo");
        assert_eq!(
            short(&current_dir, &home_dir, &git_root).unwrap(),
            "~/a/b/repo".to_string()
        );
    }

    #[test]
    fn short_test_single_dir_repo() {
        let current_dir = PathBuf::from("/home/foo/axx");
        let home_dir = PathBuf::from("/home/foo");
        let git_root = Path::new("/home/foo/axx/.git");

        assert_eq!(
            short(&current_dir, &home_dir, &git_root).unwrap(),
            "~/axx".to_string()
        );
    }
}
