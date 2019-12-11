use crate::error::Error;
use crossterm::style::{Color as CrosstermColor, ResetColor, SetForegroundColor};

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
    pub fn display(&self) -> Result<String, Error> {
        if self == &Color::Reset {
            return Ok(format!("{}", ResetColor));
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
        Ok(format!("{}", SetForegroundColor(color)))
    }
}
