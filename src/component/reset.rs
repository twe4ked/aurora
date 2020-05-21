use crate::style::Style;
use crate::Shell;

pub fn display(shell: &Shell) -> Option<String> {
    Some(Style::Reset(shell).to_string())
}
