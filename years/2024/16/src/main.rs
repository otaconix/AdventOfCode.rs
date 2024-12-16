use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use aoc_macros::EnumVariants;
use aoc_timing::trace::log_run;
use aoc_utils::EnumVariants;
use grid::Grid;

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    End,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, EnumVariants)]
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
type Input = (usize, HashSet<Coord>);

type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (rows, reindeer_position, end_position) = input.enumerate().fold(
        (vec![], None, None),
        |(mut rows, mut reindeer_position, mut end_position), (row, line)| {
            let line = line.as_ref();

            rows.push(
                line.chars()
                    .enumerate()
                    .map(|(column, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'E' => {
                            end_position = Some((column, row));
                            Tile::End
                        }
                        'S' => {
                            reindeer_position = Some((column, row));
                            Tile::Empty
                        }
                        _ => panic!("Unsupported tile {c} at {column},{row}"),
                    })
                    .collect(),
            );

            (rows, reindeer_position, end_position)
        },
    );

    dijkstra(
        &Grid::new(rows).unwrap(),
        reindeer_position.unwrap(),
        end_position.unwrap(),
    )
}

fn dijkstra(
    map: &Grid<Tile>,
    reindeer_position: Coord,
    end_position: Coord,
) -> (usize, HashSet<Coord>) {
    let mut prev = HashMap::from([((reindeer_position, Direction::East), HashSet::new())]);
    let mut distances = HashMap::from([((reindeer_position, Direction::East), 0usize)]);
    let mut queue = BinaryHeap::from([Queued {
        priority: 0,
        coord: reindeer_position,
        direction: Direction::East,
    }]);

    while let Some(next) = queue.pop() {
        if next.coord == end_position {
            // We've found the end! Don't stop entirely, but there's no point in going further
            // along this path.
            continue;
        }

        let prev_distance = distances[&(next.coord, next.direction)];

        let potential_nexts = [
            next.direction
                .advance(&next.coord, map)
                .map(|advanced| (prev_distance + 1, advanced, next.direction)),
            next.direction
                .turn_left()
                .advance(&next.coord, map)
                .map(|advanced| (prev_distance + 1001, advanced, next.direction.turn_left())),
            next.direction
                .turn_right()
                .advance(&next.coord, map)
                .map(|advanced| (prev_distance + 1001, advanced, next.direction.turn_right())),
        ];

        for (next_distance, next_coord, next_direction) in potential_nexts
            .into_iter()
            .flatten()
            .filter(|(_, (column, row), _)| map.get(*column, *row).unwrap() != &Tile::Wall)
        {
            {
                let distance_compared_to_original = distances
                    .get(&(next_coord, next_direction))
                    .map(|original| next_distance.cmp(original))
                    .unwrap_or(std::cmp::Ordering::Less);

                if distance_compared_to_original.is_le() {
                    let prevs = prev.entry((next_coord, next_direction)).or_default();

                    if distance_compared_to_original.is_lt() {
                        prevs.clear();
                    }
                    prevs.insert((next.coord, next.direction));

                    distances.insert((next_coord, next_direction), next_distance);
                    queue.push(Queued {
                        priority: next_distance,
                        coord: next_coord,
                        direction: next_direction,
                    });
                }
            }
        }
    }

    let minimal_distance = Direction::variants()
        .into_iter()
        .flat_map(|direction| distances.get(&(end_position, direction)))
        .copied()
        .min()
        .unwrap();

    let mut tiles_in_optimal_paths = HashSet::new();
    let mut queue = Direction::variants()
        .into_iter()
        .map(|direction| (end_position, direction))
        .filter(|directed_tile| {
            distances
                .get(directed_tile)
                .map(|distance| *distance == minimal_distance)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    while let Some(directed_tile) = queue.pop() {
        tiles_in_optimal_paths.insert(directed_tile.0);

        queue.extend(prev[&directed_tile].iter());
    }

    (minimal_distance, tiles_in_optimal_paths)
}

fn part_1((shortest_distance, _): &Input) -> Output {
    *shortest_distance
}

fn part_2((_, prev): &Input) -> Output {
    prev.len()
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

        assert_eq!(result, 64);
    }
}
