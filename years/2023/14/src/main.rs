use std::{collections::HashMap, io};

use aoc_timing::trace::log_run;
use grid::Grid;
use log::info;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
    None,
}

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Grid<Rock> {
    input
        .map(|line| {
            line.to_string()
                .chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::None,
                    _ => panic!("Unknown rock type {c}"),
                })
                .collect()
        })
        .collect()
}

fn tilt_platform(input: &mut Grid<Rock>, direction: Direction) {
    match direction {
        Direction::North => {
            for column in 0..input.width() {
                for row in 0..input.height() {
                    let rock = input.get(column, row).unwrap();

                    if rock == &Rock::None {
                        if let Some(first_round_cube_row) = (row + 1..input.height())
                            .take_while(|subrow| input.get(column, *subrow).unwrap() != &Rock::Cube)
                            .find(|subrow| input.get(column, *subrow).unwrap() == &Rock::Round)
                        {
                            input.update(column, row, Rock::Round);
                            input.update(column, first_round_cube_row, Rock::None);
                        }
                    }
                }
            }
        }
        Direction::South => {
            for column in 0..input.width() {
                for row in (0..input.height()).rev() {
                    let rock = input.get(column, row).unwrap();

                    if rock == &Rock::None {
                        if let Some(first_round_cube_row) = (0..row)
                            .rev()
                            .take_while(|subrow| input.get(column, *subrow).unwrap() != &Rock::Cube)
                            .find(|subrow| input.get(column, *subrow).unwrap() == &Rock::Round)
                        {
                            input.update(column, row, Rock::Round);
                            input.update(column, first_round_cube_row, Rock::None);
                        }
                    }
                }
            }
        }
        Direction::East => {
            for row in 0..input.height() {
                for column in (0..input.width()).rev() {
                    let rock = input.get(column, row).unwrap();

                    if rock == &Rock::None {
                        if let Some(first_round_cube_column) = (0..column)
                            .rev()
                            .take_while(|subcolumn| {
                                input.get(*subcolumn, row).unwrap() != &Rock::Cube
                            })
                            .find(|subcolumn| input.get(*subcolumn, row).unwrap() == &Rock::Round)
                        {
                            input.update(column, row, Rock::Round);
                            input.update(first_round_cube_column, row, Rock::None);
                        }
                    }
                }
            }
        }
        Direction::West => {
            for row in 0..input.height() {
                for column in 0..input.width() {
                    let rock = input.get(column, row).unwrap();

                    if rock == &Rock::None {
                        if let Some(first_round_cube_column) = (column + 1..input.width())
                            .take_while(|subcolumn| {
                                input.get(*subcolumn, row).unwrap() != &Rock::Cube
                            })
                            .find(|subcolumn| input.get(*subcolumn, row).unwrap() == &Rock::Round)
                        {
                            input.update(column, row, Rock::Round);
                            input.update(first_round_cube_column, row, Rock::None);
                        }
                    }
                }
            }
        }
    }
}

fn grid_string(input: &Grid<Rock>) -> String {
    (0..input.height())
        .map(|row| {
            (0..input.width())
                .map(|column| match input.get(column, row).unwrap() {
                    Rock::Round => 'O',
                    Rock::Cube => '#',
                    Rock::None => '.',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn score(input: &Grid<Rock>) -> usize {
    input
        .coordinates()
        .filter_map(|(column, row)| {
            if input.get(column, row).unwrap() == &Rock::Round {
                Some(input.height() - row)
            } else {
                None
            }
        })
        .sum()
}

fn part_1(input: &Grid<Rock>) -> usize {
    let mut input = input.clone();
    tilt_platform(&mut input, Direction::North);

    info!("Part 1 result\n{}", grid_string(&input));

    score(&input)
}

fn apply_tilt_cycle(input: &mut Grid<Rock>) {
    use Direction::*;

    for direction in [North, West, South, East] {
        tilt_platform(input, direction);
    }
}

fn part_2(input: &Grid<Rock>) -> usize {
    let mut input = input.clone();
    let mut seen_states = HashMap::new();

    for cycle in 0..1_000_000_000 {
        if let Some(first_occurrence) = seen_states.get(&input) {
            let repeat_length = seen_states.len() - first_occurrence;
            let remaining_cycles = 1_000_000_000 - cycle;
            let remainder_after_repeats = remaining_cycles % repeat_length;

            info!("Cycle detected.");
            info!("Cycle length: {repeat_length}");
            info!("First occurrence: {first_occurrence}");
            info!("Remainder after repeats: {remainder_after_repeats}");

            for _ in 0..remainder_after_repeats {
                apply_tilt_cycle(&mut input);
            }
            break;
        }

        seen_states.insert(input.clone(), cycle);

        apply_tilt_cycle(&mut input);
    }

    info!("Part 2 result:\n{}", grid_string(&input));

    score(&input)
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

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 136);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 64);
    }
}
