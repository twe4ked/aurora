use crate::component::Component;
use crate::utility::wrap_no_change_cursor_position as wrap;
use crate::Shell;

use crossterm::style::ResetColor;

pub fn display(shell: &Shell) -> Option<Component> {
    Some(Component::ColorReset(wrap(ResetColor, shell)))
}
