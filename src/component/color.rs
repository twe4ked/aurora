use crate::style::Style;
use crate::token::Color;
use crate::Shell;

pub fn display(color: &Color, shell: &Shell) -> Option<String> {
    Some(Style::from_color_token(&color, &shell).to_string())
}
