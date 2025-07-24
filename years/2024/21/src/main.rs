use std::collections::HashMap;
use std::io;
use std::iter::once;

use aoc_timing::trace::log_run;
use dijkstra::dijkstra_all_shortest_paths;
use grid::Grid;
use itertools::Itertools;

struct Input {
    codes: Vec<String>,
    numeric_paths: HashMap<(char, char), String>,
    directional_paths: HashMap<(char, char), String>,
}

type Coord = (usize, usize);
type Output = usize;

const NUMERIC_KEYPAD: &str = include_str!("numeric_keypad");
const DIRECTIONAL_KEYPAD: &str = include_str!("directional_keypad");

fn dijkstra_paths(map: &Grid<char>, start_position: Coord, end_position: Coord) -> Vec<String> {
    dijkstra_all_shortest_paths(
        start_position,
        |coord| coord == &end_position,
        |(column, row)| {
            map.get_neighbors(*column, *row)
                .into_iter()
                .filter(|(column, row)| map.get(*column, *row).unwrap() != &'#')
                .map(|coord| (coord, 1))
        },
    )
    .unwrap()
    .build_minimal_paths()
    .unwrap()
    .into_iter()
    .map(|path| {
        use std::cmp::Ordering::{Less, Greater};
        path.into_iter()
            .map(|(coord, _)| coord)
            .tuple_windows()
            .map(|(a, b)| match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
                (Less, _) => '>',
                (Greater, _) => '<',
                (_, Less) => 'v',
                (_, Greater) => '^',
                _ => panic!("More than one step?"),
            })
            .collect()
    })
    .collect()
}

fn path_movement(path: &str) -> usize {
    path.chars()
        .dedup()
        .map(|c| match c {
            '^' => (1usize, 0usize),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => panic!("Invalid movement character: {c}"),
        })
        .tuple_windows()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum::<usize>()
        * path.chars().dedup().count()
        + path // prefer going right first
            .chars()
            .rev()
            .positions(|c| c == '>')
            .sum::<usize>()
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let codes = input.map(|line| line.as_ref().to_string()).collect();

    let directional_grid = Grid::new(
        DIRECTIONAL_KEYPAD
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    )
    .unwrap();
    let directional_paths = directional_grid
        .coordinates()
        .filter(|(column, row)| directional_grid.get(*column, *row).unwrap() != &'#')
        .permutations(2)
        .map(|from_to| {
            (
                (
                    *directional_grid.get(from_to[0].0, from_to[0].1).unwrap(),
                    *directional_grid.get(from_to[1].0, from_to[1].1).unwrap(),
                ),
                dijkstra_paths(&directional_grid, from_to[0], from_to[1])
                    .into_iter()
                    .map(|mut path| {
                        path.push('A');
                        path
                    })
                    .min_by_key(|path| path_movement(path))
                    .unwrap(),
            )
        })
        .collect();

    let numeric_grid = Grid::new(
        NUMERIC_KEYPAD
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    )
    .unwrap();
    let numeric_paths = numeric_grid
        .coordinates()
        .filter(|(column, row)| numeric_grid.get(*column, *row).unwrap() != &'#')
        .permutations(2)
        .map(|from_to| {
            (
                (
                    *numeric_grid.get(from_to[0].0, from_to[0].1).unwrap(),
                    *numeric_grid.get(from_to[1].0, from_to[1].1).unwrap(),
                ),
                dijkstra_paths(&numeric_grid, from_to[0], from_to[1])
                    .into_iter()
                    .map(|mut path| {
                        path.push('A');
                        path
                    })
                    .min_by_key(|path| path_movement(path))
                    .unwrap(),
            )
        })
        .collect();

    Input {
        codes,
        numeric_paths,
        directional_paths,
    }
}

fn path_length(
    path: &str,
    directional_robot_count: usize,
    directional_paths: &HashMap<(char, char), String>,
    cache: &mut HashMap<((char, char), usize), usize>,
) -> usize {
    if directional_robot_count == 0 {
        path.len()
    } else if path.is_empty() {
        0
    } else {
        once('A')
            .chain(path.chars())
            .tuple_windows()
            .map(|from_to| {
                if let Some(length) = cache.get(&(from_to, directional_robot_count)) {
                    *length
                } else {
                    let length = path_length(
                        directional_paths.get(&from_to).unwrap_or(&"A".to_string()),
                        directional_robot_count - 1,
                        directional_paths,
                        cache,
                    );

                    cache.insert((from_to, directional_robot_count), length);

                    length
                }
            })
            .sum()
    }
}

fn solve(input: &Input, directional_robot_count: usize) -> Output {
    let numeric_paths = input
        .codes
        .iter()
        .map(|code| {
            once('A')
                .chain(code.chars())
                .tuple_windows()
                .map(|(from, to)| input.numeric_paths[&(from, to)].clone())
                .join("")
        })
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let path_lengths = numeric_paths
        .into_iter()
        .map(|path| {
            path_length(
                &path,
                directional_robot_count + 1,
                &input.directional_paths,
                &mut cache,
            )
        })
        .collect::<Vec<_>>();

    input
        .codes
        .iter()
        .map(|code| code[0..code.len() - 1].parse::<usize>().unwrap())
        .zip(path_lengths)
        .map(|(code, path_length)| code * path_length)
        .sum()
}

fn part_1(input: &Input) -> Output {
    solve(input, 1)
}

fn part_2(input: &Input) -> Output {
    solve(input, 24)
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

        assert_eq!(result, 126384);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 154115708116294);
    }
}
