use crossterm::style::{Attribute, Color, ResetColor, SetForegroundColor};

use crate::Shell;

use std::fmt;

pub enum Style<'a> {
    Color(&'a Shell, Color),
    Reset(&'a Shell),
    Underlined(&'a Shell),
    NoUnderline(&'a Shell),
    Bold(&'a Shell),
    NoBold(&'a Shell),
}

impl<'a> Style<'a> {
    pub fn from_color_token(color: &Color, shell: &'a Shell) -> Self {
        match color {
            Color::Reset | Color::Rgb { .. } => panic!("unsupported color"),
            _ => Style::Color(shell, *color),
        }
    }
}

impl fmt::Display for Style<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Style::Color(shell, color) => write(f, *shell, SetForegroundColor(*color)),
            Style::Reset(shell) => write(f, *shell, ResetColor),
            Style::Underlined(shell) => write(f, *shell, Attribute::Underlined),
            Style::NoUnderline(shell) => write(f, *shell, Attribute::NoUnderline),
            Style::Bold(shell) => write(f, *shell, Attribute::Bold),
            Style::NoBold(shell) => write(f, *shell, Attribute::NoBold),
        }
    }
}

// Include a string as a literal escape sequence. The string within the braces should not change
// the cursor position. Brace pairs can nest.
fn write<T>(f: &mut fmt::Formatter<'_>, shell: &Shell, style: T) -> fmt::Result
where
    T: fmt::Display,
{
    match shell {
        // %{...%}
        Shell::Zsh => write!(f, "%{{{}%}}", style),
        // /[.../]
        Shell::Bash => write!(f, "\\[{}\\]", style),
        //   ...
        Shell::NoWrap => write!(f, "{}", style),
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
