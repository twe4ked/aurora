mod component;
mod error;
mod parser;

const DEFAULT_CONFIG: &str = "{cwd} $ ";

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.get(1) == Some(&"init".to_string()) {
        init(args);
    } else {
        prompt(args);
    }
}

/// Currently only supports Zsh
fn init(args: Vec<String>) {
    let config = match args.get(2) {
        Some(c) => format!(" '{}'", c),
        None => "".to_string(),
    };

    println!(
        r#"PROMPT="\$({}{})""#,
        std::env::current_exe()
            .expect("could not return path to executable")
            .display(),
        config
    )
}

fn prompt(args: Vec<String>) {
    let default = DEFAULT_CONFIG.to_string();
    let config = args.get(1).unwrap_or(&default);
    let output = parser::parse(&config).unwrap().1;

    for component in output {
        print!("{}", component);
    }
}
