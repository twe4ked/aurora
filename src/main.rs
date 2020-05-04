use aurora_prompt::Shell;

use anyhow::{Context, Result};
use clap::Clap;

static DEFAULT_CONFIG: &str = "{cwd} {git_branch} $ ";

#[derive(Debug, Clap)]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Clap)]
enum SubCommand {
    /// Outputs the prompt
    Run(Run),
    /// Outputs init shell function. To be called from your dotfiles. This will in turn call "run"
    Init(Init),
}

#[derive(Debug, Clap)]
pub struct Run {
    #[clap(short, long)]
    jobs: String,
    #[clap(short, long)]
    shell: Shell,
    #[clap(short, long, default_value = DEFAULT_CONFIG)]
    config: String,
    #[clap(long)]
    status: usize,
}

impl Run {
    fn jobs(&self) -> Option<String> {
        // https://github.com/clap-rs/clap/issues/1740
        if self.jobs.is_empty() || self.jobs == "__empty__" {
            None
        } else {
            Some(self.jobs.to_string())
        }
    }
}

#[derive(Debug, Clap)]
struct Init {
    #[clap(name = "shell")]
    shell: Shell,
    #[clap(name = "config", default_value = DEFAULT_CONFIG)]
    config: String,
}

fn main() {
    let options = Options::parse();

    match options.subcmd {
        SubCommand::Init(o) => init(o),
        SubCommand::Run(o) => run(o),
    }
    .unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });
}

fn init(options: Init) -> Result<()> {
    let script = match options.shell {
        Shell::Zsh => include_str!("init/init.zsh"),
        Shell::Bash => include_str!("init/init.bash"),
    };

    let path = std::env::current_exe().with_context(|| "could not return path to executable")?;

    let script = script.replace("__CMD__", &format!("\"{}\"", path.display()));
    let script = script.replace("__CONFIG__", &format!("'{}'", options.config));

    print!("{}", script);

    Ok(())
}

fn run(options: Run) -> Result<()> {
    use aurora_prompt::component;
    use aurora_prompt::parser;

    let tokens = parser::parse(&options.config)?;
    let tokens = component::evaluate_token_conditionals(tokens, options.status);

    let components = component::components_from_tokens(tokens, &options.shell, options.jobs());
    for component in components? {
        print!("{}", component);
    }
    Ok(())
}
