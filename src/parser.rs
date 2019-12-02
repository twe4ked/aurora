use crate::component::Component;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::many0;
use nom::IResult;

fn expression(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{cwd}")(input)?;
    Ok((input, Component::Cwd))
}

fn any_char(input: &str) -> IResult<&str, Component> {
    let (input, output) = anychar(input)?;
    Ok((input, Component::Char(output)))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Component>> {
    many0(alt((expression, any_char)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(parse(&"{cwd}").unwrap().1, vec![Component::Cwd]);
        assert_eq!(
            parse(&"{cwd} $").unwrap().1,
            vec![Component::Cwd, Component::Char(' '), Component::Char('$')]
        );
    }
}
