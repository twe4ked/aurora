use crate::component::Component;
use crate::token::StyleToken;
use crate::Shell;
use crossterm::style::{Color as CrosstermColor, ResetColor, SetForegroundColor};
use std::fmt::Display;

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

pub fn display(style_token: &StyleToken, shell: &Shell) -> Option<Component> {
    if style_token == &StyleToken::Reset {
        return Some(Component::ColorReset(wrap_no_change_cursor_position(
            ResetColor, shell,
        )));
    }

    let crossterm_color = CrosstermColor::from(style_token);

    Some(Component::Color(wrap_no_change_cursor_position(
        SetForegroundColor(crossterm_color),
        shell,
    )))
}

// Include a string as a literal escape sequence. The string within the braces should not change
// the cursor position. Brace pairs can nest.
fn wrap_no_change_cursor_position<T: Display>(color: T, shell: &Shell) -> String {
    match shell {
        // %{...%}
        Shell::Zsh => format!("%{{{}%}}", color),
        // /[.../]
        Shell::Bash => format!("\\[{}\\]", color),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_green() {
        if let Some(Component::Color(green)) = display(&StyleToken::Green, &Shell::Zsh) {
            assert_eq!(format!("{}", green), "%{\u{1b}[38;5;10m%}".to_string());
        } else {
            unreachable!();
        }

        if let Some(Component::Color(green)) = display(&StyleToken::Green, &Shell::Bash) {
            assert_eq!(format!("{}", green), "\\[\u{1b}[38;5;10m\\]".to_string());
        } else {
            unreachable!();
        }
    }
}
