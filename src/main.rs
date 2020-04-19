mod component;
mod error;
mod parser;
mod token;

use clap::Clap;
use git2::Repository;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clap)]
struct Options {
    #[clap(short, long)]
    jobs: Option<String>,
    #[clap(
        name = "init .. prompt string",
        default_value = "{cwd} {git_branch} $ "
    )]
    args: Vec<String>,
}

fn main() {
    let options = Options::parse();

    if options.args.get(0) == Some(&"init".to_string()) {
        init(options);
    } else {
        prompt(options);
    }
}

/// Currently only supports Zsh
fn init(options: Options) {
    let config = match options.args.get(1) {
        Some(c) => format!(" '{}'", c),
        None => "".to_string(),
    };

    let path = std::env::current_exe().expect("could not return path to executable");
    let path = format!("\"{}\"", path.display());

    let script = include_str!("init/init.zsh");
    let script = script.replace("CMD", &path);
    let script = script.replace("CONFIG", &config);

    print!("{}", script);
}

fn prompt(options: Options) {
    let config = options.args.get(0).unwrap();
    let output = parser::parse(&config).unwrap().1;

    // TODO: Don't get current_dir if it's not needed.
    let current_dir = env::var("PWD")
        .map(PathBuf::from)
        .unwrap_or(env::current_dir().expect("unable to get current dir"));

    // TODO: Don't try to discover repository if nothing relies on it.
    let mut git_repository = Repository::discover(&current_dir).ok();

    // Generate a Component with an optional finished String.
    use token::*;
    let components = output
        .iter()
        .map(|component| match component {
            Token::Char(c) => component::character::display(&c),
            Token::Style(style) => component::style::display(&style),
            Token::Cwd { style } => {
                component::cwd::display(&style, &current_dir, git_repository.as_ref())
            }
            Token::GitBranch => component::git_branch::display(git_repository.as_ref()),
            Token::GitCommit => component::git_commit::display(git_repository.as_ref()),
            Token::GitStash => component::git_stash::display(git_repository.as_mut()),
            Token::Jobs => component::jobs::display(&options.jobs),
        })
        .collect();

    // Squash any characters we don't need where a component has returned ::Empty.
    let components = component::squash(components);

    // Print components.
    for component in components {
        print!("{}", component);
    }
}
