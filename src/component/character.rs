use crate::error::Error;

/// # Notes
///
/// * Does not return `Err` and you can safely unwrap.
pub fn display(c: &char) -> Result<String, Error> {
    Ok(format!("{}", c))
}
