use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::iter::once;

use aoc_timing::trace::log_run;
use grid::Grid;
use itertools::Itertools;

#[derive(PartialEq, Eq)]
struct Queued {
    distance: usize,
    coord: Coord,
}

impl Queued {
    fn new(distance: usize, coord: Coord) -> Self {
        Self { distance, coord }
    }
}

impl Ord for Queued {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for Queued {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Input {
    codes: Vec<String>,
    numeric_paths: HashMap<(char, char), Vec<String>>,
    directional_paths: HashMap<(char, char), Vec<String>>,
}

type Coord = (usize, usize);
type Output1 = usize;
type Output2 = Output1;

const NUMERIC_KEYPAD: &str = include_str!("numeric_keypad");
const DIRECTIONAL_KEYPAD: &str = include_str!("directional_keypad");

fn dijkstra_paths(map: &Grid<char>, start_position: Coord, end_position: Coord) -> Vec<String> {
    let mut prev = HashMap::from([(start_position, HashSet::new())]);
    let mut distances = HashMap::from([(start_position, 0usize)]);
    let mut queue = BinaryHeap::from([Queued::new(0, start_position)]);

    while let Some(current) = queue.pop() {
        if current.coord == end_position {
            // We've found the end! Don't stop entirely, but there's no point in going further
            // along this path.
            continue;
        }

        let prev_distance = distances[&current.coord];

        for potential_next in [
            Some((current.coord.0 + 1, current.coord.1)),
            Some((current.coord.0, current.coord.1 + 1)),
            current.coord.0.checked_sub(1).map(|x| (x, current.coord.1)),
            current.coord.1.checked_sub(1).map(|y| (current.coord.0, y)),
        ]
        .into_iter()
        .flatten()
        .filter(|(column, row)| {
            *column < map.width() && *row < map.height() && map.get(*column, *row).unwrap() != &'#'
        }) {
            {
                let next_distance = prev_distance + 1;
                let distance_compared_to_original = distances
                    .get(&potential_next)
                    .map(|original| next_distance.cmp(original))
                    .unwrap_or(Ordering::Less);

                if distance_compared_to_original.is_le() {
                    let prevs = prev.entry(potential_next).or_default();

                    if distance_compared_to_original.is_lt() {
                        prevs.clear();
                        distances.insert(potential_next, next_distance);
                        queue.push(Queued::new(next_distance, potential_next));
                    }
                    prevs.insert(current.coord);
                }
            }
        }
    }

    fn paths(current: &Coord, prev_map: &HashMap<Coord, HashSet<Coord>>) -> Vec<Vec<Coord>> {
        if let Some(prevs) = prev_map.get(current).filter(|prevs| !prevs.is_empty()) {
            prevs
                .iter()
                .flat_map(|prev| {
                    paths(prev, prev_map)
                        .into_iter()
                        .map(|mut path| {
                            path.push(*current);
                            path
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            vec![vec![*current]]
        }
    }

    paths(&end_position, &prev)
        .into_iter()
        .map(|path| {
            path.into_iter()
                .tuple_windows()
                .map(
                    |(a, b)| match (a.0 as isize - b.0 as isize, a.1 as isize - b.1 as isize) {
                        (-1, _) => '>',
                        (1, _) => '<',
                        (_, -1) => 'v',
                        (_, 1) => '^',
                        _ => panic!("More than one step?"),
                    },
                )
                .collect()
        })
        .collect()
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
                    .collect(),
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
                    .collect(),
            )
        })
        .collect();

    Input {
        codes,
        numeric_paths,
        directional_paths,
    }
}

fn flatten_paths(result: Vec<String>, paths: Vec<String>) -> Vec<String> {
    result
        .into_iter()
        .flat_map(|result_path| paths.iter().map(move |path| result_path.clone() + path))
        .collect()
}

fn part_1(input: &Input) -> Output1 {
    let numeric_paths = input
        .codes
        .iter()
        .map(|code| {
            once('A')
                .chain(code.chars())
                .tuple_windows()
                .map(|(from, to)| input.numeric_paths[&(from, to)].clone())
                .fold(vec![String::new()], flatten_paths)
        })
        .collect::<Vec<_>>();

    let first_directional_paths = numeric_paths
        .iter()
        .map(|paths| {
            paths
                .iter()
                .flat_map(|path| {
                    once('A')
                        .chain(path.chars())
                        .tuple_windows()
                        .map(|(from, to)| {
                            input
                                .directional_paths
                                .get(&(from, to))
                                .unwrap_or(&vec!["A".to_string()])
                                .clone()
                        })
                        .fold(vec![String::new()], flatten_paths)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let human_directional_paths_shortest_lengths = first_directional_paths // paths for all codes
        .iter()
        .flat_map(|paths| {
            // paths per single code
            paths
                .iter()
                .flat_map(|path| {
                    // single path for single code
                    once('A')
                        .chain(path.chars())
                        .tuple_windows()
                        .map(|(from, to)| {
                            input
                                .directional_paths
                                .get(&(from, to))
                                .unwrap_or(&vec!["A".to_string()])
                                .clone()
                        })
                        .fold(vec![String::new()], flatten_paths)
                })
                .map(|s| s.len())
                .min()
        })
        .collect::<Vec<_>>();

    input
        .codes
        .iter()
        .map(|code| code[0..code.len() - 1].parse::<usize>().unwrap())
        .zip(human_directional_paths_shortest_lengths)
        .map(|(code, shortest_sequence_length)| code * shortest_sequence_length)
        .sum()
}

fn path_movement(path: &str) -> usize {
    path.chars()
        .map(|c| match c {
            '#' => (0usize, 0usize),
            '^' => (1, 0),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => panic!("Invalid movement character: {c}"),
        })
        .tuple_windows()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum()
}

fn part_2(input: &Input) -> Output2 {
    // let numeric_paths = input
    //     .codes
    //     .iter()
    //     .map(|code| {
    //         once('A')
    //             .chain(code.chars())
    //             .tuple_windows()
    //             .map(|(from, to)| input.numeric_paths[&(from, to)].clone())
    //             .fold(vec![String::new()], flatten_paths)
    //     })
    //     .collect::<Vec<_>>();
    //
    // let robot_directional_paths = (0..25).fold(numeric_paths, |paths, _| {
    //     paths
    //         .iter()
    //         .map(|paths| {
    //             paths
    //                 .iter()
    //                 .flat_map(|path| {
    //                     once('A')
    //                         .chain(path.chars())
    //                         .tuple_windows()
    //                         .map(|(from, to)| {
    //                             input
    //                                 .directional_paths
    //                                 .get(&(from, to))
    //                                 .unwrap_or(&vec!["A".to_string()])
    //                                 .clone()
    //                         })
    //                         .fold(vec![String::new()], flatten_paths)
    //                         .into_iter()
    //                         .min_by_key(|path| path_movement(path))
    //                 })
    //                 .collect::<Vec<_>>()
    //         })
    //         .collect::<Vec<_>>()
    // });
    //
    // let human_directional_paths_shortest_lengths = robot_directional_paths
    //     .iter()
    //     .flat_map(|paths| {
    //         paths
    //             .iter()
    //             .flat_map(|path| {
    //                 once('A')
    //                     .chain(path.chars())
    //                     .tuple_windows()
    //                     .map(|(from, to)| {
    //                         input
    //                             .directional_paths
    //                             .get(&(from, to))
    //                             .unwrap_or(&vec!["A".to_string()])
    //                             .clone()
    //                     })
    //                     .fold(vec![String::new()], flatten_paths)
    //             })
    //             .map(|s| s.len())
    //             .min()
    //     })
    //     .collect::<Vec<_>>();
    //
    // input
    //     .codes
    //     .iter()
    //     .map(|code| code[0..code.len() - 1].parse::<usize>().unwrap())
    //     .zip(human_directional_paths_shortest_lengths)
    //     .map(|(code, shortest_sequence_length)| code * shortest_sequence_length)
    //     .sum()
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
    fn test_paths() {
        let input = parse(INPUT.lines());
        let paths_from_2_to_9 = &input.numeric_paths[&('2', '9')];

        assert_eq!(paths_from_2_to_9.len(), 3);
        assert!(paths_from_2_to_9.contains(&"^^>".to_string()));
        assert!(paths_from_2_to_9.contains(&"^>^".to_string()));
        assert!(paths_from_2_to_9.contains(&">^^".to_string()));
    }

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

        assert_eq!(result, 0);
    }
}
