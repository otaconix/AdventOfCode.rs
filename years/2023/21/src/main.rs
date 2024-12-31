use std::collections::HashSet;
use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;

type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    GardenPlot,
    Rock,
}

struct Input {
    map: Grid<Cell>,
    starting_position: Coord,
}
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let mut starting_position = (0, 0);
    let map = Grid::new(
        input
            .enumerate()
            .map(|(row, line)| {
                let line = line.as_ref();

                line.char_indices()
                    .map(|(column, c)| match c {
                        '.' => Cell::GardenPlot,
                        '#' => Cell::Rock,
                        'S' => {
                            starting_position = (column, row);

                            Cell::GardenPlot
                        }
                        _ => panic!("Unknown map character: {c}"),
                    })
                    .collect()
            })
            .collect(),
    )
    .unwrap();

    Input {
        map,
        starting_position,
    }
}

fn step(map: &Grid<Cell>, positions: HashSet<Coord>) -> HashSet<Coord> {
    positions
        .into_iter()
        .flat_map(|(column, row)| {
            map.get_neighbors(column, row)
                .into_iter()
                .filter(|(column, row)| map.get(*column, *row).unwrap() != &Cell::Rock)
        })
        .collect()
}

fn part_1(input: &Input, steps: usize) -> Output1 {
    (0..steps)
        .fold(HashSet::from([input.starting_position]), |positions, _| {
            step(&input.map, positions)
        })
        .len()
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

        let part_1 = log_run("Part 1", || part_1(&input, 64));
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
        let result = part_1(&input, 6);

        assert_eq!(result, 16);
    }
}
