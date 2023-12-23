use crate::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, u64},
    combinator::{opt, value},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

type ParseResult<'a, T> = IResult<&'a str, T>;

pub(crate) fn named_workflow_parser(input: &str) -> ParseResult<NamedWorkflow> {
    tuple((
        alphanumeric1,
        delimited(
            char('{'),
            separated_list0(char(','), conditional_destination_parser),
            char('}'),
        ),
    ))
    .map(|(name, conditions)| NamedWorkflow {
        name: name.to_string(),
        workflow: Workflow { conditions },
    })
    .parse(input)
}

pub(crate) fn part_parser(input: &str) -> ParseResult<Part> {
    delimited(
        char('{'),
        tuple((
            preceded(tag("x="), usize),
            preceded(tag(",m="), usize),
            preceded(tag(",a="), usize),
            preceded(tag(",s="), usize),
        )),
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

fn conditional_destination_parser(input: &str) -> ParseResult<ConditionalDestination> {
    let less_than = tuple((terminated(category_parser, char('<')), usize))
        .map(|(category, n)| Condition::LessThan(category, n));
    let greater_than = tuple((terminated(category_parser, char('>')), usize))
        .map(|(category, n)| Condition::GreaterThan(category, n));

    tuple((
        opt(terminated(alt((less_than, greater_than)), char(':'))),
        destination_parser,
    ))
    .map(|(condition, destination)| ConditionalDestination {
        condition: condition.unwrap_or(Condition::Unconditional),
        destination,
    })
    .parse(input)
}

fn destination_parser(input: &str) -> ParseResult<Destination> {
    alt((
        value(Destination::Accept, char('A')),
        value(Destination::Reject, char('R')),
        alphanumeric1
            .map(str::to_string)
            .map(Destination::NextWorkflow),
    ))
    .parse(input)
}

fn category_parser(input: &str) -> ParseResult<Category> {
    alt((
        value(Category::X, char('x')),
        value(Category::M, char('m')),
        value(Category::A, char('a')),
        value(Category::S, char('s')),
    ))
    .parse(input)
}

fn usize(input: &str) -> ParseResult<usize> {
    u64.map(|n| n as usize).parse(input)
}
