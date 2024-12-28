use std::collections::HashSet;
use std::io;

use aoc_timing::trace::log_run;
use direction::Direction;
use grid::Grid;
use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    End,
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
    let dijkstra_state = dijkstra::dijkstra_all_shortest_paths(
        (reindeer_position, Direction::Right),
        |(position, _)| position == &end_position,
        |(position, direction)| {
            [
                direction
                    .advance(position, 1)
                    .map(|advanced| ((advanced, *direction), 1)),
                direction
                    .turn_left()
                    .advance(position, 1)
                    .map(|advanced| ((advanced, direction.turn_left()), 1001)),
                direction
                    .turn_right()
                    .advance(position, 1)
                    .map(|advanced| ((advanced, direction.turn_right()), 1001)),
            ]
            .into_iter()
            .flatten()
            .filter(|(((column, row), _), _)| {
                map.is_valid_coord(*column, *row) && map.get(*column, *row).unwrap() != &Tile::Wall
            })
        },
    )
    .unwrap();

    let mut queue = dijkstra_state
        .found_ends
        .iter()
        .cloned()
        .min_set_by_key(|(_, distance)| *distance)
        .into_iter()
        .map(|(node, _)| node)
        .collect_vec();
    let minimal_distance = dijkstra_state.distances[&queue[0]];
    let mut nodes_in_shortest_paths = HashSet::default();

    while let Some(node @ (coord, _)) = queue.pop() {
        nodes_in_shortest_paths.insert(coord);

        if let Some(prevs) = dijkstra_state.prevs.get(&node) {
            queue.append(&mut prevs.iter().copied().collect());
        }
    }

    (minimal_distance, nodes_in_shortest_paths)
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
