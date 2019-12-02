mod component;
mod parser;

fn main() {
    let config = std::env::args().nth(1).unwrap_or("{cwd} $".into());
    let output = parser::parse(&config).unwrap().1;

    for component in output {
        print!("{}", component);
    }
}
