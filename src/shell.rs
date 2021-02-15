#[derive(Debug)]
pub enum Shell {
    Zsh,
    Bash,
    NoWrap,
}

impl std::str::FromStr for Shell {
    type Err = &'static str;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "zsh" => Ok(Shell::Zsh),
            "bash" => Ok(Shell::Bash),
            "no_wrap" => Ok(Shell::NoWrap), // Undocumented
            _ => Err("valid options are: bash, zsh\n"),
        }
    }
}
