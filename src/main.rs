mod component;
mod error;
mod parser;
mod token;

use clap::Clap;
use git2::Repository;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

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
    #[clap(name = "shell")]
    shell: Shell,
    #[clap(name = "config", default_value = DEFAULT_CONFIG)]
    config: String,
}

#[derive(Debug)]
pub enum Shell {
    Zsh,
    Bash,
}

impl FromStr for Shell {
    type Err = &'static str;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "zsh" => Ok(Shell::Zsh),
            "bash" => Ok(Shell::Bash),
            _ => Err("valid options are: bash, zsh"),
        }
    }
}

fn main() {
    let options = Options::parse();

    match options.subcmd {
        Some(SubCommand::Init(o)) => init(o),
        _ => prompt(options),
    }
}

fn init(init: Init) {
    let script = match init.shell {
        Shell::Zsh => include_str!("init/init.zsh"),
        Shell::Bash => include_str!("init/init.bash"),
    };

    let path = std::env::current_exe().expect("could not return path to executable");
    let path = format!("\"{}\"", path.display());

    let config = format!(" '{}'", init.config);
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

    let shell = match env::var("AURORA_SHELL") {
        Ok(s) => Shell::from_str(&s).unwrap(),
        _ => panic!("AURORA_SHELL not set"),
    };

    // Generate a Component with an optional finished String.
    use token::*;
    let components = output
        .iter()
        .map(|component| match component {
            Token::Char(c) => component::character::display(&c),
            Token::Style(style) => component::style::display(&style, &shell),
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
