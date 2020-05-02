use crate::token::{StyleToken, Token};
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{none_of, one_of, space0};
use nom::combinator::{map_res, opt};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::collections::HashMap;
use std::iter::FromIterator;

fn any_char_except_opening_brace(input: &str) -> IResult<&str, Token> {
    let (input, output) = many1(none_of("{"))(input)?;
    Ok((input, Token::Static(String::from_iter(output))))
}

fn escaped_opening_brace(input: &str) -> IResult<&str, Token> {
    let (input, output) = tag("{{")(input)?;
    Ok((input, Token::Static(output.to_string())))
}

fn style(input: &str) -> IResult<&str, Token> {
    let (input, output) = map_res(
        terminated(preceded(tag("{"), identifier), tag("}")),
        |s: String| s.parse::<StyleToken>(),
    )(input)?;
    Ok((input, Token::Style(output)))
}

fn key_value(input: &str) -> IResult<&str, (String, String)> {
    let (input, _) = space0(input)?;
    let (input, key) = many1(none_of("}="))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = many1(none_of("} "))(input)?;
    Ok((input, (String::from_iter(key), String::from_iter(value))))
}

fn identifier(input: &str) -> IResult<&str, String> {
    let (input, name) = many1(one_of("abcdefghijkllmnopqrstuvwxyz_"))(input)?;
    Ok((input, String::from_iter(name)))
}

fn component(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("{")(input)?;
    let (input, name) = identifier(input)?;
    let (input, options) = many0(key_value)(input)?;
    let options = options.into_iter().collect();
    let (input, _) = tag("}")(input)?;
    Ok((input, Token::Component { name, options }))
}

pub fn parse(input: &str) -> Result<Vec<Token>> {
    many0(alt((
        any_char_except_opening_brace,
        escaped_opening_brace,
        style,
        component,
    )))(input)
    .map(|(_, tokens)| Ok(tokens))
    .unwrap_or_else(|_| Err(anyhow::anyhow!("parse error")))
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
                Token::Static(" $".to_string())
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
                Token::Static("{{".to_string()),
                Token::Static("cwd".to_string()),
            ]
        );

        assert_eq!(
            parse(&"{{cwd{cwd}").unwrap(),
            vec![
                Token::Static("{{".to_string()),
                Token::Static("cwd".to_string()),
                Token::Component {
                    name: "cwd".to_string(),
                    options: HashMap::new(),
                }
            ]
        );
    }

    #[test]
    fn it_parses_options() {
        let mut options = HashMap::new();
        options.insert("a".to_string(), "bc".to_string());
        options.insert("d".to_string(), "12".to_string());

        assert_eq!(
            parse(&"{foo a=bc d=12}").unwrap(),
            vec![Token::Component {
                name: "foo".to_string(),
                options,
            }]
        );
    }
}
