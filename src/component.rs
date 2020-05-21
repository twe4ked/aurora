use anyhow::Result;

use std::collections::HashMap;
use std::fmt;

use crate::token::{self, Condition, Token};
use crate::Context;

pub mod color;
pub mod cwd;
pub mod env;
pub mod git_branch;
pub mod git_commit;
pub mod git_stash;
pub mod git_status;
pub mod hostname;
pub mod jobs;
pub mod reset;
pub mod user;

#[derive(Debug, PartialEq)]
enum Component {
    Static(String),
    Color(String),
    ColorReset(String),
    Computed(String),
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

pub fn components(tokens: Vec<Token>, mut context: &mut Context) -> Result<Vec<String>> {
    let components = components_from_tokens(tokens, &mut context)?;
    let components = squash(components);
    let components = components.iter().map(|c| c.to_string()).collect();

    Ok(components)
}

fn components_from_tokens(
    tokens: Vec<Token>,
    mut context: &mut Context,
) -> Result<Vec<Option<Component>>> {
    let mut components = Vec::new();

    for token in tokens.into_iter() {
        match token {
            Token::Static(s) => components.push(Some(Component::Static(s))),
            Token::Color(color) => {
                components.push(color::display(&color, &context.shell).map(Component::Color))
            }
            Token::Reset => {
                components.push(reset::display(&context.shell).map(Component::ColorReset))
            }
            Token::Component { name, mut options } => {
                let c = match name {
                    token::Component::GitBranch => git_branch::display(&context),
                    token::Component::GitCommit => git_commit::display(&context),
                    token::Component::GitStash => git_stash::display(&mut context),
                    token::Component::GitStatus => git_status::display(&context)?,
                    token::Component::Hostname => hostname::display(),
                    token::Component::Jobs => jobs::display(context.backgrounded_jobs.as_deref()),
                    token::Component::Cwd => cwd::display(&context, &mut options)?,
                    token::Component::Env => env::display(&mut options)?,
                    token::Component::User => user::display(),
                };

                // Components should use all the options they are given by removing them from the
                // collection.
                if !options.is_empty() {
                    let options = options
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ");

                    return Err(anyhow::anyhow!("error: invalid options: {}", options));
                }

                components.push(c.map(Component::Computed));
            }
            Token::Conditional {
                condition,
                left,
                right,
            } => {
                let result = match condition {
                    Condition::LastCommandStatus => context.last_command_status == 0,
                    Condition::EnvironmentVariable(var_name) => std::env::var(var_name).is_ok(),
                };
                if result {
                    components.append(&mut components_from_tokens(left, context)?);
                } else if let Some(right) = right {
                    components.append(&mut components_from_tokens(right, context)?);
                }
            }
        };
    }

    Ok(components)
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
                map,
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
                .or_insert_with(Vec::new);
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

fn squash(components: Vec<Option<Component>>) -> Vec<Component> {
    into_groups(components)
        .into_iter()
        .filter(|g| should_keep_group(&g))
        .flatten()
        .filter_map(|c| c)
        .collect()
}

fn should_keep_group(group: &Vec<Option<Component>>) -> bool {
    // Groups with just a Static and or Color/ColorReset should be kept:
    //
    // {red}>{reset}
    //  ^   ^
    //  |   ` Static
    //  ` Color
    let group_contains_only_static_or_color_or_color_reset = group.iter().all(|c| match c {
        Some(Component::Color(_)) | Some(Component::ColorReset(_)) | Some(Component::Static(_)) => {
            true
        }
        _ => false,
    });

    // If the group contains at least one computer value we want to keep it:
    //
    // {red}+{git_stash}{reset}
    //      ^ ^
    //      | `None -- git_stash returned a None
    //      ` Static
    let group_contains_a_computed_value = group.iter().any(|c| match c {
        Some(Component::Computed(_)) => true,
        _ => false,
    });

    group_contains_only_static_or_color_or_color_reset || group_contains_a_computed_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Shell;

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
            // XXX: Component::Color("red".to_string()),
            // XXX: None,
            // XXX: Component::Static("c squash".to_string()),
            // XXX: Component::ColorReset("reset".to_string()),
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
            // XXX: Component::Color("blue".to_string()),
            // XXX: Some(Component::Static("c squash".to_string())),
            // XXX: None,
        ];
        assert_eq!(squash(components), expected);
    }

    #[test]
    fn test_filter_just_static() {
        let group = vec![Some(Component::Static("a keep".to_string()))];
        assert!(should_keep_group(&group));
    }

    #[test]
    fn test_filter_just_static_ignores_color() {
        let group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            Some(Component::ColorReset("reset".to_string())),
        ];
        assert!(should_keep_group(&group));
    }

    #[test]
    fn test_filter_static_and_cwd() {
        let group = vec![
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
        ];
        assert!(should_keep_group(&group));
    }

    #[test]
    fn test_filter_static_and_cwd_ignores_color() {
        let group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
            Some(Component::ColorReset("reset".to_string())),
        ];
        assert!(should_keep_group(&group));
    }

    #[test]
    fn test_filter_static_and_empty() {
        let group = vec![Some(Component::Static("a keep".to_string())), None];
        assert!(!should_keep_group(&group));
    }

    #[test]
    fn test_filter_static_and_empty_removes_colors() {
        let group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            None,
            Some(Component::ColorReset("reset".to_string())),
        ];
        assert!(!should_keep_group(&group));
    }

    #[test]
    fn test_filter_static_cwd_and_empty() {
        let group = vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a keep".to_string())),
            Some(Component::Computed("b keep".to_string())),
            None,
            Some(Component::ColorReset("reset".to_string())),
        ];
        assert!(should_keep_group(&group));
    }

    #[test]
    fn test_components_from_tokens() {
        use std::collections::HashMap;

        let mut options = HashMap::new();
        options.insert("foo".to_string(), "bar".to_string());

        let mut context = Context::new(Shell::Zsh, 0, None);
        let result = components_from_tokens(
            vec![Token::Component {
                name: token::Component::Jobs,
                options,
            }],
            &mut context,
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            "error: invalid options: foo=bar"
        );
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
