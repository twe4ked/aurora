use crate::current_dir::CurrentDir;
use crate::error::Error;
use git2::Repository;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub struct GitRepo<'a> {
    current_dir: &'a CurrentDir,
    root: OnceCell<PathBuf>,
    repository: OnceCell<Repository>,
}

impl<'a> GitRepo<'a> {
    pub fn new(current_dir: &'a CurrentDir) -> Self {
        Self {
            current_dir: current_dir,
            root: OnceCell::new(),
            repository: OnceCell::new(),
        }
    }

    pub fn root(&self) -> Result<&PathBuf, Error> {
        self.root.get_or_try_init(|| -> Result<PathBuf, Error> {
            Ok(self.repository()?.path().to_path_buf())
        })
    }

    pub fn repository(&self) -> Result<&Repository, Error> {
        self.repository
            .get_or_try_init(|| -> Result<Repository, Error> {
                Repository::discover(self.current_dir.get()).map_err(Error::from)
            })
    }
}
