use crate::component::Component;
use anyhow::Result;
use std::collections::HashMap;

pub fn display(options: &mut HashMap<String, String>) -> Result<Option<Component>> {
    options.remove("name").map_or_else(
        || Err(anyhow::anyhow!("error: missing environment variable name")),
        |name| Ok(std::env::var(name).ok().map(Component::Computed)),
    )
}
