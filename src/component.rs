use std::fmt;

pub mod character;
pub mod cwd;
pub mod git_branch;
pub mod git_commit;
pub mod git_stash;
pub mod jobs;
pub mod style;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(String),
    Style(style::Style),
    Cwd(String),
    GitBranch(String),
    GitCommit(String),
    GitStash(String),
    Jobs(String),
    Empty,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::Char(c)
            | Component::Style(style::Style::Color(c))
            | Component::Style(style::Style::Reset(c))
            | Component::Cwd(c)
            | Component::GitBranch(c)
            | Component::GitCommit(c)
            | Component::Jobs(c)
            | Component::GitStash(c) => write!(f, "{}", c),
            Component::Empty => write!(f, ""),
        }
    }
}

// A group is something between a Style::Color and a Style::Reset.
pub fn squash(components: Vec<Component>) -> Vec<Component> {
    let mut ret: Vec<Component> = Vec::new();
    let mut group: Vec<Component> = Vec::new();

    for component in components {
        match &component {
            Component::Char(_c) => {
                // Store every Char in the group, we're not sure if we want to squash them yet.
                group.push(component);
            }

            Component::Style(style::Style::Reset(_c)) => {
                // End group
                group = filter(group);
                group.push(component);
                ret.append(&mut group);
                group.clear();
            }

            Component::Style(style::Style::Color(_c)) => {
                group.push(component);

                // If we're already in a group, let's end the current one, and start a new one.
                if !group.is_empty() {
                    ret.append(&mut group);
                    group.clear();
                }
            }

            Component::Cwd(_)
            | Component::GitBranch(_)
            | Component::GitCommit(_)
            | Component::GitStash(_)
            | Component::Jobs(_)
            | Component::Empty => group.push(component),
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
        | Component::GitStash(_)
        | Component::Jobs(_) => true,
        _ => false,
    });

    let group_contains_none_value = group.iter().any(|c| match c {
        Component::Empty => true,
        _ => false,
    });

    let group_contains_all_char_and_or_color = group.iter().all(|c| match c {
        Component::Char(_) | Component::Style(_) => true,
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
    use crate::component::style::Style;

    #[test]
    fn test_squash_keep_1() {
        let components = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2 (Squash)
            Component::Style(Style::Color("red".to_string())),
            Component::Empty,
            Component::Char("c squash".to_string()),
            Component::Style(Style::Reset("reset".to_string())),
            // Group 3
            Component::Style(Style::Color("green".to_string())),
            Component::Char("d keep".to_string()),
            // Group 4
            Component::Style(Style::Color("blue".to_string())),
            Component::Char("e keep".to_string()),
            Component::Cwd("f keep".to_string()),
        ];
        let expected = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2 (Squash)
            Component::Style(Style::Color("red".to_string())),
            // XXX: Component::Empty,
            // XXX: Component::Char("c squash".to_string()),
            Component::Style(Style::Reset("reset".to_string())),
            // Group 3
            Component::Style(Style::Color("green".to_string())),
            Component::Char("d keep".to_string()),
            // Group 4
            Component::Style(Style::Color("blue".to_string())),
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
            Component::Style(Style::Color("blue".to_string())),
            Component::Char("c squash".to_string()),
            Component::Empty,
        ];
        let expected = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2
            Component::Style(Style::Color("blue".to_string())),
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
                Component::Style(Style::Color("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Style(Style::Reset("reset".to_string())),
            ]),
            vec![
                Component::Style(Style::Color("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Style(Style::Reset("reset".to_string())),
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
                Component::Style(Style::Color("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Style(Style::Reset("reset".to_string())),
            ]),
            vec![
                Component::Style(Style::Color("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Style(Style::Reset("reset".to_string())),
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
                Component::Style(Style::Color("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Empty,
                Component::Style(Style::Reset("reset".to_string())),
            ]),
            vec![
                Component::Style(Style::Color("green".to_string())),
                Component::Style(Style::Reset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_char_cwd_and_empty() {
        assert_eq!(
            filter(vec![
                Component::Style(Style::Color("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Empty,
                Component::Style(Style::Reset("reset".to_string())),
            ]),
            vec![
                Component::Style(Style::Color("green".to_string())),
                Component::Char("a keep".to_string()),
                Component::Cwd("b keep".to_string()),
                Component::Empty,
                Component::Style(Style::Reset("reset".to_string())),
            ]
        );
    }
}
