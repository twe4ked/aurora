use crate::component::Component;
use crate::static_component;
use crossterm::style::{Color as CrosstermColor, ResetColor, SetForegroundColor};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Style {
    Color(String),
    Reset(String),
}

impl std::convert::From<&static_component::Style> for CrosstermColor {
    fn from(static_component_color: &static_component::Style) -> Self {
        match static_component_color {
            static_component::Style::Black => CrosstermColor::Black,
            static_component::Style::DarkGrey => CrosstermColor::DarkGrey,
            static_component::Style::Blue => CrosstermColor::Blue,
            static_component::Style::DarkBlue => CrosstermColor::DarkBlue,
            static_component::Style::Green => CrosstermColor::Green,
            static_component::Style::DarkGreen => CrosstermColor::DarkGreen,
            static_component::Style::Red => CrosstermColor::Red,
            static_component::Style::DarkRed => CrosstermColor::DarkRed,
            static_component::Style::Cyan => CrosstermColor::Cyan,
            static_component::Style::DarkCyan => CrosstermColor::DarkCyan,
            static_component::Style::Magenta => CrosstermColor::Magenta,
            static_component::Style::DarkMagenta => CrosstermColor::DarkMagenta,
            static_component::Style::Yellow => CrosstermColor::Yellow,
            static_component::Style::DarkYellow => CrosstermColor::DarkYellow,
            static_component::Style::White => CrosstermColor::White,
            static_component::Style::Reset => unreachable!(),
        }
    }
}

pub fn display(static_component_color: &static_component::Style) -> Component {
    if static_component_color == &static_component::Style::Reset {
        return Component::Style(Style::Reset(wrap_in_zsh_no_change_cursor_position(
            ResetColor,
        )));
    }

    let crossterm_color = CrosstermColor::from(static_component_color);

    Component::Style(Style::Color(wrap_in_zsh_no_change_cursor_position(
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
        if let Component::Style(Style::Color(green)) = display(&static_component::Style::Green) {
            assert_eq!(format!("{}", green), "%{\u{1b}[38;5;10m%}".to_string());
        } else {
            unreachable!();
        }
    }
}
