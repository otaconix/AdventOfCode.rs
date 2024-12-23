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

#[derive(PartialEq, Eq)]
struct QueueItem {
    distance: usize,
    position: Coord,
}

impl QueueItem {
    fn new(priority: usize, position: Coord) -> Self {
        Self {
            distance: priority,
            position,
        }
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &Grid<Cell>, start_position: Coord, end_position: Coord) -> Option<usize> {
    let mut distances = FxHashMap::default();
    distances.insert(start_position, 0usize);
    let mut queue = BTreeSet::from([QueueItem::new(0, start_position)]);

    while let Some(QueueItem {
        distance,
        position: position @ (x, y),
    }) = queue.pop_last()
    {
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
                if distances.get(&potential_next).unwrap_or(&usize::MAX) > &distance {
                    distances.insert(potential_next, distance + 1);
                    queue.insert(QueueItem::new(distance + 1, potential_next));
                }
            }
        }
    }

    None
}

fn dijkstra_with_falling_blocks(
    mut map: Grid<Cell>,
    start_position: Coord,
    end_position: Coord,
    mut falling_blocks: Vec<Coord>,
) -> Option<Coord> {
    let mut distances = FxHashMap::default();
    distances.insert(start_position, 0usize);
    let mut queue = BTreeSet::from([QueueItem::new(0, start_position)]);
    let mut last_removed_corrupted_block = None;

    'outer_loop: while let Some(corrupted_block) = falling_blocks.pop() {
        while let Some(QueueItem {
            distance,
            position: position @ (x, y),
        }) = queue.pop_last()
        {
            if map.get(x, y).unwrap() == &Cell::Corrupted {
                continue;
            }

            if position == end_position {
                // We've found the end!
                break 'outer_loop;
            }

            for potential_next in [
                Some((x + 1, y)),
                Some((x, y + 1)),
                x.checked_sub(1).map(|x| (x, y)),
                y.checked_sub(1).map(|y| (x, y)),
            ]
            .into_iter()
            .flatten()
            .filter(|(column, row)| *column < map.width() && *row < map.height())
            {
                {
                    if distances.get(&potential_next).unwrap_or(&usize::MAX) > &distance {
                        distances.insert(potential_next, distance + 1);
                        queue.insert(QueueItem::new(distance + 1, potential_next));
                    }
                }
            }
        }

        last_removed_corrupted_block = Some(corrupted_block);
        map.update(corrupted_block.0, corrupted_block.1, Cell::Empty);
        if distances.contains_key(&corrupted_block) {
            queue.insert(QueueItem::new(distances[&corrupted_block], corrupted_block));
        }
    }

    last_removed_corrupted_block
}

fn part_1(input: &Input, side: usize, bytes: usize) -> Output1 {
    let mut space = Grid::<Cell>::with_size(side, side);

    for (x, y) in &input[..bytes] {
        space.update(*x, *y, Cell::Corrupted);
    }

    dijkstra(&space, (0, 0), (side - 1, side - 1)).unwrap()
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
