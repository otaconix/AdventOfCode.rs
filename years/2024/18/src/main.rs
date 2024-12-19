use std::collections::BTreeSet;
use std::io;

use aoc_timing::trace::log_run;
use fxhash::FxHashMap;
use grid::Grid;
use itertools::Itertools;

type Coord = (usize, usize);
type Input = Vec<Coord>;
type Output1 = usize;
type Output2 = String;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Cell {
    #[default]
    Empty,
    Corrupted,
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            line.as_ref()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn dijkstra(map: &Grid<Cell>, start_position: Coord, end_position: Coord) -> Option<usize> {
    let mut prev = FxHashMap::default();
    prev.insert(start_position, None);
    let mut distances = FxHashMap::default();
    distances.insert(start_position, 0usize);
    let mut queue = BTreeSet::from([(0, start_position)]);

    while let Some((distance, position @ (x, y))) = queue.pop_first() {
        if position == end_position {
            // We've found the end!
            return Some(distance);
        }

        for potential_next in [
            Some((x + 1, y)),
            Some((x, y + 1)),
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
        ]
        .into_iter()
        .flatten()
        .filter(|(column, row)| {
            *column < map.width()
                && *row < map.height()
                && map.get(*column, *row).unwrap() != &Cell::Corrupted
        }) {
            {
                if distances
                    .get(&potential_next)
                    .filter(|original| distance >= **original)
                    .is_none()
                {
                    prev.insert(potential_next, Some(position));
                    distances.insert(potential_next, distance + 1);
                    queue.insert((distance + 1, potential_next));
                }
            }
        }
    }

    None
}

fn part_1(input: &Input, side: usize, bytes: usize) -> Output1 {
    let mut space = Grid::<Cell>::with_size(side, side);

    for (x, y) in &input[..bytes] {
        space.update(*x, *y, Cell::Corrupted);
    }

    dijkstra(&space, (0, 0), (side - 1, side - 1)).unwrap()
}

fn part_2(input: &Input, side: usize, initial_bytes: usize) -> Output2 {
    let mut space = Grid::with_size(side, side);

    for (x, y) in &input[..initial_bytes] {
        space.update(*x, *y, Cell::Corrupted);
    }

    input
        .iter()
        .skip(initial_bytes)
        .find(|(x, y)| {
            space.update(*x, *y, Cell::Corrupted);

            dijkstra(&space, (0, 0), (side - 1, side - 1)).is_none()
        })
        .map(|(x, y)| format!("{},{}", x, y))
        .unwrap()
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input, 71, 1024));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input, 71, 1024));
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
        let result = part_1(&input, 7, 12);

        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input, 7, 12);

        assert_eq!(result, "6,1");
    }
}
