use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(char),
    Cwd { style: CwdStyle },
}

#[derive(Debug, PartialEq)]
pub enum CwdStyle {
    Default,
    Long,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::Char(c) => write!(f, "{}", c),
            Component::Cwd { style } => {
                match std::env::current_dir() {
                    Ok(dir) => {
                        match style {
                            // Replace the home directory portion of the path with "~/"
                            CwdStyle::Default => {
                                let home_dir =
                                    dirs::home_dir().unwrap_or(std::path::PathBuf::new());
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
        }
    }
}
