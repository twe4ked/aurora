use crate::token::{StyleToken, Token};
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{none_of, space0};
use nom::combinator::opt;
use nom::multi::many0;
use nom::IResult;

fn cwd(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("cwd")(input)?;
    Ok((input, Token::Cwd))
}

fn any_char_except_opening_brace(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, output) = none_of("{")(input)?;
    Ok((input, vec![Token::Char(output)]))
}

fn escaped_opening_brace(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = tag("{{")(input)?;
    Ok((input, vec![Token::Char('{'), Token::Char('{')]))
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

fn key_values(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = space0(input)?;
    let (input, key) = many0(none_of("}="))(input)?;
    let (input, _) = opt(tag("="))(input)?;
    let (input, value) = many0(none_of("}"))(input)?;

    use std::iter::FromIterator;

    if !key.is_empty() && !value.is_empty() {
        Ok((
            input,
            vec![Token::KeyValue(
                String::from_iter(key),
                String::from_iter(value),
            )],
        ))
    } else {
        Ok((input, Vec::new()))
    }
}

fn component(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = tag("{")(input)?;
    let (input, component) = alt((cwd, style, git_branch, git_commit, git_stash, jobs))(input)?;
    let (input, mut key_values) = key_values(input)?;
    let (input, _) = tag("}")(input)?;

    let mut components = vec![component];
    components.append(&mut key_values);
    Ok((input, components))
}

pub fn parse(input: &str) -> Result<Vec<Token>> {
    many0(alt((
        any_char_except_opening_brace,
        escaped_opening_brace,
        component,
    )))(input)
    .map(|(_, tokens)| Ok(tokens.into_iter().flatten().collect()))
    .unwrap_or(Err(anyhow::anyhow!("parse error")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(parse(&"{cwd}").unwrap(), vec![Token::Cwd]);
        assert_eq!(
            parse(&"{cwd} $").unwrap(),
            vec![Token::Cwd, Token::Char(' '), Token::Char('$')]
        );
        assert_eq!(
            parse(&"{cwd style=default}").unwrap(),
            vec![
                Token::Cwd,
                Token::KeyValue("style".to_string(), "default".to_string())
            ]
        );
        assert_eq!(
            parse(&"{cwd style=short}").unwrap(),
            vec![
                Token::Cwd,
                Token::KeyValue("style".to_string(), "short".to_string())
            ]
        );
        assert_eq!(
            parse(&"{cwd style=long}").unwrap(),
            vec![
                Token::Cwd,
                Token::KeyValue("style".to_string(), "long".to_string())
            ]
        );
    }

    #[test]
    fn it_allows_escaped_braces_as_char() {
        assert_eq!(
            parse(&"{{cwd").unwrap(),
            vec![
                Token::Char('{'),
                Token::Char('{'),
                Token::Char('c'),
                Token::Char('w'),
                Token::Char('d'),
            ]
        );

        assert_eq!(
            parse(&"{{cwd{cwd}").unwrap(),
            vec![
                Token::Char('{'),
                Token::Char('{'),
                Token::Char('c'),
                Token::Char('w'),
                Token::Char('d'),
                Token::Cwd,
            ]
        );
    }
}
