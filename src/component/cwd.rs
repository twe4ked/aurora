use crate::style;
use crate::Context;
use crate::Shell;

use anyhow::Result;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
enum Style {
    Default,
    Long,
    Short {
        underline_repo: bool,
        bold_repo: bool,
    },
}

fn parse_boolean(input: String) -> Result<bool> {
    match input.as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(anyhow::anyhow!("error: invalid boolean: {}", input)),
    }
}

fn extract_options(options: &mut HashMap<String, String>) -> Result<Style> {
    match options.remove("style") {
        Some(s) => match s.as_ref() {
            "default" => Ok(Style::Default),
            "short" => {
                let underline_repo = match options.remove("underline_repo") {
                    Some(s) => parse_boolean(s)?,
                    None => false,
                };
                let bold_repo = match options.remove("bold_repo") {
                    Some(s) => parse_boolean(s)?,
                    None => false,
                };
                Ok(Style::Short {
                    underline_repo,
                    bold_repo,
                })
            }
            "long" => Ok(Style::Long),
            _ => Err(anyhow::anyhow!("error: invalid style: {}", s)),
        },
        None => Ok(Style::Default),
    }
}

// Displays the current working directory.
//
// Options:
//
// style=default
//
// Default displays the full path with the home directory replaced with "~/"
//
// style=short
//
//      Shortens the current working directory
//
//      The short option has the home dir replaced and shortens every path part up to the current
//      directory. If you're in a Git repository it won't shorten the repository root directory.
//
//      Examples:
//
//      Standard directories within home:
//
//          /users/admin/home                                   ->  ~
//          /users/admin/home/a_directory                       ->  ~/some_directory
//          /users/admin/home/a_directory/other                 ->  ~/s/other
//
//      Repositories within home:
//
//          /users/admin/home/a_repo                            ->  ~/s/a_repo
//          /users/admin/home/a_repo/other                      ->  ~/s/a_repo/other
//          /users/admin/home/a_repo/other/new                  ->  ~/s/a_repo/o/new
//
//      A deeply nested repository from the root:
//
//          /repos/personal/aurora_prompt/src/components/cwd    ->  /r/p/aurora_prompt/s/c/cwd
//
// style=long
//
// Outputs the full path unmodified.
pub fn display(
    context: &Context,
    mut options: &mut HashMap<String, String>,
) -> Result<Option<String>> {
    let style = extract_options(&mut options)?;

    let output = match style {
        Style::Default => default(context.current_dir()),
        Style::Short {
            underline_repo,
            bold_repo,
        } => short(
            &context.current_dir(),
            &dirs::home_dir().unwrap_or_default(),
            context.git_repository().map(|r| r.path()),
            underline_repo,
            bold_repo,
            &context.shell,
        ),
        Style::Long => long(context.current_dir()),
    };

    Ok(Some(output))
}

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
    bold_repo: bool,
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
                // Don't truncate the repository

                use style::Style::{Bold, NoBold, NoUnderline, Underlined};

                let mut out = part.to_owned();

                if underline_repo {
                    out = format!("{}{}{}", Underlined(shell), out, NoUnderline(shell))
                }

                if bold_repo {
                    out = format!("{}{}{}", Bold(shell), out, NoBold(shell))
                }

                out
            } else if i == full_path_length - 1 {
                // Or the final dir
                part.to_owned()
            } else {
                // Truncate everything else
                let p = part.get(0..1).unwrap_or("");
                // If the path starts with a ".", let's grab the first two characters.
                //
                // Eg. ~/.config/shell -> ~/.c/shell
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
