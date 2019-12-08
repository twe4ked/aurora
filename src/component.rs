pub mod character;
pub mod color;
pub mod cwd;

#[derive(Debug, PartialEq)]
pub enum Component {
    Char(char),
    Color(color::Color),
    Cwd { style: cwd::CwdStyle },
}
