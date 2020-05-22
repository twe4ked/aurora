mod component;
mod context;
mod parser;
mod shell;
mod style;
mod token;

use anyhow::Result;

use context::Context;
pub use shell::Shell;

pub fn components(
    config: &str,
    shell: Shell,
    jobs: Option<String>,
    status: usize,
) -> Result<Vec<String>> {
    let tokens = parser::parse(config)?;

    let mut context = Context::new(shell, status, jobs);
    let components = component::components(tokens, &mut context)?;

    Ok(components)
}
