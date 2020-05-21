use crossterm::style::{Attribute, Color as CrosstermColor, ResetColor, SetForegroundColor};

use crate::token::Color;
use crate::Shell;

use std::fmt;

pub enum Style {
    Color(Shell, CrosstermColor),
    Reset(Shell),
    Underlined(Shell),
    NoUnderline(Shell),
}

impl Style {
    pub fn from_color_token(color: &Color, shell: &Shell) -> Self {
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

        Style::Color(*shell, color)
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Style::Color(shell, color) => write(f, *shell, SetForegroundColor(*color)),
            Style::Reset(shell) => write(f, *shell, ResetColor),
            Style::Underlined(shell) => write(f, *shell, Attribute::Underlined),
            Style::NoUnderline(shell) => write(f, *shell, Attribute::NoUnderline),
        }
    }
}

// Include a string as a literal escape sequence. The string within the braces should not change
// the cursor position. Brace pairs can nest.
fn write<T>(f: &mut fmt::Formatter<'_>, shell: Shell, style: T) -> fmt::Result
where
    T: fmt::Display,
{
    match shell {
        // %{...%}
        Shell::Zsh => write!(f, "%{{{}%}}", style),
        // /[.../]
        Shell::Bash => write!(f, "\\[{}\\]", style),
    }
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
        let green = Style::from_color_token(&Color::Green, &shell);
        assert_eq!(format!("{}", green), expected.to_string())
    }
}
