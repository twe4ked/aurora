use crate::component::Component;
use crate::token::StyleToken;
use crate::Shell;
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

pub fn display(style_token: &StyleToken, shell: &Shell) -> Component {
    if style_token == &StyleToken::Reset {
        return Component::Style(Style::Reset(wrap_no_change_cursor_position(
            ResetColor, shell,
        )));
    }

    let crossterm_color = CrosstermColor::from(style_token);

    Component::Style(Style::Color(wrap_no_change_cursor_position(
        SetForegroundColor(crossterm_color),
        shell,
    )))
}

const START: &str = "%{"; // %{ESC
const END: &str = "%}"; // %}

fn wrap_no_change_cursor_position<T: Display>(color: T, shell: &Shell) -> String {
    match shell {
        Shell::Zsh => {
            // %{...%}
            //
            // Include a string as a literal escape sequence. The string within the braces should not change
            // the cursor position. Brace pairs can nest.
            format!("{}{}{}", START, color, END)
        }
    }
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
