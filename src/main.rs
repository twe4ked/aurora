mod component;
mod parser;

use component::Component;

fn main() {
    let config = std::env::args().nth(1).unwrap_or("{cwd} $".into());
    let output = parser::parse(&config).unwrap().1;

    for component in output {
        match component {
            Component::Char(c) => print!("{}", c),
            Component::Cwd => {
                if let Ok(dir) = std::env::current_dir() {
                    print!("{}", dir.to_string_lossy())
                }
            }
        }
    }
}
