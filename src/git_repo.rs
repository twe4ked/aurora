use crate::current_dir::CurrentDir;
use git2::Repository;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub struct GitRepo<'a> {
    current_dir: &'a CurrentDir,
    root: OnceCell<Option<PathBuf>>,
    repository: OnceCell<Option<Repository>>,
}

impl<'a> GitRepo<'a> {
    pub fn new(current_dir: &'a CurrentDir) -> Self {
        Self {
            current_dir: current_dir,
            root: OnceCell::new(),
            repository: OnceCell::new(),
        }
    }

    pub fn root(&self) -> Option<&PathBuf> {
        self.root
            .get_or_init(|| match self.repository() {
                Some(repo) => Some(repo.path().to_path_buf()),
                None => None,
            })
            .as_ref()
    }

    pub fn repository(&self) -> Option<&Repository> {
        self.repository
            .get_or_init(|| Repository::discover(self.current_dir.get()).ok())
            .as_ref()
    }
}
