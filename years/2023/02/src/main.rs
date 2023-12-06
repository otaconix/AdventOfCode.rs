use std::{io, str::FromStr};

use pom::utf8::*;

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

fn number_parser<'a>() -> Parser<'a, u8> {
    is_a(|c| c.is_ascii_digit())
        .repeat(1..)
        .collect()
        .convert(|digits| digits.parse())
}

impl Game {
    fn parser<'a>() -> Parser<'a, Self> {
        let id = seq("Game ") * number_parser();
        let sets = list(call(Set::parser), seq("; "));

        ((id - seq(": ")) + sets).map(|(id, sets)| Game { id, sets })
    }
}

impl FromStr for Game {
    type Err = pom::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Game::parser().parse_str(s)
    }
}

impl Set {
    fn parser<'a>() -> Parser<'a, Self> {
        let cubes = (number_parser() - seq(" ")) + (seq("red") | seq("green") | seq("blue"));
        list(cubes, seq(", ")).map(|cubes_list| {
            let mut set = Set::default();

            for (count, color) in cubes_list {
                match color {
                    "red" => set.red = count,
                    "green" => set.green = count,
                    "blue" => set.blue = count,
                    _ => panic!("Unexpected color matched"),
                };
            }

            set
        })
    }
}

fn parse<S: ToString, T: Iterator<Item = S>>(input: T) -> Vec<Game> {
    input
        .map(|line| {
            line.to_string()
                .parse::<Game>()
                .expect("Failed to parse game")
        })
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
        .map(|game| game.id as u16)
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
        .map(|minimum| minimum.red as u32 * minimum.green as u32 * minimum.blue as u32)
        .sum()
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = part_1(&input);

    println!("Part 1: {part_1}");

    let part_2 = part_2(&input);

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
