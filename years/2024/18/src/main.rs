use std::io;

use aoc_timing::trace::log_run;
use dijkstra::DijkstraState;
use dijkstra::DijkstraVertex;
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

fn dijkstra_with_falling_blocks(
    mut map: Grid<Cell>,
    start_position: Coord,
    end_position: Coord,
    mut falling_blocks: Vec<Coord>,
) -> Option<Coord> {
    let mut dijkstra_state = DijkstraState::new(start_position);
    let mut last_removed_corrupted_block = None;

    while let Some(corrupted_block) = falling_blocks.pop() {
        dijkstra::dijkstra_with_state(
            &mut dijkstra_state,
            |coord| coord == &end_position,
            |(x, y)| {
                let is_corrupted = map.get(*x, *y).unwrap() == &Cell::Corrupted;
                map.get_neighbors(*x, *y)
                    .into_iter()
                    .map(|coord| (coord, 1))
                    .filter(move |_| !is_corrupted)
            },
        );

        if !dijkstra_state.found_ends.is_empty() {
            break;
        }

        last_removed_corrupted_block = Some(corrupted_block);
        map.update(corrupted_block.0, corrupted_block.1, Cell::Empty);
        if dijkstra_state.distances.contains_key(&corrupted_block) {
            dijkstra_state.queue.push(DijkstraVertex::new(
                corrupted_block,
                dijkstra_state.distances[&corrupted_block],
            ));
        }
    }

    last_removed_corrupted_block
}

fn part_1(input: &Input, side: usize, bytes: usize) -> Output1 {
    let mut space = Grid::<Cell>::with_size(side, side);

    for (x, y) in &input[..bytes] {
        space.update(*x, *y, Cell::Corrupted);
    }

    {
        let map: &Grid<Cell> = &space;
        let start_position = (0, 0);
        let end_position = (side - 1, side - 1);
        dijkstra::dijkstra(
            start_position,
            |coord| coord == &end_position,
            |(x, y)| {
                map.get_neighbors(*x, *y)
                    .into_iter()
                    .filter(|(column, row)| map.get(*column, *row).unwrap() != &Cell::Corrupted)
                    .map(|coord| (coord, 1))
            },
        )
        .map(|path| path.len() - 1)
    }
    .unwrap()
}

fn part_2(input: &Input, side: usize) -> Output2 {
    let mut space = Grid::with_size(side, side);

    for (x, y) in input {
        space.update(*x, *y, Cell::Corrupted);
    }

    dijkstra_with_falling_blocks(space, (0, 0), (side - 1, side - 1), input.clone())
        .map(|(x, y)| format!("{x},{y}"))
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

        let part_2 = log_run("Part 2", || part_2(&input, 71));
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
        let result = part_2(&input, 7);

        assert_eq!(result, "6,1");
    }
}
