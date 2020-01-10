use crate::component::cwd;
use crate::static_component::{Component, Style};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::many0;
use nom::IResult;

fn cwd(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{cwd}")(input)?;
    let style = cwd::CwdStyle::Default;
    Ok((input, Component::Cwd { style: style }))
}

fn cwd_with_style(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{cwd style=")(input)?;
    let (input, output) = alt((tag("default"), tag("short"), tag("long")))(input)?;
    let style = match output {
        "default" => cwd::CwdStyle::Default,
        "short" => cwd::CwdStyle::Short,
        "long" => cwd::CwdStyle::Long,
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

fn style(input: &str) -> IResult<&str, Component> {
    use Style::*;

    let (input, _) = tag("{")(input)?;
    let (input, output) = alt((
        tag("black"),
        tag("dark_grey"),
        tag("grey_blue"),
        tag("blue"),
        tag("dark_blue"),
        tag("green"),
        tag("dark_green"),
        tag("red"),
        tag("dark_red"),
        tag("cyan"),
        tag("dark_cyan"),
        tag("magenta"),
        tag("dark_magenta"),
        tag("yellow"),
        tag("dark_yellow"),
        tag("white"),
        tag("reset"),
    ))(input)?;
    let (input, _) = tag("}")(input)?;

    let style = match output {
        "black" => Black,
        "dark_grey" => DarkGrey,
        "blue" => Blue,
        "dark_blue" => DarkBlue,
        "green" => Green,
        "dark_green" => DarkGreen,
        "red" => Red,
        "dark_red" => DarkRed,
        "cyan" => Cyan,
        "dark_cyan" => DarkCyan,
        "magenta" => Magenta,
        "dark_magenta" => DarkMagenta,
        "yellow" => Yellow,
        "dark_yellow" => DarkYellow,
        "white" => White,
        "reset" => Reset,
        _ => unreachable!(),
    };

    Ok((input, Component::Style(style)))
}

fn git_branch(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{git_branch}")(input)?;
    Ok((input, Component::GitBranch))
}

fn git_commit(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{git_commit}")(input)?;
    Ok((input, Component::GitCommit))
}

fn git_stash(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("{git_stash}")(input)?;
    Ok((input, Component::GitStash))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Component>> {
    many0(alt((
        expression, style, git_branch, git_commit, git_stash, any_char,
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::cwd;

    #[test]
    fn it_works() {
        assert_eq!(
            parse(&"{cwd}").unwrap().1,
            vec![Component::Cwd {
                style: cwd::CwdStyle::Default
            }]
        );
        assert_eq!(
            parse(&"{cwd} $").unwrap().1,
            vec![
                Component::Cwd {
                    style: cwd::CwdStyle::Default
                },
                Component::Char(' '),
                Component::Char('$')
            ]
        );
        assert_eq!(
            parse(&"{cwd style=default}").unwrap().1,
            vec![Component::Cwd {
                style: cwd::CwdStyle::Default,
            }]
        );
        assert_eq!(
            parse(&"{cwd style=short}").unwrap().1,
            vec![Component::Cwd {
                style: cwd::CwdStyle::Short
            }]
        );
        assert_eq!(
            parse(&"{cwd style=long}").unwrap().1,
            vec![Component::Cwd {
                style: cwd::CwdStyle::Long
            }]
        );
    }
}
