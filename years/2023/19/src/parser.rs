use crate::*;

use chumsky::error::Cheap;
use chumsky::prelude::*;
use std::str::FromStr;

pub(crate) trait CharParser<T>: Parser<char, T, Error = Cheap<char>> {}
impl<T, P: Parser<char, T, Error = Cheap<char>>> CharParser<T> for P {}

pub(crate) fn named_workflow_parser() -> impl CharParser<NamedWorkflow> {
    (identifier().then(
        conditional_destination_parser()
            .separated_by(just(','))
            .delimited_by(just('{'), just('}')),
    ))
    .map(|(name, conditions)| NamedWorkflow {
        name,
        workflow: Workflow { conditions },
    })
}

pub(crate) fn part_parser() -> impl CharParser<Part> {
    just("x=")
        .ignore_then(number())
        .then(just(",m=").ignore_then(number()))
        .then(just(",a=").ignore_then(number()))
        .then(just(",s=").ignore_then(number()))
        .delimited_by(just("{"), just("}"))
        .map(|(((x_rating, m_rating), a_rating), s_rating)| Part {
            x_rating,
            m_rating,
            a_rating,
            s_rating,
        })
}

fn conditional_destination_parser() -> impl CharParser<ConditionalDestination> {
    let less_than = category_parser()
        .then_ignore(just('<'))
        .then(number())
        .map(|(category, n)| Condition::LessThan(category, n));
    let greater_than = category_parser()
        .then_ignore(just('>'))
        .then(number())
        .map(|(category, n)| Condition::GreaterThan(category, n));

    choice((less_than, greater_than))
        .then_ignore(just(':'))
        .or_not()
        .then(destination_parser())
        .map(|(condition, destination)| ConditionalDestination {
            condition: condition.unwrap_or(Condition::Unconditional),
            destination,
        })
}

fn destination_parser() -> impl CharParser<Destination> {
    choice((
        just('A').to(Destination::Accept),
        just('R').to(Destination::Reject),
        identifier().map(Destination::NextWorkflow),
    ))
}

fn category_parser() -> impl CharParser<Category> {
    choice([
        just('x').to(Category::X),
        just('m').to(Category::M),
        just('a').to(Category::A),
        just('s').to(Category::S),
    ])
}

fn number<Err: Debug, T: FromStr<Err = Err>>() -> impl CharParser<T> {
    filter(char::is_ascii_digit)
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|n| T::from_str(&n).unwrap())
}

fn identifier() -> impl CharParser<String> {
    filter(char::is_ascii_alphanumeric)
        .repeated()
        .at_least(1)
        .collect()
}
