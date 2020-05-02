use crate::token::{StyleToken, Token};
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{none_of, one_of, space0};
use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::collections::HashMap;
use std::iter::FromIterator;

fn any_char_except_opening_brace(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, output) = none_of("{")(input)?;
    Ok((input, vec![Token::Char(output)]))
}

fn escaped_opening_brace(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = tag("{{")(input)?;
    Ok((input, vec![Token::Char('{'), Token::Char('{')]))
}

fn color(input: &str) -> IResult<&str, &str> {
    alt((
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
    ))(input)
}

fn style(input: &str) -> IResult<&str, Vec<Token>> {
    use StyleToken::*;

    let (input, output) = terminated(preceded(tag("{"), color), tag("}"))(input)?;

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

    Ok((input, vec![Token::Style(style)]))
}

fn key_value(input: &str) -> IResult<&str, HashMap<String, String>> {
    let (input, _) = space0(input)?;
    let (input, key) = many0(none_of("}="))(input)?;
    let (input, _) = opt(tag("="))(input)?;
    let (input, value) = many0(none_of("}"))(input)?;

    let mut map = HashMap::new();
    if !key.is_empty() && !value.is_empty() {
        map.insert(String::from_iter(key), String::from_iter(value));
    }
    Ok((input, map))
}

fn identifier(input: &str) -> IResult<&str, String> {
    let (input, name) = many1(one_of("abcdefghijkllmnopqrstuvwxyz_"))(input)?;
    Ok((input, String::from_iter(name)))
}

fn component(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = tag("{")(input)?;
    let (input, name) = identifier(input)?;
    let (input, options) = key_value(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, vec![Token::Component { name, options }]))
}

pub fn parse(input: &str) -> Result<Vec<Token>> {
    many0(alt((
        any_char_except_opening_brace,
        escaped_opening_brace,
        style,
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
        assert_eq!(
            parse(&"{cwd}").unwrap(),
            vec![Token::Component {
                name: "cwd".to_string(),
                options: HashMap::new(),
            }]
        );
        assert_eq!(
            parse(&"{cwd} $").unwrap(),
            vec![
                Token::Component {
                    name: "cwd".to_string(),
                    options: HashMap::new(),
                },
                Token::Char(' '),
                Token::Char('$')
            ]
        );

        let mut options = HashMap::new();
        options.insert("style".to_string(), "default".to_string());
        assert_eq!(
            parse(&"{cwd style=default}").unwrap(),
            vec![Token::Component {
                name: "cwd".to_string(),
                options,
            },]
        );

        let mut options = HashMap::new();
        options.insert("style".to_string(), "short".to_string());
        assert_eq!(
            parse(&"{cwd style=short}").unwrap(),
            vec![Token::Component {
                name: "cwd".to_string(),
                options,
            },]
        );

        let mut options = HashMap::new();
        options.insert("style".to_string(), "long".to_string());
        assert_eq!(
            parse(&"{cwd style=long}").unwrap(),
            vec![Token::Component {
                name: "cwd".to_string(),
                options,
            },]
        );
    }

    #[test]
    fn it_parses_identifiers() {
        assert_eq!(identifier(&"cwd").unwrap().1, "cwd".to_string());
        assert_eq!(
            identifier(&"git_branch").unwrap().1,
            "git_branch".to_string()
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
                Token::Component {
                    name: "cwd".to_string(),
                    options: HashMap::new(),
                }
            ]
        );
    }
}
