use crate::component::Component;
use crate::style::Style;
use crate::token::Color;
use crate::Shell;

pub fn display(color: &Color, shell: &Shell) -> Option<Component> {
    let style = Style::from_color_token(&color, &shell);
    Some(Component::Color(style.to_string()))
}
