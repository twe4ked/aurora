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
    #[clap(long)]
    jobs: String,
    #[clap(long)]
    shell: Shell,
    #[clap(long, default_value = DEFAULT_CONFIG)]
    config: String,
    #[clap(long)]
    status: usize,
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
        eprintln!("{}", err);
        std::process::exit(1);
    });
}

fn init(options: Init) -> Result<()> {
    let script = match options.shell {
        Shell::Zsh => include_str!("init/init.zsh"),
        Shell::Bash => include_str!("init/init.bash"),
        Shell::NoWrap => panic!("init not supported for no_wrap shell"),
    };

    let path = std::env::current_exe().with_context(|| "could not return path to executable")?;

    let script = script.replace("__CMD__", &format!("\"{}\"", path.display()));
    let script = script.replace("__CONFIG__", &format!("'{}'", options.config));

    print!("{}", script);

    Ok(())
}

fn run(options: Run) -> Result<()> {
    #[rustfmt::skip]
    let Run { config, shell, jobs, status } = options;

    // https://github.com/clap-rs/clap/issues/1740
    let jobs = if jobs.is_empty() || jobs == "__empty__" {
        None
    } else {
        Some(jobs)
    };

    for component in aurora_prompt::components(&config, shell, jobs, status)? {
        print!("{}", component);
    }

    Ok(())
}
