use crate::component::Component;
use crate::utility::wrap_no_change_cursor_position;
use crate::Context;
use crate::Shell;

use anyhow::Result;
use crossterm::style::Attribute;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
enum Style {
    Default,
    Long,
    Short,
}

impl TryFrom<String> for Style {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "default" => Ok(Style::Default),
            "short" => Ok(Style::Short),
            "long" => Ok(Style::Long),
            _ => Err(anyhow::anyhow!("error: invalid style: {}", value)),
        }
    }
}

fn parse_boolean(input: Option<String>) -> Result<bool> {
    match input.as_ref().map(|s| &s[..]) {
        Some("true") => Ok(true),
        Some("false") => Ok(false),
        Some(s) => Err(anyhow::anyhow!("error: invalid boolean: {}", s)),
        None => Ok(false),
    }
}

struct Options {
    style: Style,
    underline_repo: bool,
}

impl Options {
    fn extract(options: &mut HashMap<String, String>) -> Result<Self> {
        let style = match options.remove("style") {
            Some(s) => Style::try_from(s)?,
            None => Style::Default,
        };
        let underline_repo = parse_boolean(options.remove("underline_repo"))?;

        Ok(Self {
            style,
            underline_repo,
        })
    }
}

pub fn display(
    context: &Context,
    mut options: &mut HashMap<String, String>,
    shell: &Shell,
) -> Result<Option<Component>> {
    let options = Options::extract(&mut options)?;

    let output = match options.style {
        Style::Default => default(context.current_dir()),
        Style::Short => short(
            &context.current_dir(),
            &dirs::home_dir().unwrap_or_default(),
            context.git_repository().map(|r| r.path()),
            options.underline_repo,
            shell,
        ),
        Style::Long => long(context.current_dir()),
    };

    Ok(Some(Component::Computed(output)))
}

/// Replace the home directory portion of the path with "~/"
fn replace_home_dir(current_dir: &PathBuf, home_dir: &PathBuf) -> String {
    format!("{}", current_dir.display()).replacen(&format!("{}", home_dir.display()), "~", 1)
}

fn default(current_dir: &PathBuf) -> String {
    replace_home_dir(current_dir, &dirs::home_dir().unwrap_or_default())
}

fn short(
    full_path: &PathBuf,
    home_dir: &PathBuf,
    git_path: Option<&Path>,
    underline_repo: bool,
    shell: &Shell,
) -> String {
    let git_path_length = git_path.map(|git_path| {
        let git_path = git_path.parent().unwrap(); // Remove ".git"
        let git_path = replace_home_dir(&git_path.to_path_buf(), &home_dir);
        git_path.split('/').count()
    });

    let full_path = replace_home_dir(&full_path, &home_dir);
    let full_path_length = full_path.split('/').count();

    full_path
        .split('/')
        .enumerate()
        .map(|(i, part)| {
            if git_path_length.map(|l| i == l - 1).unwrap_or(false) {
                // Don't truncate the repository or the final dir
                if underline_repo {
                    format!(
                        "{}{}{}",
                        wrap_no_change_cursor_position(Attribute::Underlined, shell),
                        part,
                        wrap_no_change_cursor_position(Attribute::NoUnderline, shell)
                    )
                } else {
                    part.to_owned()
                }
            } else if i == full_path_length - 1 {
                part.to_owned()
            } else {
                let p = part.get(0..1).unwrap_or("");
                if p == "." {
                    part.get(0..2).unwrap_or(p).to_string()
                } else {
                    p.to_string()
                }
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn long(current_dir: &PathBuf) -> String {
    format!("{}", current_dir.display())
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
            short(&current_dir, &home_dir, Some(&git_root), false, &Shell::Zsh),
            "~/a/b/repo/c/dxx".to_string()
        );

        let current_dir = PathBuf::from("/home/foo/axx/bxx/repo");
        assert_eq!(
            short(&current_dir, &home_dir, Some(&git_root), false, &Shell::Zsh),
            "~/a/b/repo".to_string()
        );
    }

    #[test]
    fn short_test_single_dir_repo() {
        let current_dir = PathBuf::from("/home/foo/axx");
        let home_dir = PathBuf::from("/home/foo");
        let git_root = Path::new("/home/foo/axx/.git");

        assert_eq!(
            short(&current_dir, &home_dir, Some(&git_root), false, &Shell::Zsh),
            "~/axx".to_string()
        );
    }

    #[test]
    fn short_test_root() {
        let current_dir = PathBuf::from("/foo/bar/axx/bxx/cxx/dxx");
        let home_dir = PathBuf::from("/home/baz");
        let git_root = Path::new("/foo/bar/axx/.git");

        assert_eq!(
            short(&current_dir, &home_dir, Some(&git_root), false, &Shell::Zsh),
            "/f/b/axx/b/c/dxx".to_string()
        );
    }

    #[test]
    fn short_test_root_no_repo() {
        let current_dir = PathBuf::from("/foo/bar/axx/bxx/cxx/dxx");
        let home_dir = PathBuf::from("/home/baz");

        assert_eq!(
            short(&current_dir, &home_dir, None, false, &Shell::Zsh),
            "/f/b/a/b/c/dxx".to_string()
        );
    }

    #[test]
    fn short_test_dot_dirs() {
        let current_dir = PathBuf::from("/.axx/./..xx/.dxx");
        let home_dir = PathBuf::from("/home/baz");

        assert_eq!(
            short(&current_dir, &home_dir, None, false, &Shell::Zsh),
            "/.a/./../.dxx".to_string()
        );
    }
}
