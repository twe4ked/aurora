use crate::component::Component;
use crate::token::StyleToken;
use crossterm::style::{Color as CrosstermColor, ResetColor, SetForegroundColor};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Style {
    Color(String),
    Reset(String),
}

impl std::convert::From<&StyleToken> for CrosstermColor {
    fn from(style_token: &StyleToken) -> Self {
        match style_token {
            StyleToken::Black => CrosstermColor::Black,
            StyleToken::DarkGrey => CrosstermColor::DarkGrey,
            StyleToken::Blue => CrosstermColor::Blue,
            StyleToken::DarkBlue => CrosstermColor::DarkBlue,
            StyleToken::Green => CrosstermColor::Green,
            StyleToken::DarkGreen => CrosstermColor::DarkGreen,
            StyleToken::Red => CrosstermColor::Red,
            StyleToken::DarkRed => CrosstermColor::DarkRed,
            StyleToken::Cyan => CrosstermColor::Cyan,
            StyleToken::DarkCyan => CrosstermColor::DarkCyan,
            StyleToken::Magenta => CrosstermColor::Magenta,
            StyleToken::DarkMagenta => CrosstermColor::DarkMagenta,
            StyleToken::Yellow => CrosstermColor::Yellow,
            StyleToken::DarkYellow => CrosstermColor::DarkYellow,
            StyleToken::White => CrosstermColor::White,
            StyleToken::Reset => unreachable!(),
        }
    }
}

pub fn display(style_token: &StyleToken) -> Component {
    if style_token == &StyleToken::Reset {
        return Component::Style(Style::Reset(wrap_in_zsh_no_change_cursor_position(
            ResetColor,
        )));
    }

    let crossterm_color = CrosstermColor::from(style_token);

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
        if let Component::Style(Style::Color(green)) = display(&StyleToken::Green) {
            assert_eq!(format!("{}", green), "%{\u{1b}[38;5;10m%}".to_string());
        } else {
            unreachable!();
        }
    }
}
