use anyhow::Result;

use std::collections::HashMap;
use std::fmt;

use crate::token::{self, Condition, Token};
use crate::Shell;

pub mod cwd;
pub mod git_branch;
pub mod git_commit;
pub mod git_stash;
pub mod jobs;
pub mod style;

#[derive(Debug, PartialEq)]
pub enum Component {
    Static(String),
    Color(String),
    ColorReset(String),
    Computed(String),
}

fn components_from_token(
    token: Token,
    shell: &Shell,
    jobs: Option<&str>,
    status: usize,
) -> Result<Vec<Option<Component>>> {
    let mut ret = Vec::new();
    match token {
        Token::Static(s) => ret.push(Some(Component::Static(s.to_string()))),
        Token::Style(style) => ret.push(style::display(&style, &shell)),
        Token::Component { name, mut options } => {
            let c = match name {
                token::Component::GitBranch => git_branch::display(),
                token::Component::GitCommit => git_commit::display(),
                token::Component::GitStash => git_stash::display(),
                token::Component::Jobs => jobs::display(jobs.clone()),
                token::Component::Cwd => cwd::display(options.remove("style")),
            };

            if !options.is_empty() {
                return Err(anyhow::anyhow!("invalid options"));
            }

            ret.push(c);
        }
        Token::Conditional {
            condition,
            left,
            right,
        } => {
            let result = match condition {
                Condition::LastCommandStatus => status == 0,
            };
            if result {
                ret.append(&mut components_from_tokens(left, shell, jobs, status)?);
            } else {
                if let Some(right) = right {
                    ret.append(&mut components_from_tokens(right, shell, jobs, status)?);
                }
            }
        }
    };

    Ok(ret)
}

pub fn components_from_tokens(
    tokens: Vec<Token>,
    shell: &Shell,
    jobs: Option<&str>,
    status: usize,
) -> Result<Vec<Option<Component>>> {
    let mut components = Vec::new();

    for token in tokens.into_iter() {
        let mut c = components_from_token(token, shell, jobs, status)?;
        components.append(&mut c);
    }

    Ok(components)
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::Color(c)
            | Component::ColorReset(c)
            | Component::Static(c)
            | Component::Computed(c) => write!(f, "{}", c),
        }
    }
}

fn into_groups(components: Vec<Option<Component>>) -> Vec<Vec<Option<Component>>> {
    struct Groups {
        map: HashMap<usize, Vec<Option<Component>>>,
        current_group_index: usize,
    }

    impl Groups {
        fn new() -> Self {
            let mut map = HashMap::new();
            map.insert(0, Vec::new());
            Groups {
                map: map,
                current_group_index: 0,
            }
        }

        fn current_group_empty(&self) -> bool {
            match self.map.get(&self.current_group_index) {
                Some(g) => g.is_empty(),
                None => true,
            }
        }

        fn add_to_current_group(&mut self, component: Option<Component>) {
            let group = self
                .map
                .entry(self.current_group_index)
                .or_insert(Vec::new());
            group.push(component)
        }

        fn start_new_group(&mut self) {
            self.current_group_index += 1;
        }

        fn groups(mut self) -> Vec<Vec<Option<Component>>> {
            (0..=self.current_group_index)
                .filter_map(|i| self.map.remove(&i))
                .collect()
        }
    }

    let mut groups = Groups::new();

    for component in components {
        match component {
            Some(Component::Color(_)) => {
                if groups.current_group_empty() {
                    // If we're already in a new group
                    groups.add_to_current_group(component);
                } else {
                    // Otherwise start a new group
                    groups.start_new_group();
                    groups.add_to_current_group(component);
                }
            }
            Some(Component::ColorReset(_)) => {
                // Add the reset style to the end of the current group
                groups.add_to_current_group(component);
                // Then start a new group
                groups.start_new_group();
            }
            // Always push other components to the current group
            _ => groups.add_to_current_group(component),
        }
    }

    groups.groups()
}

pub fn squash(components: Vec<Option<Component>>) -> Vec<Component> {
    let mut groups = into_groups(components);

    let mut components = Vec::new();
    for mut group in groups.iter_mut() {
        filter(&mut group);
        components.append(&mut group.drain(0..).filter_map(|c| c).collect());
    }

    components
}

fn filter(group: &mut Vec<Option<Component>>) {
    // Groups with just a Static and or Color/ColorReset should be kept:
    //
    // {red}>{reset}
    //  ^   ^
    //  |   ` Static
    //  ` Color
    let group_contains_something_other_than_static_or_style = !group.iter().all(|c| match c {
        // Check for Color, ColorReset, or Static
        Some(Component::Static(_)) | Some(Component::Color(_)) | Some(Component::ColorReset(_)) => {
            true
        }
        _ => false,
    });

    // However, if the group also contains a None value, we want to run the filter.
    //
    // {red}+{git_stash}{reset}
    //      ^ ^
    //      | `None -- git_stash returned a None
    //      ` Static
    let group_contains_no_value = !group.iter().any(|c| match c {
        // We don't want to count Color, ColorReset, or Static as we don't consider them "values"
        Some(Component::Color(_)) | Some(Component::ColorReset(_)) | Some(Component::Static(_)) => {
            false
        }
        // Everything else is a "value"
        Some(_) => true,
        // Except None
        None => false,
    });

    if group_contains_something_other_than_static_or_style && group_contains_no_value {
        // Retain everything that isn't a Static or a None
        group.retain(|c| match c {
            // Remove Static
            Some(Component::Static(_)) => false,
            // Remove None
            None => false,
            // Keep everything else
            _ => true,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squash_keep_1() {
        let components = vec![
            // Group 1
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
            // Group 2 (Squash)
            Some(Component::Color("red".to_string())),
            None,
            Some(Component::Static("c squash".to_string())),
            Some(Component::ColorReset("reset".to_string())),
            // Group 3
            Some(Component::Color("green".to_string())),
            Some(Component::Static("d keep".to_string())),
            // Group 4
            Some(Component::Color("blue".to_string())),
            Some(Component::Static("e keep".to_string())),
            Some(Component::Computed("f keep".to_string())),
        ];
        let expected = vec![
            // Group 1
            Component::Static("a keep".to_string()),
            Component::Computed("b keep".to_string()),
            // Group 2 (Squash)
            Component::Color("red".to_string()),
            // XXX: None,
            // XXX: Component::Static("c squash".to_string()),
            Component::ColorReset("reset".to_string()),
            // Group 3
            Component::Color("green".to_string()),
            Component::Static("d keep".to_string()),
            // Group 4
            Component::Color("blue".to_string()),
            Component::Static("e keep".to_string()),
            Component::Computed("f keep".to_string()),
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_squash_keep_empty_end() {
        let components = vec![
            // Group 1
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
            // Group 2
            Some(Component::Color("blue".to_string())),
            Some(Component::Static("c squash".to_string())),
            None,
        ];
        let expected = vec![
            // Group 1
            Component::Static("a keep".to_string()),
            Component::Computed("b keep".to_string()),
            // Group 2
            Component::Color("blue".to_string()),
            // XXX: Some(Component::Static("c squash".to_string())),
            // XXX: None,
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_filter_just_static() {
        let mut group = vec![Some(Component::Static("a keep".to_string()))];
        filter(&mut group);
        assert_eq!(group, vec![Some(Component::Static("a keep".to_string()))]);
    }

    #[test]
    fn test_filter_just_static_ignores_color() {
        let mut group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            Some(Component::ColorReset("reset".to_string())),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a keep".to_string())),
                Some(Component::ColorReset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_static_and_cwd() {
        let mut group = vec![
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Static("a keep".to_string())),
                Some(Component::Computed("b keep".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_static_and_cwd_ignores_color() {
        let mut group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
            Some(Component::ColorReset("reset".to_string())),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a keep".to_string())),
                Some(Component::Computed("b keep".to_string())),
                Some(Component::ColorReset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_static_and_empty() {
        let mut group = vec![Some(Component::Static("a keep".to_string())), None];
        filter(&mut group);
        assert_eq!(group, vec![]);
    }

    #[test]
    fn test_filter_static_and_empty_ignores_color() {
        let mut group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            None,
            Some(Component::ColorReset("reset".to_string())),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::ColorReset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_filter_static_cwd_and_empty() {
        let mut group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
            None,
            Some(Component::ColorReset("reset".to_string())),
        ];
        filter(&mut group);
        assert_eq!(
            group,
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a keep".to_string())),
                Some(Component::Computed("b keep".to_string())),
                None,
                Some(Component::ColorReset("reset".to_string())),
            ]
        );
    }

    #[test]
    fn test_components_from_tokens() {
        use std::collections::HashMap;

        let mut options = HashMap::new();
        options.insert("foo".to_string(), "bar".to_string());

        let result = components_from_tokens(
            vec![Token::Component {
                name: token::Component::Jobs,
                options,
            }],
            &Shell::Zsh,
            None,
            0,
        );

        assert_eq!(result.unwrap_err().to_string(), "invalid options");
    }

    #[test]
    fn test_into_groups_single_group() {
        let components = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a".to_string())),
            Some(Component::Static("b".to_string())),
            None,
            Some(Component::ColorReset("reset".to_string())),
        ];

        let groups = into_groups(components);

        assert_eq!(
            groups,
            vec![vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a".to_string())),
                Some(Component::Static("b".to_string())),
                None,
                Some(Component::ColorReset("reset".to_string())),
            ],]
        );
    }

    #[test]
    fn test_into_groups_two_groups() {
        let components = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a".to_string())),
            Some(Component::Static("b".to_string())),
            None,
            Some(Component::ColorReset("reset".to_string())),
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a".to_string())),
        ];

        let groups = into_groups(components);

        assert_eq!(
            groups,
            vec![
                vec![
                    Some(Component::Color("green".to_string())),
                    Some(Component::Static("a".to_string())),
                    Some(Component::Static("b".to_string())),
                    None,
                    Some(Component::ColorReset("reset".to_string())),
                ],
                vec![
                    Some(Component::Color("green".to_string())),
                    Some(Component::Static("a".to_string())),
                ],
            ]
        );
    }

    #[test]
    fn test_into_groups_two_groups_no_reset() {
        let components = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a".to_string())),
            Some(Component::Static("b".to_string())),
            None,
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a".to_string())),
        ];

        let groups = into_groups(components);

        assert_eq!(
            groups,
            vec![
                vec![
                    Some(Component::Color("green".to_string())),
                    Some(Component::Static("a".to_string())),
                    Some(Component::Static("b".to_string())),
                    None,
                ],
                vec![
                    Some(Component::Color("green".to_string())),
                    Some(Component::Static("a".to_string())),
                ],
            ]
        );
    }

    #[test]
    fn test_into_groups_two_groups_no_color() {
        let components = vec![
            Some(Component::Static("a".to_string())),
            Some(Component::ColorReset("reset".to_string())),
            Some(Component::Static("b".to_string())),
        ];

        let groups = into_groups(components);

        assert_eq!(
            groups,
            vec![
                vec![
                    Some(Component::Static("a".to_string())),
                    Some(Component::ColorReset("reset".to_string())),
                ],
                vec![Some(Component::Static("b".to_string())),],
            ]
        );
    }
}
