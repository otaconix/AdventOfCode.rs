use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u32},
    combinator::{opt, value},
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

use super::{Blueprint, Resource, Resources};

fn resource_parser(input: &str) -> IResult<&str, Resource> {
    alt((
        value(Resource::Ore, tag("ore")),
        value(Resource::Clay, tag("clay")),
        value(Resource::Obsidian, tag("obsidian")),
        value(Resource::Geode, tag("geode")),
    ))(input)
}

fn resources_parser(input: &str) -> IResult<&str, Resources> {
    let single_cost = || terminated(u32, char(' ')).and(resource_parser);

    tuple((single_cost(), many0(preceded(tag(" and "), single_cost()))))
        .map(|(first, rest)| {
            let mut result = Resources::default();

            for item in [first].iter().chain(rest.iter()) {
                result[&item.1] = item.0;
            }

            result
        })
        .parse(input)
}

pub(crate) fn blueprint_parser(input: &str) -> IResult<&str, Blueprint> {
    let robot_cost_parser = preceded(
        tag("Each "),
        terminated(
            tuple((
                resource_parser,
                preceded(tag(" robot costs "), resources_parser),
            )),
            char('.').and(opt(char(' '))),
        ),
    );

    tuple((
        terminated(preceded(tag("Blueprint "), u32), tag(": ")),
        many1(robot_cost_parser),
    ))
    .map(|x| Blueprint::new(x.0, x.1.into_iter().collect()))
    .parse(input)
}
