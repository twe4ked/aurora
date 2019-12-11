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

    for component in output {
        let component = match component {
            Component::Char(c) => component::character::display(&c),
            Component::Color(color) => color.display(),
            Component::Cwd { style } => style.display(&current_dir, git_repository.as_mut()),
            Component::GitBranch => component::git_branch::display(git_repository.as_mut()),
            Component::GitCommit => component::git_commit::display(git_repository.as_mut()),
            Component::GitStash => component::git_stash::display(git_repository.as_mut()),
        };

        print!("{}", component.unwrap_or(String::new()))
    }
}
