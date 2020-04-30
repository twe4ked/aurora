use crate::component::cwd::CwdStyle;
use crate::token::{StyleToken, Token};
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::multi::many0;
use nom::IResult;

fn cwd(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("cwd")(input)?;
    let style = CwdStyle::Default;
    Ok((input, Token::Cwd(style)))
}

fn cwd_with_style(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("cwd style=")(input)?;
    let (input, output) = alt((tag("default"), tag("short"), tag("long")))(input)?;
    let style = match output {
        "default" => CwdStyle::Default,
        "short" => CwdStyle::Short,
        "long" => CwdStyle::Long,
        _ => panic!("invalid style"),
    };

    Ok((input, Token::Cwd(style)))
}

fn any_char_except_opening_brace(input: &str) -> IResult<&str, Token> {
    let (input, output) = none_of("{")(input)?;
    Ok((input, Token::Char(output)))
}

fn style(input: &str) -> IResult<&str, Token> {
    use StyleToken::*;

    let (input, output) = alt((
        tag("black"),
        tag("dark_grey"),
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

    Ok((input, Token::Style(style)))
}

fn git_branch(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("git_branch")(input)?;
    Ok((input, Token::GitBranch))
}

fn git_commit(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("git_commit")(input)?;
    Ok((input, Token::GitCommit))
}

fn git_stash(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("git_stash")(input)?;
    Ok((input, Token::GitStash))
}

fn jobs(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("jobs")(input)?;
    Ok((input, Token::Jobs))
}

fn component(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("{")(input)?;

    let (input, component) = alt((
        cwd_with_style,
        cwd,
        style,
        git_branch,
        git_commit,
        git_stash,
        jobs,
    ))(input)?;

    let (input, _) = tag("}")(input)?;

    Ok((input, component))
}

pub fn parse(input: &str) -> Result<Vec<Token>> {
    many0(alt((any_char_except_opening_brace, component)))(input)
        .map(|(_, tokens)| Ok(tokens))
        .unwrap_or(Err(anyhow::anyhow!("parse error")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            parse(&"{cwd}").unwrap(),
            vec![Token::Cwd(CwdStyle::Default)]
        );
        assert_eq!(
            parse(&"{cwd} $").unwrap(),
            vec![
                Token::Cwd(CwdStyle::Default),
                Token::Char(' '),
                Token::Char('$')
            ]
        );
        assert_eq!(
            parse(&"{cwd style=default}").unwrap(),
            vec![Token::Cwd(CwdStyle::Default)]
        );
        assert_eq!(
            parse(&"{cwd style=short}").unwrap(),
            vec![Token::Cwd(CwdStyle::Short)]
        );
        assert_eq!(
            parse(&"{cwd style=long}").unwrap(),
            vec![Token::Cwd(CwdStyle::Long)]
        );
    }
}
