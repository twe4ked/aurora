mod component;
mod error;
mod parser;
mod token;

use anyhow::{Context, Result};
use clap::Clap;
use git2::Repository;
use once_cell::sync::Lazy;

use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

static DEFAULT_CONFIG: &str = "{cwd} {git_branch} $ ";

pub static CURRENT_DIR: Lazy<Mutex<PathBuf>> = Lazy::new(|| {
    let current_dir = env::var("PWD")
        .map(PathBuf::from)
        .with_context(|| "unable to get current dir")
        .unwrap();
    Mutex::new(current_dir)
});

pub static GIT_REPOSITORY: Lazy<Mutex<Option<Repository>>> = Lazy::new(|| {
    // TODO: Re-use CURRENT_DIR here.
    let current_dir = env::var("PWD")
        .map(PathBuf::from)
        .with_context(|| "unable to get current dir")
        .unwrap();
    let r = Repository::discover(&current_dir).ok();
    Mutex::new(r)
});

#[derive(Debug, Clap)]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Clap)]
enum SubCommand {
    /// Outputs the prompt
    Run(Run),
    /// Outputs init shell function. To be called from your dotfiles. This will in turn call "run"
    Init(Init),
}

#[derive(Debug, Clap)]
pub struct Run {
    #[clap(short, long)]
    jobs: String,
    #[clap(short, long)]
    shell: Shell,
    #[clap(short, long, default_value = DEFAULT_CONFIG)]
    config: String,
}

impl Run {
    fn jobs(&self) -> Option<String> {
        // https://github.com/clap-rs/clap/issues/1740
        if self.jobs.is_empty() || self.jobs == "__empty__" {
            None
        } else {
            Some(self.jobs.to_string())
        }
    }
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
        SubCommand::Init(o) => init(o),
        SubCommand::Run(o) => run(o),
    }
    .unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });
}

fn init(options: Init) -> Result<()> {
    let script = match options.shell {
        Shell::Zsh => include_str!("init/init.zsh"),
        Shell::Bash => include_str!("init/init.bash"),
    };

    let path = std::env::current_exe().with_context(|| "could not return path to executable")?;
    let command = format!("\"{}\"", path.display());
    let script = script.replace("__CMD__", &command);

    let config = format!("'{}'", options.config);
    let script = script.replace("__CONFIG__", &config);

    print!("{}", script);

    Ok(())
}

fn run(options: Run) -> Result<()> {
    let output = parser::parse(&options.config)?;

    let components = output
        .iter()
        .map(|component| component::run(component, &options))
        .collect();

    let components = component::squash(components);

    for component in components {
        print!("{}", component);
    }

    Ok(())
}
