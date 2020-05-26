use crate::token::{Component, Condition, Token};
use anyhow::Result;
use crossterm::style::Color;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0, none_of, one_of};
use nom::combinator::{all_consuming, map, map_res, opt, recognize, verify};
use nom::error::{convert_error, VerboseError};
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::convert::TryFrom;

type IResult<I, O> = nom::IResult<I, O, VerboseError<I>>;

static RESERVED_KEYWORDS: Lazy<HashSet<&str>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert("end");
    s.insert("else");
    s
});

fn static_component(input: &str) -> IResult<&str, Token> {
    map(
        alt((any_char_except_opening_brace, escaped_opening_brace)),
        |s| Token::Static(s.to_owned()),
    )(input)
}

fn any_char_except_opening_brace(input: &str) -> IResult<&str, &str> {
    recognize(many1(none_of("{")))(input)
}

fn escaped_opening_brace(input: &str) -> IResult<&str, &str> {
    tag("{{")(input)
}

fn start_tag(input: &str) -> IResult<&str, &str> {
    terminated(tag("{"), multispace0)(input)
}

fn end_tag(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, tag("}"))(input)
}

fn start_identifier_end(input: &str) -> IResult<&str, &str> {
    terminated(preceded(start_tag, identifier), end_tag)(input)
}

fn parse_color(input: &str) -> Result<Color, ()> {
    let color = input
        .parse::<Color>()
        .expect("Color::from_str() does not return error");

    match color {
        // For unknown values from_str() returns Color::White
        c @ Color::White => {
            if input.to_lowercase() == "white" {
                Ok(c)
            } else {
                Err(())
            }
        }
        c => Ok(c),
    }
}

fn color(input: &str) -> IResult<&str, Token> {
    map(map_res(start_identifier_end, parse_color), Token::Color)(input)
}

fn reset(input: &str) -> IResult<&str, Token> {
    let reset = terminated(preceded(start_tag, tag("reset")), end_tag);
    map(reset, |_| Token::Reset)(input)
}

fn if_start(input: &str) -> IResult<&str, ()> {
    map(pair(start_tag, tag("if")), drop)(input)
}

fn environment_variable_name(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("$"),
        recognize(many1(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ_"))),
    )(input)
}

fn if_condition(input: &str) -> IResult<&str, Condition> {
    let basic_condition = map_res(identifier, Condition::try_from);
    let environment_variable = map(environment_variable_name, |var_name: &str| {
        Condition::EnvironmentVariable(var_name.to_owned())
    });
    let condition = alt((basic_condition, environment_variable));

    terminated(preceded(multispace0, condition), end_tag)(input)
}

fn end(input: &str) -> IResult<&str, ()> {
    map(tag("{end}"), drop)(input)
}

fn if_else_tag(input: &str) -> IResult<&str, ()> {
    map(tag("{else}"), drop)(input)
}

fn if_else_branch(input: &str) -> IResult<&str, Vec<Token>> {
    preceded(if_else_tag, tokens)(input)
}

fn conditional(input: &str) -> IResult<&str, Token> {
    map(
        tuple((
            // {if condition}
            preceded(if_start, if_condition),
            // foo bar baz
            tokens,
            terminated(
                // {else}
                // foo bar baz
                opt(if_else_branch),
                // {end}
                end,
            ),
        )),
        |(condition, left, right)| Token::Conditional {
            condition,
            left,
            right,
        },
    )(input)
}

fn key(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, alpha_underscore)(input)
}

fn value(input: &str) -> IResult<&str, &str> {
    recognize(many1(none_of("} ")))(input)
}

fn key_value(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(key, tag("="), value)(input)
}

fn key_values(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(key_value)(input)
}

fn underscore(input: &str) -> IResult<&str, &str> {
    tag("_")(input)
}

fn alpha_underscore(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alpha1, underscore))))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    verify(alpha_underscore, |s: &str| !RESERVED_KEYWORDS.contains(s))(input)
}

fn component(input: &str) -> IResult<&str, Token> {
    map(
        tuple((
            preceded(start_tag, map_res(identifier, Component::try_from)),
            terminated(key_values, end_tag),
        )),
        |(name, options)| {
            let options = options
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect();
            Token::Component { name, options }
        },
    )(input)
}

fn tokens(input: &str) -> IResult<&str, Vec<Token>> {
    many1(alt((
        static_component,
        color,
        reset,
        conditional,
        component,
    )))(input)
}

pub fn parse(input: &str) -> Result<Vec<Token>> {
    match all_consuming(tokens)(input) {
        Ok((_input, tokens)) => Ok(tokens),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let message = convert_error(input, e);
            Err(anyhow::anyhow!("parse error:\n\n{}", message.trim_end()))
        }
        Err(e) => Err(anyhow::anyhow!("parse error: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_parses_a_component() {
        assert_eq!(
            parse(&"{cwd}").unwrap(),
            vec![Token::Component {
                name: Component::Cwd,
                options: HashMap::new(),
            }]
        );
    }

    #[test]
    fn it_parses_a_component_and_static() {
        assert_eq!(
            parse(&"{cwd} $").unwrap(),
            vec![
                Token::Component {
                    name: Component::Cwd,
                    options: HashMap::new(),
                },
                Token::Static(" $".to_string())
            ]
        );
    }

    #[test]
    fn it_parses_a_component_with_options() {
        let mut options = HashMap::new();
        options.insert("style".to_string(), "default".to_string());
        assert_eq!(
            parse(&"{cwd style=default}").unwrap(),
            vec![Token::Component {
                name: Component::Cwd,
                options,
            },]
        );

        let mut options = HashMap::new();
        options.insert("style".to_string(), "short".to_string());
        assert_eq!(
            parse(&"{cwd style=short}").unwrap(),
            vec![Token::Component {
                name: Component::Cwd,
                options,
            },]
        );

        let mut options = HashMap::new();
        options.insert("style".to_string(), "long".to_string());
        assert_eq!(
            parse(&"{cwd style=long}").unwrap(),
            vec![Token::Component {
                name: Component::Cwd,
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

        assert!(identifier(&"end").is_err());
    }

    #[test]
    fn it_parses_static() {
        assert_eq!(
            parse(&"cwd").unwrap(),
            vec![Token::Static("cwd".to_string()),]
        );
    }

    #[test]
    fn it_allows_escaped_braces_as_static() {
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
                    name: Component::Cwd,
                    options: HashMap::new(),
                }
            ]
        );
    }

    #[test]
    fn it_parses_components_and_options_with_spaces() {
        let mut options = HashMap::new();
        options.insert("a".to_string(), "bc".to_string());
        options.insert("d".to_string(), "12".to_string());

        assert_eq!(
            parse(&"{  git_branch a=bc   d=12  }  { git_commit }").unwrap(),
            vec![
                Token::Component {
                    name: Component::GitBranch,
                    options,
                },
                Token::Static("  ".to_string()),
                Token::Component {
                    name: Component::GitCommit,
                    options: HashMap::new(),
                },
            ]
        );
    }

    #[test]
    fn it_parses_options() {
        let mut options = HashMap::new();
        options.insert("a".to_string(), "bc".to_string());
        options.insert("d".to_string(), "12".to_string());

        assert_eq!(
            parse(&"{git_branch a=bc d=12}").unwrap(),
            vec![Token::Component {
                name: Component::GitBranch,
                options,
            }]
        );
    }

    #[test]
    fn it_parses_conditionals() {
        assert_eq!(
            parse(&"{if last_command_status}left{end}").unwrap(),
            vec![Token::Conditional {
                condition: Condition::LastCommandStatus,
                left: vec![Token::Static("left".to_string())],
                right: None,
            }]
        );
    }

    #[test]
    fn it_parses_conditionals_with_else_branch() {
        assert_eq!(
            parse(&"{if last_command_status}left{else}right{end}").unwrap(),
            vec![Token::Conditional {
                condition: Condition::LastCommandStatus,
                left: vec![Token::Static("left".to_string())],
                right: Some(vec![Token::Static("right".to_string())]),
            }]
        );
    }

    #[test]
    fn it_ensures_all_input_is_consumed() {
        assert!(parse(&"foo{git_branch bar=").is_err());
    }

    #[test]
    fn it_parses_style_components() {
        assert_eq!(parse(&"{green}").unwrap(), vec![Token::Color(Color::Green)]);
    }

    #[test]
    fn it_parses_style_with_whitespace() {
        assert_eq!(
            parse(&"{  green  }").unwrap(),
            vec![Token::Color(Color::Green)]
        );
    }

    #[test]
    fn it_parses_environment_variable_names() {
        let subject = environment_variable_name;

        assert_eq!(subject(&"$TEST").unwrap(), ("", "TEST"));
        assert_eq!(subject(&"$FOO_BAR").unwrap(), ("", "FOO_BAR"));
        assert_eq!(subject(&"$FOO BAR").unwrap(), (" BAR", "FOO"));
    }

    #[test]
    fn it_parses_colors() {
        let a = assert_result_return_tokens;

        assert_eq!(a(color(&"{grey}")), Token::Color(Color::Grey));
        assert_eq!(a(color(&"{dArk_grEy}")), Token::Color(Color::DarkGrey));
        assert_eq!(a(color(&"{White}")), Token::Color(Color::White));
    }

    fn assert_result_return_tokens<O: std::fmt::Debug, E: std::fmt::Debug>(
        result: Result<(&str, O), E>,
    ) -> O {
        assert!(result.is_ok());
        let (input, output) = result.unwrap();
        assert_eq!(input, "");
        output
    }
}
