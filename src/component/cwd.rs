use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CwdStyle {
    Default,
    Long,
}

pub fn display(f: &mut fmt::Formatter<'_>, style: &CwdStyle) -> fmt::Result {
    match std::env::current_dir() {
        Ok(dir) => {
            match style {
                // Replace the home directory portion of the path with "~/"
                CwdStyle::Default => {
                    let home_dir = dirs::home_dir().unwrap_or(std::path::PathBuf::new());
                    match dir.strip_prefix(home_dir) {
                        Ok(dir) => write!(f, "~/{}", dir.display()),
                        // Unable to strip the prefix, fall back to full path
                        Err(_) => write!(f, "{}", dir.display()),
                    }
                }
                CwdStyle::Long => write!(f, "{}", dir.display()),
            }
        }
        // unable to read current directory
        Err(_) => Ok(()),
    }
}
