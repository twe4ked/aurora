use crate::component::{ColorStartReset, Component};
use crossterm::style::{Color as CrosstermColor, ResetColor, SetForegroundColor};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    Reset,
}

impl Color {
    pub fn display(&self) -> Component {
        if self == &Color::Reset {
            return Component::Color(ColorStartReset::Reset(
                wrap_in_zsh_no_change_cursor_position(ResetColor),
            ));
        }

        let color = match self {
            Color::Black => CrosstermColor::Black,
            Color::Blue => CrosstermColor::Blue,
            Color::Green => CrosstermColor::Green,
            Color::Red => CrosstermColor::Red,
            Color::Cyan => CrosstermColor::Cyan,
            Color::Magenta => CrosstermColor::Magenta,
            Color::Yellow => CrosstermColor::Yellow,
            Color::White => CrosstermColor::White,
            Color::Reset => unreachable!(),
        };

        Component::Color(ColorStartReset::Start(
            wrap_in_zsh_no_change_cursor_position(SetForegroundColor(color)),
        ))
    }
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
        if let Component::Color(ColorStartReset::Start(green)) = Color::Green.display() {
            assert_eq!(format!("{}", green), "%{\u{1b}[38;5;10m%}".to_string());
        } else {
            unreachable!();
        }
    }
}
