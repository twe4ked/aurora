mod component;
mod error;
mod parser;

use component::Component;
use git2::Repository;

const DEFAULT_CONFIG: &str = "{cwd} {git_branch} $ ";

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.get(1) == Some(&"init".to_string()) {
        init(args);
    } else {
        prompt(args);
    }
}

/// Currently only supports Zsh
fn init(args: Vec<String>) {
    let config = match args.get(2) {
        Some(c) => format!(" '{}'", c),
        None => "".to_string(),
    };

    println!(
        r#"PROMPT="\$({}{})""#,
        std::env::current_exe()
            .expect("could not return path to executable")
            .display(),
        config
    )
}

fn prompt(args: Vec<String>) {
    let default = DEFAULT_CONFIG.to_string();
    let config = args.get(1).unwrap_or(&default);
    let output = parser::parse(&config).unwrap().1;

    // TODO: Don't get current_dir if it's not needed.
    let current_dir = std::env::current_dir().expect("unable to get current dir");

    // TODO: Don't try to discover repository if nothing relies on it.
    let mut git_repository = Repository::discover(&current_dir).ok();

    let components = output
        .iter()
        .map(|component| match component {
            parser::Component::Char(c) => component::character::display(&c),
            parser::Component::Color(color) => component::color::display(&color),
            parser::Component::Cwd { style } => {
                style.display(&current_dir, git_repository.as_ref())
            }
            parser::Component::GitBranch => component::git_branch::display(git_repository.as_ref()),
            parser::Component::GitCommit => component::git_commit::display(git_repository.as_ref()),
            parser::Component::GitStash => component::git_stash::display(git_repository.as_mut()),
        })
        .collect();

    let components = squash(components);

    for component in components {
        print!("{}", component);
    }
}

// A group is something between a Color and a ColorReset OR between a Squash and a Normal(Some(_))
fn squash(components: Vec<Component>) -> Vec<Component> {
    let mut ret: Vec<Component> = Vec::new();
    let mut group: Vec<Component> = Vec::new();

    for component in components {
        match &component {
            Component::Char(_c) => {
                // Store every Char in the group, we're not sure if we want to squash them yet.
                group.push(component);
            }

            Component::Color(component::color::Color::Reset(_c)) => {
                // End group
                group = filter(group);

                group.push(component);
                ret.append(&mut group);
                group = Vec::new();
            }

            Component::Color(_c) => {
                group.push(component);

                // If we're already in a group, let's end the current one, and start a new one.
                if !group.is_empty() {
                    ret.append(&mut group);
                    group = Vec::new();
                }
            }

            Component::Cwd(_)
            | Component::GitBranch(_)
            | Component::GitCommit(_)
            | Component::GitStash(_) => {
                group.push(component);
            }
            Component::Empty => group.push(component),
        }
    }

    group = filter(group);
    ret.append(&mut group);
    ret
}

fn filter(group: Vec<Component>) -> Vec<Component> {
    let group_contains_some_value = group.iter().any(|c| match c {
        Component::Cwd(_)
        | Component::GitBranch(_)
        | Component::GitCommit(_)
        | Component::GitStash(_) => true,
        _ => false,
    });

    let group_contains_none_value = group.iter().any(|c| match c {
        Component::Empty => true,
        _ => false,
    });

    let group_contains_all_char_and_or_color = group.iter().all(|c| match c {
        Component::Char(_) | Component::Color(_) => true,
        _ => false,
    });

    if !group_contains_none_value
        || group_contains_all_char_and_or_color
        || group_contains_some_value
    {
        group
    } else {
        group
            .into_iter()
            .filter(|c| match c {
                Component::Char(_) | Component::Empty => false,
                _ => true,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::color::Color;

    #[test]
    fn test_squash_keep_1() {
        let components = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2 (Squash)
            Component::Color(Color::Red("red".to_string())),
            Component::Empty,
            Component::Char("c squash".to_string()),
            Component::Color(Color::Reset("reset".to_string())),
            // Group 3
            Component::Color(Color::Green("green".to_string())),
            Component::Char("d keep".to_string()),
            // Group 4
            Component::Color(Color::Blue("blue".to_string())),
            Component::Char("e keep".to_string()),
            Component::Cwd("f keep".to_string()),
        ];
        let expected = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2 (Squash)
            Component::Color(Color::Red("red".to_string())),
            // XXX: Component::Empty,
            // XXX: Component::Char("c squash".to_string()),
            Component::Color(Color::Reset("reset".to_string())),
            // Group 3
            Component::Color(Color::Green("green".to_string())),
            Component::Char("d keep".to_string()),
            // Group 4
            Component::Color(Color::Blue("blue".to_string())),
            Component::Char("e keep".to_string()),
            Component::Cwd("f keep".to_string()),
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_squash_keep_empty_end() {
        let components = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2
            Component::Color(Color::Blue("blue".to_string())),
            Component::Char("c squash".to_string()),
            Component::Empty,
        ];
        let expected = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2
            Component::Color(Color::Blue("blue".to_string())),
            // XXX: Component::Char("c squash".to_string()),
            // XXX: Component::Empty,
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_filter_just_char() {
        assert_eq!(
            filter(vec![Component::Char("a keep".to_string())]),
            vec![Component::Char("a keep".to_string())]
        );
    }

    #[test]
    fn test_filter_just_char_ignores_color() {
        assert_eq!(
            filter(vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Color(Color::Reset("reset".to_string())),
            ]),
            vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Color(Color::Reset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_cwd() {
        assert_eq!(
            filter(vec![
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
            ]),
            vec![
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_cwd_ignores_color() {
        assert_eq!(
            filter(vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Color(Color::Reset("reset".to_string())),
            ]),
            vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Color(Color::Reset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_empty() {
        assert_eq!(
            filter(vec![
                Component::Char("a keep".to_string()),
                Component::Empty,
            ]),
            vec![]
        );
    }

    #[test]
    fn test_filter_char_and_empty_ignores_color() {
        assert_eq!(
            filter(vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Empty,
                Component::Color(Color::Reset("reset".to_string())),
            ]),
            vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Color(Color::Reset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_char_cwd_and_empty() {
        assert_eq!(
            filter(vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Empty,
                Component::Color(Color::Reset("reset".to_string())),
            ]),
            vec![
                Component::Color(Color::Green("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Empty,
                Component::Color(Color::Reset("reset".to_string())),
            ]
        );
    }
}
