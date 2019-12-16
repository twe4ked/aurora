use crate::component::Component;
use crate::parser;
use crossterm::style::{Color as CrosstermColor, ResetColor, SetForegroundColor};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Color {
    Black(String),
    Blue(String),
    Green(String),
    Red(String),
    Cyan(String),
    Magenta(String),
    Yellow(String),
    White(String),
    Reset(String),
}

impl std::convert::From<&parser::Color> for CrosstermColor {
    fn from(parser_color: &parser::Color) -> Self {
        match parser_color {
            parser::Color::Black => CrosstermColor::Black,
            parser::Color::Blue => CrosstermColor::Blue,
            parser::Color::Green => CrosstermColor::Green,
            parser::Color::Red => CrosstermColor::Red,
            parser::Color::Cyan => CrosstermColor::Cyan,
            parser::Color::Magenta => CrosstermColor::Magenta,
            parser::Color::Yellow => CrosstermColor::Yellow,
            parser::Color::White => CrosstermColor::White,
            parser::Color::Reset => unreachable!(),
        }
    }
}

pub fn display(parser_color: &parser::Color) -> Component {
    if parser_color == &parser::Color::Reset {
        return Component::Color(Color::Reset(wrap_in_zsh_no_change_cursor_position(
            ResetColor,
        )));
    }

    let crossterm_color = CrosstermColor::from(parser_color);

    let color = match crossterm_color {
        CrosstermColor::Black => Color::Black,
        CrosstermColor::Blue => Color::Blue,
        CrosstermColor::Green => Color::Green,
        CrosstermColor::Red => Color::Red,
        CrosstermColor::Cyan => Color::Cyan,
        CrosstermColor::Magenta => Color::Magenta,
        CrosstermColor::Yellow => Color::Yellow,
        CrosstermColor::White => Color::White,
        _ => unreachable!(),
    };

    Component::Color(color(wrap_in_zsh_no_change_cursor_position(
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
        if let Component::Color(Color::Green(green)) = display(&parser::Color::Green) {
            assert_eq!(format!("{}", green), "%{\u{1b}[38;5;10m%}".to_string());
        } else {
            unreachable!();
        }
    }
}
