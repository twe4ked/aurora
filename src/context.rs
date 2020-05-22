use anyhow::Context as AnyhowContext;
use git2::Repository;
use once_cell::sync::OnceCell;

use std::env;
use std::path::PathBuf;

use crate::Shell;

pub struct Context {
    pub current_dir: OnceCell<PathBuf>,
    pub git_repository: OnceCell<Option<Repository>>,
    pub last_command_status: usize,
    pub backgrounded_jobs: Option<String>,
    pub shell: Shell,
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
