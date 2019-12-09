use crate::current_dir::CurrentDir;
use git2::Repository;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub struct GitRepo<'a> {
    current_dir: &'a CurrentDir,
    root: OnceCell<Option<PathBuf>>,
}

impl<'a> GitRepo<'a> {
    pub fn new(current_dir: &'a CurrentDir) -> Self {
        Self {
            current_dir: current_dir,
            root: OnceCell::new(),
        }
    }

    pub fn root(&self) -> Option<&PathBuf> {
        self.root
            .get_or_init(|| match Repository::discover(self.current_dir.get()) {
                Ok(repo) => Some(repo.path().to_path_buf()),
                Err(_) => None,
            })
            .as_ref()
    }
}
