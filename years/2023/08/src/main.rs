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
                    left: words[1].clone(),
                    right: words[2].clone(),
                },
            )
        })
        .collect();

    Input { directions, map }
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    println!("Input: {input:#?}");
}
