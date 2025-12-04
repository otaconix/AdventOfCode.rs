use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;
use rapidhash::RapidHashSet;

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    PaperRoll,
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
                    '@' => Cell::PaperRoll,
                    '.' => Cell::Empty,
                    _ => panic!("Unknown cell type {c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part_1(input: &Input) -> Output1 {
    input
        .coordinates()
        .map(|(column, row)| {
            (
                input.get(column, row).unwrap(),
                input
                    .get_neighbors_incl_diagonals_iter(column, row)
                    .filter(|neighbor| input.get_coord(*neighbor).unwrap() == &Cell::PaperRoll)
                    .count(),
            )
        })
        .filter(|(cell, n)| *cell == &Cell::PaperRoll && *n < 4)
        .count()
}

fn part_2(input: &Input) -> Output2 {
    let mut paper_roll_coords = RapidHashSet::default();
    paper_roll_coords.extend(
        input
            .coordinates()
            .filter(|coord| input.get_coord(*coord).unwrap() == &Cell::PaperRoll),
    );
    let mut removed = 0;

    loop {
        let to_remove = paper_roll_coords
            .iter()
            .copied()
            .filter(|(column, row)| {
                input
                    .get_neighbors_incl_diagonals_iter(*column, *row)
                    .filter(|neighbor| paper_roll_coords.contains(neighbor))
                    .count()
                    < 4
            })
            .collect::<Vec<_>>();

        if to_remove.is_empty() {
            break;
        }

        removed += to_remove.len();

        for to_remove in to_remove.into_iter() {
            paper_roll_coords.remove(&to_remove);
        }
    }

    removed
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

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 43);
    }
}
