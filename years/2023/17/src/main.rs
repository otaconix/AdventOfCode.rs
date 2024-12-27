use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io,
    iter::successors,
};

use aoc_timing::trace::log_run;
use grid::Grid;

type Input = Grid<u8>;
type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

impl Direction {
    fn determine(from: &Coord, to: &Coord) -> Self {
        match (to.0.cmp(&from.0), to.1.cmp(&from.1)) {
            (Ordering::Greater, _) => Self::Right,
            (Ordering::Less, _) => Self::Left,
            (_, Ordering::Greater) => Self::Down,
            (_, Ordering::Less) => Self::Up,
            _ => panic!("Can't go diagonally!"),
        }
    }

    fn turn_around(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn advance(&self, coord: &Coord, steps: usize) -> Option<Coord> {
        match self {
            Direction::Up => coord.1.checked_sub(steps).map(|y| (coord.0, y)),
            Direction::Down => Some((coord.0, coord.1 + steps)),
            Direction::Left => coord.0.checked_sub(steps).map(|x| (x, coord.1)),
            Direction::Right => Some((coord.0 + steps, coord.1)),
        }
    }

    fn advance_with_intermediate_coords(&self, coord: &Coord, steps: usize) -> Option<Vec<Coord>> {
        let result = successors(self.advance(coord, 1), |next| self.advance(next, 1))
            .take(steps)
            .collect::<Vec<_>>();

        if result.len() == steps {
            Some(result)
        } else {
            None
        }
    }
}

impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            line.as_ref()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(PartialEq, Eq, Hash)]
struct DijkstraVertex<T: Hash, P: Ord + Hash> {
    priority: P,
    value: T,
}

impl<T: Eq + Hash, P: Ord + Hash> PartialOrd for DijkstraVertex<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + Hash, P: Ord + Hash> Ord for DijkstraVertex<T, P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

fn shortest_path(grid: &Input, start: Coord, end: Coord) -> Option<Vec<(Coord, Direction)>> {
    let mut queue: BinaryHeap<DijkstraVertex<(Coord, Direction, usize), usize>> =
        BinaryHeap::from([DijkstraVertex {
            priority: 0,
            value: (start, Direction::Down, 0),
        }]);
    let mut prevs: HashMap<(Coord, Direction, usize), (Coord, Direction, usize)> = HashMap::new();
    let mut heat_losses: HashMap<(Coord, Direction, usize), usize> =
        HashMap::from([((start, Direction::Down, 0), 0)]);
    let mut found_end = None;

    while let Some(DijkstraVertex {
        value: current @ (coord, direction, count),
        priority: current_heat_loss,
    }) = queue.pop()
    {
        if current.0 == end {
            // We're done.
            found_end = Some(current);
            break;
        }

        for (neighbor, new_direction, new_count) in [
            direction
                .advance(&coord, 1)
                .map(|coord| (coord, direction, count + 1)),
            direction
                .turn_left()
                .advance(&coord, 1)
                .map(|coord| (coord, direction.turn_left(), 1)),
            direction
                .turn_right()
                .advance(&coord, 1)
                .map(|coord| (coord, direction.turn_right(), 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|((next_column, next_row), _, next_count)| {
            grid.is_valid_coord(*next_column, *next_row) && next_count <= &3
        }) {
            let new_heat_loss =
                current_heat_loss + *grid.get(neighbor.0, neighbor.1).unwrap() as usize;
            let existing_heat_loss = heat_losses.get(&(neighbor, new_direction, new_count));

            if existing_heat_loss.is_none() || &new_heat_loss < existing_heat_loss.unwrap() {
                heat_losses.insert((neighbor, new_direction, new_count), new_heat_loss);
                prevs.insert((neighbor, new_direction, new_count), current);
                queue.push(DijkstraVertex {
                    value: (neighbor, new_direction, new_count),
                    priority: new_heat_loss,
                });
            }
        }
    }

    if let Some(end) = found_end {
        let mut path = successors(Some(&end), |current| prevs.get(current))
            .map(|x| (x.0, x.1))
            .collect::<Vec<_>>();
        path.reverse();
        Some(path)
    } else {
        None
    }
}

fn part_1(input: &Input) -> usize {
    let shortest = shortest_path(input, (0, 0), (input.width() - 1, input.height() - 1)).unwrap();
    // println!(
    //     "{}",
    //     (0..input.height())
    //         .map(|row| (0..input.width())
    //             .map(|column| {
    //                 if let Some((_, direction)) = shortest
    //                     .iter()
    //                     .find(|((x, y), _)| x == &column && y == &row)
    //                 {
    //                     char::from(*direction)
    //                 } else {
    //                     (input.get(column, row).unwrap() + b'0') as char
    //                 }
    //             })
    //             .collect::<String>())
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // );

    shortest
        .into_iter()
        .skip(1)
        .map(|((column, row), _)| *input.get(column, row).unwrap() as usize)
        .sum()
}

// fn part_2(input: &Input) -> usize {
//     todo!()
// }

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        // let part_2 = log_run("Part 2", || part_2(&input));
        // println!("Part 2: {part_2}");
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

        assert_eq!(result, 102);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = parse(INPUT.lines());
    //     let result = part_2(&input);
    //
    //     assert_eq!(result, 0);
    // }
}
