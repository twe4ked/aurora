mod component;
mod error;
mod parser;
mod static_component;

use git2::Repository;
use std::env;
use std::path::PathBuf;

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
    let current_dir = env::var("PWD")
        .map(PathBuf::from)
        .unwrap_or(env::current_dir().expect("unable to get current dir"));

    // TODO: Don't try to discover repository if nothing relies on it.
    let mut git_repository = Repository::discover(&current_dir).ok();

    // Generate a Component with an optional finished String.
    let components = output
        .iter()
        .map(|component| match component {
            static_component::Component::Char(c) => component::character::display(&c),
            static_component::Component::Style(style) => component::style::display(&style),
            static_component::Component::Cwd { style } => {
                component::cwd::display(&style, &current_dir, git_repository.as_ref())
            }
            static_component::Component::GitBranch => {
                component::git_branch::display(git_repository.as_ref())
            }
            static_component::Component::GitCommit => {
                component::git_commit::display(git_repository.as_ref())
            }
            static_component::Component::GitStash => {
                component::git_stash::display(git_repository.as_mut())
            }
        })
        .collect();

    // Squash any characters we don't need where a component has returned ::Empty.
    let components = component::squash(components);

    // Print components.
    for component in components {
        print!("{}", component);
    }
}
