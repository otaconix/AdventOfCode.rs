use crate::*;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, u64},
    combinator::{opt, value},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated},
};

type ParseResult<'a, T> = IResult<&'a str, T>;

pub(crate) fn named_workflow_parser(input: &str) -> ParseResult<'_, NamedWorkflow> {
    (
        alphanumeric1,
        delimited(
            char('{'),
            separated_list0(char(','), conditional_destination_parser),
            char('}'),
        ),
    )
        .map(|(name, conditions)| NamedWorkflow {
            name: name.to_string(),
            workflow: Workflow { conditions },
        })
        .parse(input)
}

pub(crate) fn part_parser(input: &str) -> ParseResult<'_, Part> {
    delimited(
        char('{'),
        (
            preceded(tag("x="), usize),
            preceded(tag(",m="), usize),
            preceded(tag(",a="), usize),
            preceded(tag(",s="), usize),
        ),
        char('}'),
    )
    .map(|(x_rating, m_rating, a_rating, s_rating)| Part {
        x_rating,
        m_rating,
        a_rating,
        s_rating,
    })
    .parse(input)
}

fn conditional_destination_parser(input: &str) -> ParseResult<'_, ConditionalDestination> {
    let less_than = (terminated(category_parser, char('<')), usize)
        .map(|(category, n)| Condition::LessThan(category, n));
    let greater_than = (terminated(category_parser, char('>')), usize)
        .map(|(category, n)| Condition::GreaterThan(category, n));

    (
        opt(terminated(alt((less_than, greater_than)), char(':'))),
        destination_parser,
    )
        .map(|(condition, destination)| ConditionalDestination {
            condition: condition.unwrap_or(Condition::Unconditional),
            destination,
        })
        .parse(input)
}

fn destination_parser(input: &str) -> ParseResult<'_, Destination> {
    alt((
        value(Destination::Accept, char('A')),
        value(Destination::Reject, char('R')),
        alphanumeric1
            .map(str::to_string)
            .map(Destination::NextWorkflow),
    ))
    .parse(input)
}

fn category_parser(input: &str) -> ParseResult<'_, Category> {
    alt((
        value(Category::X, char('x')),
        value(Category::M, char('m')),
        value(Category::A, char('a')),
        value(Category::S, char('s')),
    ))
    .parse(input)
}

fn usize(input: &str) -> ParseResult<'_, usize> {
    u64.map(|n| n as usize).parse(input)
}
