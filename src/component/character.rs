use crate::error::Error;

pub fn display(c: &char) -> Result<String, Error> {
    Ok(format!("{}", c))
}
