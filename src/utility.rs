use crate::Shell;
use std::fmt::Display;

// Include a string as a literal escape sequence. The string within the braces should not change
// the cursor position. Brace pairs can nest.
pub fn wrap_no_change_cursor_position<T: Display>(color: T, shell: &Shell) -> String {
    match shell {
        // %{...%}
        Shell::Zsh => format!("%{{{}%}}", color),
        // /[.../]
        Shell::Bash => format!("\\[{}\\]", color),
    }
}
