use std::fmt;

use crate::token::Token;
use crate::Shell;

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

pub fn components_from_tokens(
    tokens: &[Token],
    shell: &Shell,
    jobs: Option<String>,
) -> Vec<Component> {
    let mut components = Vec::new();

    for token in tokens.iter() {
        let component = match token {
            Token::Char(c) => character::display(*c),
            Token::Style(style) => style::display(&style, &shell),
            Token::Component { name, options } => match name.as_ref() {
                "git_branch" => git_branch::display(),
                "git_commit" => git_commit::display(),
                "git_stash" => git_stash::display(),
                "jobs" => jobs::display(jobs.clone()),
                "cwd" => {
                    use crate::component::cwd::CwdStyle;

                    let style = if let Some(value) = options.get("style") {
                        match value.as_ref() {
                            "default" => CwdStyle::Default,
                            "short" => CwdStyle::Short,
                            "long" => CwdStyle::Long,
                            _ => panic!("invalid style"),
                        }
                    } else {
                        CwdStyle::Default
                    };

                    cwd::display(&style)
                }
                _ => panic!("invalid component"),
            },
        };

        components.push(component);
    }

    squash(components)
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
fn squash(components: Vec<Option<Component>>) -> Vec<Component> {
    struct Ret(Vec<Component>);

    impl Ret {
        fn append(&mut self, group: &mut Vec<Option<Component>>) {
            self.0
                .append(&mut group.drain(0..).filter_map(|c| c).collect());
        }
    }

    let mut ret = Ret(Vec::new());
    let mut group: Vec<Option<Component>> = Vec::new();

    for component in components {
        group.push(component);

        if let Some(Component::Style(style::Style::Reset(_)))
        | Some(Component::Style(style::Style::Color(_))) = &group.last().unwrap()
        {
            // End current group
            filter(&mut group);
            ret.append(&mut group);
        }
    }

    // End the final group
    filter(&mut group);
    ret.append(&mut group);

    ret.0
}

fn filter(group: &mut Vec<Option<Component>>) {
    // Groups with just a Char and or Style should be kept:
    //
    // {red}>{reset}
    //  ^   ^
    //  |   ` Char
    //  ` Style
    let group_contains_something_other_than_char_or_style = !group.iter().all(|c| match c {
        // Check for Char
        Some(Component::Char(_)) => true,
        // Check for Style
        Some(Component::Style(_)) => true,
        _ => false,
    });

    // However, if the group also contains a None value, we want to run the filter.
    //
    // {red}+{git_stash}{reset}
    //      ^ ^
    //      | `None -- git_stash returned a None
    //      ` Char
    let group_contains_no_value = !group.iter().any(|c| match c {
        // We don't want to count Style or Char as we don't consider them "values"
        Some(Component::Style(_)) | Some(Component::Char(_)) => false,
        // Everything else is a "value"
        Some(_) => true,
        // Except None
        None => false,
    });

    if group_contains_something_other_than_char_or_style && group_contains_no_value {
        // Retain everything that isn't a Char or a None
        group.retain(|c| match c {
            // Remove Char
            Some(Component::Char(_)) => false,
            // Remove None
            None => false,
            // Keep everything else; Style, Cwd, etc.
            _ => true,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use style::Style;

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
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2 (Squash)
            Component::Style(Style::Color("red".to_string())),
            // XXX: None,
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
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
            // Group 2
            Some(Component::Style(Style::Color("blue".to_string()))),
            Some(Component::Char("c squash".to_string())),
            None,
        ];
        let expected = vec![
            // Group 1
            Component::Char("a keep".to_string()),
            Component::Cwd("b keep".to_string()),
            // Group 2
            Component::Style(Style::Color("blue".to_string())),
            // XXX: Some(Component::Char("c squash".to_string())),
            // XXX: None,
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_filter_just_char() {
        let mut group = vec![Some(Component::Char("a keep".to_string()))];
        filter(&mut group);
        assert_eq!(group, vec![Some(Component::Char("a keep".to_string()))]);
    }

    #[test]
    fn test_filter_just_char_ignores_color() {
        let mut group = vec![
            Some(Component::Style(Style::Color("green".to_string()))),
            Some(Component::Char("a keep".to_string())),
            Some(Component::Style(Style::Reset("reset".to_string()))),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Char("a keep".to_string())),
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_cwd() {
        let mut group = vec![
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Char("a keep".to_string())),
                Some(Component::Cwd("b keep".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_char_and_cwd_ignores_color() {
        let mut group = vec![
            Some(Component::Style(Style::Color("green".to_string()))),
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
            Some(Component::Style(Style::Reset("reset".to_string()))),
        ];
        filter(&mut group);
        assert_eq!(
            group,
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
        let mut group = vec![Some(Component::Char("a keep".to_string())), None];
        filter(&mut group);
        assert_eq!(group, vec![]);
    }

    #[test]
    fn test_filter_char_and_empty_ignores_color() {
        let mut group = vec![
            Some(Component::Style(Style::Color("green".to_string()))),
            Some(Component::Char("a keep".to_string())),
            None,
            Some(Component::Style(Style::Reset("reset".to_string()))),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Style(Style::Color("green".to_string()))),
                Some(Component::Style(Style::Reset("reset".to_string()))),
            ]
        );
    }

    #[test]
    fn test_filter_char_cwd_and_empty() {
        let mut group = vec![
            Some(Component::Style(Style::Color("green".to_string()))),
            Some(Component::Char("a keep".to_string())),
            Some(Component::Cwd("b keep".to_string())),
            None,
            Some(Component::Style(Style::Reset("reset".to_string()))),
        ];
        filter(&mut group);
        assert_eq!(
            group,
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
