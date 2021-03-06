use anyhow::Result;

use std::collections::HashMap;
use std::fmt;

use crate::style::Style;
use crate::token::{self, Condition, Token};
use crate::Context;

mod cwd;
mod env;
mod git_branch;
mod git_commit;
mod git_stash;
mod git_status;
mod hostname;
mod jobs;
mod user;

#[cfg(test)]
mod tests;

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
            Token::Color(color) => components.push(Some(Component::Color(
                Style::from_color_token(&color, &context.shell).to_string(),
            ))),
            Token::Reset => components.push(Some(Component::ColorReset(
                Style::Reset(&context.shell).to_string(),
            ))),
            Token::Component { name, mut options } => {
                // Components should return Err when they encounter bad options, in other cases
                // they should log their errors and return None, this way the prompt can always be
                // rendered unless it's been incorrectly configured.
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

fn should_keep_group(group: &[Option<Component>]) -> bool {
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
