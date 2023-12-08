use std::{collections::HashMap, io};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Destinations {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    map: HashMap<String, Destinations>,
}

fn parse<S: ToString, I: Iterator<Item = S>>(mut input: I) -> Input {
    let directions = input
        .next()
        .unwrap()
        .to_string()
        .chars()
        .map(|c| c.try_into().expect("Invalid direction"))
        .collect();

    let map = input
        .skip(1)
        .map(|line| {
            let words = line
                .to_string()
                .split_ascii_whitespace()
                .map(|word| {
                    word.chars()
                        .filter(char::is_ascii_uppercase)
                        .collect::<String>()
                })
                .collect::<Vec<_>>();

            (
                words[0].clone(),
                Destinations {
                    left: words[2].clone(),
                    right: words[3].clone(),
                },
            )
        })
        .collect();

    Input { directions, map }
}

const START: &str = "AAA";
const END: &str = "ZZZ";

fn part_1(input: &Input) -> usize {
    let mut count = 0;
    let mut current = START;

    for direction in input.directions.iter().cycle() {
        if current == END {
            break;
        }

        count += 1;
        current = match direction {
            Direction::Left => &input.map[current].left,
            Direction::Right => &input.map[current].right,
        }
    }

    count
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = part_1(&input);

    println!("Part 1: {part_1}");
}
