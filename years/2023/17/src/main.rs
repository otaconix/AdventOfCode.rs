use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;
use std::iter::once;
use std::ops::Add;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io,
    iter::successors,
};

use aoc_timing::trace::log_run;
use grid::Grid;
use itertools::Itertools;

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
    /// Determine the direction to go from `from` to `to`.
    ///
    /// Returns `None` if:
    ///   - `from == to`
    ///   - `from` and `to` aren't on either the same horizontal or vertical plane (`from.x != to.x
    ///   && from.y != to.y`)
    fn determine(from: &Coord, to: &Coord) -> Option<Self> {
        use Ordering::*;
        match (to.0.cmp(&from.0), to.1.cmp(&from.1)) {
            (Greater, Equal) => Self::Right.into(),
            (Less, Equal) => Self::Left.into(),
            (Equal, Greater) => Self::Down.into(),
            (Equal, Less) => Self::Up.into(),
            _ => None, // Coordinates are equal or not on the same horizontal/vertical axis
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
                .collect_vec()
        })
        .collect()
}

#[derive(PartialEq, Eq)]
struct DijkstraVertex<N: Eq, P: Ord> {
    distance: P,
    node: N,
}

impl<T: Eq, P: Ord> PartialOrd for DijkstraVertex<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq, P: Ord> Ord for DijkstraVertex<T, P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

fn dijkstra<
    'a,
    N: Hash + Eq + 'a,
    P: Add<P, Output = P> + Ord + Default + Copy,
    I: Iterator<Item = (&'a N, P)>,
>(
    start: &'a N,
    end: &'a N,
    neighbors: fn(&N) -> I,
) -> Option<Vec<&'a N>> {
    let mut queue: BinaryHeap<DijkstraVertex<&N, P>> = BinaryHeap::from([DijkstraVertex {
        distance: P::default(),
        node: start,
    }]);
    let mut prevs: HashMap<&N, &N> = HashMap::new();
    let mut distances: HashMap<&N, P> = HashMap::from([(start, P::default())]);

    while let Some(DijkstraVertex { distance, node }) = queue.pop() {
        if node == end {
            break;
        }

        for (neighbor, neighbor_distance) in neighbors(node) {
            let existing_distance = distances.get(&neighbor);

            if existing_distance.is_none() || neighbor_distance < *existing_distance.unwrap() {
                distances.insert(neighbor, distance + neighbor_distance);
                prevs.insert(neighbor, node);
                queue.push(DijkstraVertex {
                    distance: distance + neighbor_distance,
                    node: neighbor,
                });
            }
        }
    }

    if distances.contains_key(end) {
        let mut path = successors(Some(end), |current| prevs.remove(current)).collect_vec();
        path.reverse();
        Some(path)
    } else {
        None
    }
}

fn shortest_crucible_path(grid: &Input, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    let mut queue: BinaryHeap<DijkstraVertex<(Coord, Direction, usize), usize>> =
        BinaryHeap::from([DijkstraVertex {
            distance: 0,
            node: (start, Direction::Down, 0),
        }]);
    let mut prevs: HashMap<(Coord, Direction, usize), (Coord, Direction, usize)> = HashMap::new();
    let mut heat_losses: HashMap<(Coord, Direction, usize), usize> =
        HashMap::from([((start, Direction::Down, 0), 0)]);
    let mut found_end = None;

    while let Some(DijkstraVertex {
        node: current @ (coord, direction, count),
        distance: current_heat_loss,
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
                    node: (neighbor, new_direction, new_count),
                    distance: new_heat_loss,
                });
            }
        }
    }

    if let Some(end) = found_end {
        let mut path = successors(Some(&end), |current| prevs.get(current))
            .map(|x| x.0)
            .collect_vec();
        path.reverse();
        Some(path)
    } else {
        None
    }
}

#[allow(unused)] // Only used for debugging
fn print_grid_with_path(input: &Input, path: &[Coord]) {
    println!(
        "{}",
        (0..input.height())
            .map(|row| (0..input.width())
                .map(|column| {
                    if let Some(path_position) =
                        path.iter().position(|(x, y)| x == &column && y == &row)
                    {
                        if path_position == 0 {
                            (input.get(column, row).unwrap() + b'0') as char
                        } else {
                            char::from(
                                Direction::determine(
                                    &path[path_position - 1],
                                    &path[path_position],
                                )
                                .unwrap(),
                            )
                        }
                    } else {
                        (input.get(column, row).unwrap() + b'0') as char
                    }
                })
                .collect::<String>())
            .join("\n")
    );
}

fn part_1(input: &Input) -> usize {
    let path =
        shortest_crucible_path(input, (0, 0), (input.width() - 1, input.height() - 1)).unwrap();

    // print_grid_with_path(input, &path);

    path.into_iter()
        .skip(1)
        .map(|(column, row)| *input.get(column, row).unwrap() as usize)
        .sum()
}

fn shortest_ultra_crucible_path(grid: &Input, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    let mut queue: BinaryHeap<DijkstraVertex<(Coord, Direction, usize), usize>> =
        BinaryHeap::from([DijkstraVertex {
            distance: 0,
            node: (start, Direction::Down, 0),
        }]);
    let mut prevs: HashMap<(Coord, Direction, usize), (Coord, Direction, usize)> = HashMap::new();
    let mut heat_losses: HashMap<(Coord, Direction, usize), usize> =
        HashMap::from([((start, Direction::Down, 0), 0)]);
    let mut found_end = None;

    while let Some(DijkstraVertex {
        node: current @ (coord, direction, count),
        distance: current_heat_loss,
    }) = queue.pop()
    {
        if current.0 == end {
            // We're done.
            found_end = Some(current);
            break;
        }

        for (neighbor, new_direction, new_count) in [
            direction
                .advance_with_intermediate_coords(&coord, 1)
                .map(|coords| (coords, direction, count + 1)),
            direction
                .turn_left()
                .advance_with_intermediate_coords(&coord, 4)
                .map(|coords| (coords, direction.turn_left(), 4)),
            direction
                .turn_right()
                .advance_with_intermediate_coords(&coord, 4)
                .map(|coords| (coords, direction.turn_right(), 4)),
        ]
        .into_iter()
        .flatten()
        .filter(|(coords, _, next_count)| {
            grid.is_valid_coord(coords.last().unwrap().0, coords.last().unwrap().1)
                && next_count <= &10
        }) {
            let new_heat_loss = current_heat_loss
                + neighbor
                    .iter()
                    .map(|(column, row)| *grid.get(*column, *row).unwrap() as usize)
                    .sum::<usize>();
            let existing_heat_loss =
                heat_losses.get(&(*neighbor.last().unwrap(), new_direction, new_count));

            if existing_heat_loss.is_none() || &new_heat_loss < existing_heat_loss.unwrap() {
                heat_losses.insert(
                    (*neighbor.last().unwrap(), new_direction, new_count),
                    new_heat_loss,
                );
                prevs.insert(
                    (*neighbor.last().unwrap(), new_direction, new_count),
                    current,
                );
                queue.push(DijkstraVertex {
                    node: (*neighbor.last().unwrap(), new_direction, new_count),
                    distance: new_heat_loss,
                });
            }
        }
    }

    if let Some(end) = found_end {
        let mut path = successors(Some(&end), |current| prevs.get(current))
            .map(|x| x.0)
            .collect_vec();
        path.reverse();
        Some(path)
    } else {
        None
    }
}

fn intermediate_coords(from: &Coord, to: &Coord) -> Vec<Coord> {
    let direction = Direction::determine(from, to).unwrap();
    let distance = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);
    successors(Some(*from), |coord| direction.advance(coord, 1))
        .take(distance + 1)
        .skip(1)
        .collect()
}

fn part_2(input: &Input) -> usize {
    let path = shortest_ultra_crucible_path(input, (0, 0), (input.width() - 1, input.height() - 1))
        .unwrap();
    let path = once((0, 0))
        .chain(
            path.into_iter()
                .tuple_windows()
                .flat_map(|(from, to)| intermediate_coords(&from, &to)),
        )
        .collect_vec();

    // print_grid_with_path(input, &path);

    path.into_iter()
        .skip(1)
        .map(|(column, row)| *input.get(column, row).unwrap() as usize)
        .sum()
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

        assert_eq!(result, 102);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 94);
    }
}
