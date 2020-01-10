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
            let repository = match repository {
                Some(repository) => Some(repository.path()),
                None => None,
            };
            short(&current_dir, &home_dir, repository)
        }
        CwdStyle::Long => long(current_dir),
    }
}

/// Replace the home directory portion of the path with "~/"
fn replace_home_dir(current_dir: &PathBuf, home_dir: &PathBuf) -> String {
    format!("{}", current_dir.display()).replacen(&format!("{}", home_dir.display()), "~", 1)
}

fn short(
    full_path: &PathBuf,
    home_dir: &PathBuf,
    git_path: Option<&Path>,
) -> Result<String, Error> {
    let full_path = replace_home_dir(&full_path, &home_dir);
    let git_path_length = {
        match git_path {
            Some(git_path) => {
                let git_path = git_path.parent().unwrap(); // Remove ".git"
                let git_path = replace_home_dir(&git_path.to_path_buf(), &home_dir);
                git_path.split('/').collect::<Vec<_>>().len()
            }
            None => 1,
        }
    };

    let full_path_length = full_path.split('/').collect::<Vec<_>>().len();

    Ok(full_path
        .split('/')
        .enumerate()
        .map(|(i, part)| {
            if i == git_path_length - 1 {
                part.to_string()
            } else if i == full_path_length - 1 {
                part.to_string()
            } else if let Some(c) = part.chars().nth(0) {
                c.to_string()
            } else {
                String::new()
            }
        })
        .collect::<Vec<_>>()
        .join("/"))
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
            short(&current_dir, &home_dir, Some(&git_root)).unwrap(),
            "~/a/b/repo/c/dxx".to_string()
        );

        let current_dir = PathBuf::from("/home/foo/axx/bxx/repo");
        assert_eq!(
            short(&current_dir, &home_dir, Some(&git_root)).unwrap(),
            "~/a/b/repo".to_string()
        );
    }

    #[test]
    fn short_test_single_dir_repo() {
        let current_dir = PathBuf::from("/home/foo/axx");
        let home_dir = PathBuf::from("/home/foo");
        let git_root = Path::new("/home/foo/axx/.git");

        assert_eq!(
            short(&current_dir, &home_dir, Some(&git_root)).unwrap(),
            "~/axx".to_string()
        );
    }

    #[test]
    fn short_test_root() {
        let current_dir = PathBuf::from("/foo/bar/axx/bxx/cxx/dxx");
        let home_dir = PathBuf::from("/home/baz");
        let git_root = Path::new("/foo/bar/axx/.git");

        assert_eq!(
            short(&current_dir, &home_dir, Some(&git_root)).unwrap(),
            "/f/b/axx/b/c/dxx".to_string()
        );
    }

    #[test]
    fn short_test_root_no_repo() {
        let current_dir = PathBuf::from("/foo/bar/axx/bxx/cxx/dxx");
        let home_dir = PathBuf::from("/home/baz");

        assert_eq!(
            short(&current_dir, &home_dir, None).unwrap(),
            "/f/b/a/b/c/dxx".to_string()
        );
    }
}
