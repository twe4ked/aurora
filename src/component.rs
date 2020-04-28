use std::fmt;

use crate::token::Token;

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
}

pub fn run(token: &Token, options: &crate::Run) -> Option<Component> {
    match token {
        Token::Char(c) => character::display(*c),
        Token::Style(style) => style::display(&style, &options.shell),
        Token::Cwd(style) => cwd::display(&style),
        Token::GitBranch => git_branch::display(),
        Token::GitCommit => git_commit::display(),
        Token::GitStash => git_stash::display(),
        Token::Jobs => jobs::display(options.jobs()),
    }
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
        }
    }
}

// A group is something between a Style::Color and a Style::Reset.
pub fn squash(components: Vec<Option<Component>>) -> Vec<Option<Component>> {
    let mut ret: Vec<Option<Component>> = Vec::new();
    let mut group: Vec<Option<Component>> = Vec::new();

    for component in components {
        match &component {
            Some(Component::Char(_c)) => {
                // Store every Char in the group, we're not sure if we want to squash them yet.
                group.push(component);
            }

            Some(Component::Style(style::Style::Reset(_c))) => {
                // End group
                group = filter(group);
                group.push(component);
                ret.append(&mut group);
                group.clear();
            }

            Some(Component::Style(style::Style::Color(_c))) => {
                group.push(component);

                // If we're already in a group, let's end the current one, and start a new one.
                if !group.is_empty() {
                    ret.append(&mut group);
                    group.clear();
                }
            }

            _ => group.push(component),
        }
    }

    group = filter(group);
    ret.append(&mut group);
    ret
}

fn filter(group: Vec<Option<Component>>) -> Vec<Option<Component>> {
    let group_contains_some_value = group.iter().any(|c| match c {
        Some(Component::Style(_)) | Some(Component::Char(_)) => false,
        Some(_) => true,
        None => false,
    });

    let group_contains_none_value = group.iter().any(|c| match c {
        None => true,
        _ => false,
    });

    let group_contains_all_char_and_or_color = group.iter().all(|c| match c {
        Some(Component::Char(_)) | Some(Component::Style(_)) => true,
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
                Some(Component::Char(_)) | None => false,
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
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
            // Group 2 (Squash)
            Some(Component::Style(Style::Color("red".to_string()))),
            None,
            Some(Component::Char("c squash".to_string())),
            Some(Component::Style(Style::Reset("reset".to_string()))),
            // Group 3
            Some(Component::Style(Style::Color("green".to_string()))),
            Some(Component::Char("d keep".to_string())),
            // Group 4
            Some(Component::Style(Style::Color("blue".to_string()))),
            Some(Component::Char("e keep".to_string())),
            Some(Component::Cwd("f keep".to_string())),
        ];
        let expected = vec![
            // Group 1
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
            // Group 2 (Squash)
            Some(Component::Style(Style::Color("red".to_string()))),
            // XXX: None,
            // XXX: Some(Component::Char("c squash".to_string())),
            Some(Component::Style(Style::Reset("reset".to_string()))),
            // Group 3
            Some(Component::Style(Style::Color("green".to_string()))),
            Some(Component::Char("d keep".to_string())),
            // Group 4
            Some(Component::Style(Style::Color("blue".to_string()))),
            Some(Component::Char("e keep".to_string())),
            Some(Component::Cwd("f keep".to_string())),
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_squash_keep_empty_end() {
        let components = vec![
            // Group 1
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
            // Group 2
            Some(Component::Style(Style::Color("blue".to_string()))),
            Some(Component::Char("c squash".to_string())),
            None,
        ];
        let expected = vec![
            // Group 1
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
            // Group 2
            Some(Component::Style(Style::Color("blue".to_string()))),
            // XXX: Some(Component::Char("c squash".to_string())),
            // XXX: None,
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_filter_just_char() {
        assert_eq!(
            filter(vec![Some(Component::Char("a keep".to_string()))]),
            vec![Some(Component::Char("a keep".to_string()))]
        );
    }

    #[test]
    fn test_filter_just_char_ignores_color() {
        assert_eq!(
            filter(vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]),
            vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_cwd() {
        assert_eq!(
            filter(vec![
                Some(Component::Char("a keep".to_string())),
                Some(Component::Cwd("b keep".to_string())),
            ]),
            vec![
                Some(Component::Char("a keep".to_string())),
                Some(Component::Cwd("b keep".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_cwd_ignores_color() {
        assert_eq!(
            filter(vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                Some(Component::Cwd("b keep".to_string())),
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]),
            vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                Some(Component::Cwd("b keep".to_string())),
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_empty() {
        assert_eq!(
            filter(vec![Some(Component::Char("a keep".to_string())), None,]),
            vec![]
        );
    }

    #[test]
    fn test_filter_char_and_empty_ignores_color() {
        assert_eq!(
            filter(vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                None,
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]),
            vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]
        );
    }

    #[test]
    fn test_filter_char_cwd_and_empty() {
        assert_eq!(
            filter(vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                Some(Component::Cwd("b keep".to_string())),
                None,
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]),
            vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                Some(Component::Cwd("b keep".to_string())),
                None,
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]
        );
    }
}
