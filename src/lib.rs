mod component;
mod error;
mod parser;
mod token;

use anyhow::{Context, Result};
use git2::Repository;
use once_cell::sync::Lazy;

use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

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

#[derive(Debug)]
pub enum Shell {
    Zsh,
    Bash,
}

impl std::str::FromStr for Shell {
    type Err = &'static str;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "zsh" => Ok(Shell::Zsh),
            "bash" => Ok(Shell::Bash),
            _ => Err("valid options are: bash, zsh"),
        }
    }
}

pub fn components(
    config: &str,
    shell: &Shell,
    jobs: Option<String>,
    status: usize,
) -> Result<Vec<component::Component>> {
    let tokens = parser::parse(config)?;

    let components = component::components_from_tokens(tokens, shell, jobs.as_deref(), status)?;

    Ok(component::squash(components))
}
