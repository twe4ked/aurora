mod component;
mod error;
mod parser;
mod style;
mod token;

use anyhow::{Context as AnyhowContext, Result};
use git2::Repository;
use once_cell::sync::OnceCell;

use std::env;
use std::path::PathBuf;

pub struct Context {
    current_dir: OnceCell<PathBuf>,
    git_repository: OnceCell<Option<Repository>>,
    last_command_status: usize,
    backgrounded_jobs: Option<String>,
    shell: Shell,
}

impl Context {
    pub fn new(
        shell: Shell,
        last_command_status: usize,
        backgrounded_jobs: Option<String>,
    ) -> Self {
        Self {
            current_dir: OnceCell::new(),
            git_repository: OnceCell::new(),
            last_command_status,
            backgrounded_jobs,
            shell,
        }
    }

    pub fn current_dir(&self) -> &PathBuf {
        self.current_dir.get_or_init(|| {
            env::var("PWD")
                .map(PathBuf::from)
                .with_context(|| "unable to get current dir")
                .unwrap()
        })
    }

    pub fn git_repository(&self) -> Option<&Repository> {
        self.git_repository
            .get_or_init(|| Repository::discover(&self.current_dir()).ok())
            .as_ref()
    }

    pub fn git_repository_mut(&mut self) -> Option<&mut Repository> {
        self.git_repository();
        self.git_repository
            .get_mut()
            .expect("intitialized above")
            .as_mut()
    }
}

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
            _ => Err("valid options are: bash, zsh\n"),
        }
    }
}

pub fn components(
    config: &str,
    shell: Shell,
    jobs: Option<String>,
    status: usize,
) -> Result<Vec<component::Component>> {
    let tokens = parser::parse(config)?;

    let mut context = Context::new(shell, status, jobs);

    let components = component::components_from_tokens(tokens, &mut context)?;
    let components = component::squash(components);

    Ok(components)
}
