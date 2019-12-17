use crate::component::Component;
use crate::static_component;
use crossterm::style::{Color as CrosstermColor, ResetColor, SetForegroundColor};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Style {
    Color(String),
    Reset(String),
}

impl std::convert::From<&static_component::Color> for CrosstermColor {
    fn from(static_component_color: &static_component::Color) -> Self {
        match static_component_color {
            static_component::Color::Black => CrosstermColor::Black,
            static_component::Color::Blue => CrosstermColor::Blue,
            static_component::Color::Green => CrosstermColor::Green,
            static_component::Color::Red => CrosstermColor::Red,
            static_component::Color::Cyan => CrosstermColor::Cyan,
            static_component::Color::Magenta => CrosstermColor::Magenta,
            static_component::Color::Yellow => CrosstermColor::Yellow,
            static_component::Color::White => CrosstermColor::White,
            static_component::Color::Reset => unreachable!(),
        }
    }
}

pub fn display(static_component_color: &static_component::Color) -> Component {
    if static_component_color == &static_component::Color::Reset {
        return Component::Color(Style::Reset(wrap_in_zsh_no_change_cursor_position(
            ResetColor,
        )));
    }

    let crossterm_color = CrosstermColor::from(static_component_color);

    Component::Color(Style::Color(wrap_in_zsh_no_change_cursor_position(
        SetForegroundColor(crossterm_color),
    )))
}

const START: &str = "%{"; // %{ESC
const END: &str = "%}"; // %}

// %{...%}
//
// Include a string as a literal escape sequence. The string within the braces should not change
// the cursor position. Brace pairs can nest.
fn wrap_in_zsh_no_change_cursor_position<T: Display>(color: T) -> String {
    format!("{}{}{}", START, color, END)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_green() {
        if let Component::Color(Style::Color(green)) = display(&static_component::Color::Green) {
            assert_eq!(format!("{}", green), "%{\u{1b}[38;5;10m%}".to_string());
        } else {
            unreachable!();
        }
    }
}
