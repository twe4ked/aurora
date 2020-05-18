use crate::component::Component;
use crate::token::Color;
use crate::utility::wrap_no_change_cursor_position as wrap;
use crate::Shell;

use crossterm::style::{Color as CrosstermColor, SetForegroundColor};

pub fn display(color: &Color, shell: &Shell) -> Option<Component> {
    let color = match color {
        Color::Black => CrosstermColor::Black,
        Color::DarkGrey => CrosstermColor::DarkGrey,
        Color::Blue => CrosstermColor::Blue,
        Color::DarkBlue => CrosstermColor::DarkBlue,
        Color::Green => CrosstermColor::Green,
        Color::DarkGreen => CrosstermColor::DarkGreen,
        Color::Red => CrosstermColor::Red,
        Color::DarkRed => CrosstermColor::DarkRed,
        Color::Cyan => CrosstermColor::Cyan,
        Color::DarkCyan => CrosstermColor::DarkCyan,
        Color::Magenta => CrosstermColor::Magenta,
        Color::DarkMagenta => CrosstermColor::DarkMagenta,
        Color::Yellow => CrosstermColor::Yellow,
        Color::DarkYellow => CrosstermColor::DarkYellow,
        Color::White => CrosstermColor::White,
    };

    Some(Component::Color(wrap(SetForegroundColor(color), shell)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_wraps_green() {
        assert_green(Shell::Zsh, "%{\u{1b}[38;5;10m%}");
        assert_green(Shell::Bash, "\\[\u{1b}[38;5;10m\\]");
    }

    fn assert_green(shell: Shell, expected: &str) {
        match display(&Color::Green, &shell) {
            Some(Component::Color(green)) => assert_eq!(format!("{}", green), expected.to_string()),
            _ => unreachable!(),
        }
    }
}
