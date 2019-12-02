use crate::component;
use crate::component::Component;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::many0;
use nom::IResult;

fn cwd(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{cwd}")(input)?;
    let style = component::CwdStyle::Default;
    Ok((input, Component::Cwd { style: style }))
}

fn cwd_with_style(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{cwd style=")(input)?;
    let (input, output) = alt((tag("default"), tag("long")))(input)?;
    let style = match output {
        "default" => component::CwdStyle::Default,
        "long" => component::CwdStyle::Long,
        _ => panic!("invalid style"),
    };
    let (input, _) = tag("}")(input)?;

    Ok((input, Component::Cwd { style: style }))
}

fn expression(input: &str) -> IResult<&str, Component> {
    alt((cwd, cwd_with_style))(input)
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
    use crate::component;

    #[test]
    fn it_works() {
        assert_eq!(
            parse(&"{cwd}").unwrap().1,
            vec![Component::Cwd {
                style: component::CwdStyle::Default
            }]
        );
        assert_eq!(
            parse(&"{cwd} $").unwrap().1,
            vec![
                Component::Cwd {
                    style: component::CwdStyle::Default
                },
                Component::Char(' '),
                Component::Char('$')
            ]
        );
        assert_eq!(
            parse(&"{cwd style=default}").unwrap().1,
            vec![Component::Cwd {
                style: component::CwdStyle::Default,
            }]
        );
        assert_eq!(
            parse(&"{cwd style=long}").unwrap().1,
            vec![Component::Cwd {
                style: component::CwdStyle::Long
            }]
        );
    }
}
