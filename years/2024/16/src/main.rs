use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    End,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq)]
struct Queued {
    priority: usize,
    coord: Coord,
    direction: Direction,
}

impl Ord for Queued {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for Queued {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn advance(&self, coord: &Coord, map: &Grid<Tile>) -> Option<Coord> {
        match *self {
            Direction::North if coord.1 > 0 => Some((coord.0, coord.1 - 1)),
            Direction::East => Some((coord.0 + 1, coord.1)),
            Direction::South => Some((coord.0, coord.1 + 1)),
            Direction::West if coord.0 > 0 => Some((coord.0 - 1, coord.1)),
            _ => None,
        }
        .filter(|(column, row)| {
            (0..map.width()).contains(column) && (0..map.height()).contains(row)
        })
    }
}

type Coord = (usize, usize);
struct Input {
    map: Grid<Tile>,
    reindeer_position: Coord,
}

type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (rows, reindeer_position) = input.enumerate().fold(
        (vec![], None),
        |(mut rows, mut reindeer_position), (row, line)| {
            let line = line.as_ref();

            rows.push(
                line.chars()
                    .enumerate()
                    .map(|(column, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'E' => Tile::End,
                        'S' => {
                            reindeer_position = Some((column, row));
                            Tile::Empty
                        }
                        _ => panic!("Unsupported tile {c} at {column},{row}"),
                    })
                    .collect(),
            );

            (rows, reindeer_position)
        },
    );

    Input {
        map: Grid::new(rows).unwrap(),
        reindeer_position: reindeer_position.unwrap(),
    }
}

fn part_1(input: &Input) -> Output {
    let mut prev = HashMap::new();
    prev.insert(input.reindeer_position, None);
    let mut distances = HashMap::new();
    distances.insert(input.reindeer_position, 0usize);
    let mut queue = BinaryHeap::new();
    queue.push(Queued {
        priority: 0,
        coord: input.reindeer_position,
        direction: Direction::East,
    });

    while let Some(next) = queue.pop() {
        if input.map.get(next.coord.0, next.coord.1).unwrap() == &Tile::End {
            return distances[&next.coord];
        }

        let prev_distance = distances[&next.coord];
        if let Some((next_distance, next_coord, next_direction)) = next
            .direction
            .advance(&next.coord, &input.map)
            .map(|advanced| (prev_distance + 1, advanced, next.direction))
        {
            if input.map.get(next_coord.0, next_coord.1).unwrap() != &Tile::Wall
                && distances
                    .get(&next_coord)
                    .map(|original| next_distance < *original)
                    .unwrap_or(true)
            {
                prev.insert(next_coord, Some(next.coord));
                distances.insert(next_coord, next_distance);
                queue.push(Queued {
                    priority: next_distance,
                    coord: next_coord,
                    direction: next_direction,
                });
            }
        }

        if let Some((next_distance, next_coord, next_direction)) = next
            .direction
            .turn_left()
            .advance(&next.coord, &input.map)
            .map(|advanced| (prev_distance + 1001, advanced, next.direction.turn_left()))
        {
            if input.map.get(next_coord.0, next_coord.1).unwrap() != &Tile::Wall
                && distances
                    .get(&next_coord)
                    .map(|original| next_distance < *original)
                    .unwrap_or(true)
            {
                prev.insert(next_coord, Some(next.coord));
                distances.insert(next_coord, next_distance);
                queue.push(Queued {
                    priority: next_distance,
                    coord: next_coord,
                    direction: next_direction,
                });
            }
        }

        if let Some((next_distance, next_coord, next_direction)) = next
            .direction
            .turn_right()
            .advance(&next.coord, &input.map)
            .map(|advanced| (prev_distance + 1001, advanced, next.direction.turn_right()))
        {
            if input.map.get(next_coord.0, next_coord.1).unwrap() != &Tile::Wall
                && distances
                    .get(&next_coord)
                    .map(|original| next_distance < *original)
                    .unwrap_or(true)
            {
                prev.insert(next_coord, Some(next.coord));
                distances.insert(next_coord, next_distance);
                queue.push(Queued {
                    priority: next_distance,
                    coord: next_coord,
                    direction: next_direction,
                });
            }
        }
    }

    0
}

fn part_2(input: &Input) -> Output {
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

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 11048);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 0);
    }
}
