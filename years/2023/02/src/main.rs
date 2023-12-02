use std::{io, str::FromStr};

use pom::utf8::*;

#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

#[derive(Debug)]
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
            let mut set = Set {
                red: 0,
                green: 0,
                blue: 0,
            };

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

fn main() {
    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse::<Game>().expect("Failed to parse game"))
        .collect::<Vec<_>>();

    let part_1 = input
        .iter()
        .filter(|game| {
            !game
                .sets
                .iter()
                .any(|set| set.red > 12 || set.green > 13 || set.blue > 14)
        })
        .map(|game| game.id as u16)
        .sum::<u16>();

    println!("Part 1: {part_1}");
}
