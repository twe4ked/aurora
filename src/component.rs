#[derive(Debug, PartialEq)]
pub enum Component {
    Char(char),
    Cwd { style: CwdStyle },
}

#[derive(Debug, PartialEq)]
pub enum CwdStyle {
    Default,
    Long,
}
