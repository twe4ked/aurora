use crate::component::Component;
use crate::token::StyleToken;
use crate::utility::wrap_no_change_cursor_position as wrap;
use crate::Shell;
use crossterm::style::{Color as CrosstermColor, SetForegroundColor};

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
        }
    }
}

pub fn display(style_token: &StyleToken, shell: &Shell) -> Option<Component> {
    Some(Component::Color(wrap(
        SetForegroundColor(CrosstermColor::from(style_token)),
        shell,
    )))
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
