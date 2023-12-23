use std::collections::HashMap;
use std::io;

use aoc_timing::trace::log_run;
use num_integer::lcm;

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

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let directions = input
        .next()
        .unwrap()
        .as_ref()
        .chars()
        .map(|c| c.try_into().expect("Invalid direction"))
        .collect();

    let map = input
        .skip(1)
        .map(|line| {
            let words = line
                .as_ref()
                .split_ascii_whitespace()
                .map(|word| {
                    word.chars()
                        .filter(char::is_ascii_alphanumeric)
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

fn steps_count<F>(input: &Input, start: &str, dest_predicate: F) -> usize
where
    F: Fn(&str) -> bool,
{
    let mut current = start;
    let mut count = 0;

    for direction in input.directions.iter().cycle() {
        if dest_predicate(current) {
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

fn part_1(input: &Input) -> usize {
    steps_count(input, START, |current| current == END)
}

fn part_2(input: &Input) -> usize {
    input
        .map
        .keys()
        .filter(|position| position.ends_with('A'))
        .map(|start| steps_count(input, start, |position| position.ends_with('Z')))
        .reduce(lcm)
        .unwrap()
}

fn main() {
    env_logger::init();
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}
