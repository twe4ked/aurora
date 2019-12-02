mod component;
mod parser;

use component::Component;

fn main() {
    let config = std::env::args().nth(1).unwrap_or("{cwd} $".into());
    let output = parser::parse(&config).unwrap().1;

    for component in output {
        match component {
            Component::Char(c) => print!("{}", c),
            Component::Cwd { style } => {
                match std::env::current_dir() {
                    Ok(dir) => {
                        match style {
                            // Replace the home directory portion of the path with "~/"
                            component::CwdStyle::Default => {
                                let home_dir =
                                    dirs::home_dir().unwrap_or(std::path::PathBuf::new());
                                match dir.strip_prefix(home_dir) {
                                    Ok(dir) => print!("~/{}", dir.display()),
                                    // Unable to strip the prefix, fall back to full path
                                    Err(_) => print!("{}", dir.display()),
                                }
                            }
                            component::CwdStyle::Long => print!("{}", dir.display()),
                        }
                    }
                    Err(_) => { /* unable to read current directory */ }
                }
            }
        }
    }
}
