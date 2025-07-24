use aoc_timing::trace::log_run;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u8},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use std::io;

#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl Game {
    fn parser(input: &str) -> IResult<&str, Self> {
        let id = preceded(tag("Game "), u8);
        let sets = separated_list1(tag("; "), Set::parser);

        (terminated(id, tag(": ")), sets)
            .map(|(id, sets)| Game { id, sets })
            .parse(input)
    }
}

impl Set {
    fn parser(input: &str) -> IResult<&str, Self> {
        separated_list1(
            tag(", "),
            (
                terminated(u8, char(' ')),
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
        )
        .map(|cubes_list| {
            let mut set = Set::default();

            for (count, color) in cubes_list {
                match color {
                    "red" => set.red = count,
                    "green" => set.green = count,
                    "blue" => set.blue = count,
                    _ => panic!("Unexpected color matched"),
                }
            }

            set
        })
        .parse(input)
    }
}

fn parse<S: AsRef<str>, T: Iterator<Item = S>>(input: T) -> Vec<Game> {
    input
        .map(|line| Game::parser(line.as_ref()).expect("Failed to parse game").1)
        .collect()
}

fn part_1(input: &[Game]) -> u16 {
    input
        .iter()
        .filter(|game| {
            !game
                .sets
                .iter()
                .any(|set| set.red > 12 || set.green > 13 || set.blue > 14)
        })
        .map(|game| u16::from(game.id))
        .sum()
}

fn part_2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            game.sets.iter().fold(Set::default(), |result, set| Set {
                red: result.red.max(set.red),
                green: result.green.max(set.green),
                blue: result.blue.max(set.blue),
            })
        })
        .map(|minimum| u32::from(minimum.red) * u32::from(minimum.green) * u32::from(minimum.blue))
        .sum()
}

fn main() {
    env_logger::init();
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(&parse(INPUT.lines()));

        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(&parse(INPUT.lines()));

        assert_eq!(result, 2286);
    }
}
