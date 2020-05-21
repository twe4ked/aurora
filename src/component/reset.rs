use crate::component::Component;
use crate::style::Style;
use crate::Shell;

pub fn display(shell: &Shell) -> Option<Component> {
    Some(Component::ColorReset(Style::Reset(*shell).to_string()))
}
