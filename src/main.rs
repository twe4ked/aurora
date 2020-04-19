mod component;
mod error;
mod parser;
mod token;

use clap::Clap;
use git2::Repository;
use std::env;
use std::path::PathBuf;

static DEFAULT_CONFIG: &str = "{cwd} {git_branch} $ ";

#[derive(Debug, Clap)]
struct Options {
    #[clap(short, long)]
    jobs: Option<String>,
    #[clap(name = "config", default_value = DEFAULT_CONFIG)]
    config: String,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Debug, Clap)]
enum SubCommand {
    /// Outputs init shell function. To be called from your dotfiles.
    Init(Init),
}

#[derive(Debug, Clap)]
struct Init {
    #[clap(name = "config", default_value = DEFAULT_CONFIG)]
    config: String,
}

fn main() {
    let options = Options::parse();

    if let Some(subcmd) = options.subcmd {
        match subcmd {
            SubCommand::Init(o) => {
                init(o);
                return;
            }
        }
    } else {
        prompt(options);
    }
}

/// Currently only supports Zsh
fn init(init: Init) {
    let config = format!(" '{}'", init.config);

    let path = std::env::current_exe().expect("could not return path to executable");
    let path = format!("\"{}\"", path.display());

    let script = include_str!("init/init.zsh");
    let script = script.replace("CMD", &path);
    let script = script.replace("CONFIG", &config);

    print!("{}", script);
}

fn prompt(options: Options) {
    let output = parser::parse(&options.config).unwrap().1;

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
