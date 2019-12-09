use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub struct CurrentDir {
    dir: OnceCell<PathBuf>,
}

impl CurrentDir {
    pub fn new() -> Self {
        Self {
            dir: OnceCell::new(),
        }
    }

    pub fn get(&self) -> &PathBuf {
        self.dir
            .get_or_init(|| std::env::current_dir().expect("unable to get current dir"))
    }
}
