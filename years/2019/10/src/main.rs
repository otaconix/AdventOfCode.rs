use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Asteroid,
    Empty,
}

type Input = Grid<Cell>;
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.chars()
                .map(|c| match c {
                    '#' => Cell::Asteroid,
                    '.' => Cell::Empty,
                    _ => panic!("Unknown cell type: '{c}'"),
                })
                .collect()
        })
        .collect()
}

fn part_1(input: &Input) -> Output1 {
    input
        .coordinates()
        .map(|coord| (coord, input.get_coord(coord).unwrap()))
        .filter(|(_, cell)| *cell == &Cell::Asteroid)
        .count()
}

fn part_2(input: &Input) -> Output2 {
    todo!()
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input));
        println!("Part 2: {part_2}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    // #[test]
    // fn test_part_1() {
    //     let input = parse(INPUT.lines());
    //     let result = part_1(&input);
    //
    //     assert_eq!(result, 210);
    // }
    //
    // #[test]
    // fn test_part_2() {
    //     let input = parse(INPUT.lines());
    //     let result = part_2(&input);
    //
    //     assert_eq!(result, 0);
    // }
}
