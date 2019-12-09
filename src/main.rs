mod component;
mod current_dir;
mod error;
mod git_repo;
mod parser;

use component::Component;
use current_dir::CurrentDir;
use git_repo::GitRepo;

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
    let current_dir = CurrentDir::new();
    let git_repo = GitRepo::new(&current_dir);

    for component in output {
        let component = match component {
            Component::Char(c) => component::character::display(&c),
            Component::Color(color) => color.display(),
            Component::Cwd { style } => style.display(&current_dir, &git_repo),
            Component::GitBranch => component::git_branch::display(&git_repo),
        };

        print!("{}", component.unwrap_or(String::new()))
    }
}
