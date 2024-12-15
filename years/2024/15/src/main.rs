use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;
use indexmap::IndexSet;
use itertools::Itertools;
use log::log;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Wall,
    Empty,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Coord = (usize, usize);
struct Input {
    map: Grid<Cell>,
    robot_position: Coord,
    movements: Vec<Direction>,
}

// Since we're working with `usize`, representing a delta using negative numbers doesn't work.
// Let's lift +1 & -1 into functions, then!
type DeltaFn = fn(usize) -> usize;
type Delta = (DeltaFn, DeltaFn);
const ADD: DeltaFn = |n| n + 1;
const SUB: DeltaFn = |n| n.saturating_sub(1);
const NOP: DeltaFn = |n| n;

type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum State {
        Map(Vec<Vec<Cell>>, Option<Coord>),
        Movements(Vec<Vec<Cell>>, Coord, Vec<Direction>),
    }

    use State::*;

    let state = input
        .enumerate()
        .fold(Map(vec![], None), |state, (row, line)| {
            let line = line.as_ref();

            match state {
                Map(rows, robot_position) if line.is_empty() => {
                    Movements(rows, robot_position.unwrap(), vec![])
                }
                Map(mut rows, mut robot_position) => {
                    rows.push(
                        line.chars()
                            .enumerate()
                            .map(|(column, c)| match c {
                                '#' => Cell::Wall,
                                '.' => Cell::Empty,
                                'O' => Cell::Box,
                                '@' if robot_position.is_none() => {
                                    robot_position = Some((column, row));
                                    Cell::Empty
                                }
                                '@' => panic!("Multiple robots!?"),
                                _ => panic!("Unknown cell type: {c}"),
                            })
                            .collect(),
                    );

                    Map(rows, robot_position)
                }
                Movements(rows, robot_position, mut movements) => {
                    movements.extend(line.chars().map(|c| match c {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => panic!("Unknown direction: {c}"),
                    }));
                    Movements(rows, robot_position, movements)
                }
            }
        });

    match state {
        Movements(rows, robot_position, movements) => Input {
            map: Grid::new(rows).unwrap(),
            robot_position,
            movements,
        },
        _ => panic!("Haven't reached done state while parsing"),
    }
}

fn print_map(level: log::Level, map: &Grid<Cell>, robot_position: Coord) {
    if log::log_enabled!(level) {
        log!(
            level,
            "\n{}",
            (0..map.height())
                .map(|row| {
                    (0..map.width())
                        .map(|column| {
                            let cell = map.get(column, row).unwrap();

                            match cell {
                                _ if robot_position == (column, row) => '@',
                                Cell::Empty => '.',
                                Cell::Wall => '#',
                                Cell::Box => 'O',
                                Cell::BoxLeft => '[',
                                Cell::BoxRight => ']',
                            }
                        })
                        .collect::<String>()
                })
                .join("\n")
        );
    }
}

fn do_the_shifting(
    mut map: Grid<Cell>,
    mut robot_position: Coord,
    movements: &[Direction],
) -> usize {
    for movement in movements {
        print_map(log::Level::Debug, &map, robot_position);

        let delta = match movement {
            Direction::Up => (NOP, SUB),
            Direction::Right => (ADD, NOP),
            Direction::Down => (NOP, ADD),
            Direction::Left => (SUB, NOP),
        };
        let all_shifting = find_all_shifting(robot_position, &map, delta);

        if let Some(all_shifting) = all_shifting {
            let delta_back = match movement {
                Direction::Up => (NOP, ADD),
                Direction::Right => (SUB, NOP),
                Direction::Down => (NOP, SUB),
                Direction::Left => (ADD, NOP),
            };
            let all_shifting: IndexSet<_> = match movement {
                Direction::Up => all_shifting
                    .into_iter()
                    .sorted_by_key(|(_, y)| *y)
                    .collect(),
                Direction::Right => all_shifting
                    .into_iter()
                    .sorted_by_key(|(x, _)| *x)
                    .rev()
                    .collect(),
                Direction::Down => all_shifting
                    .into_iter()
                    .sorted_by_key(|(_, y)| *y)
                    .rev()
                    .collect(),
                Direction::Left => all_shifting
                    .into_iter()
                    .sorted_by_key(|(x, _)| *x)
                    .collect(),
            };
            for to_shift in &all_shifting {
                let from = (delta_back.0(to_shift.0), delta_back.1(to_shift.1));

                if all_shifting.contains(&from) {
                    map.update(to_shift.0, to_shift.1, *map.get(from.0, from.1).unwrap());
                } else {
                    map.update(to_shift.0, to_shift.1, Cell::Empty)
                }
            }

            robot_position = (delta.0(robot_position.0), delta.1(robot_position.1));
        }
    }

    print_map(log::Level::Info, &map, robot_position);

    map.coordinates()
        .filter(|(column, row)| {
            matches!(map.get(*column, *row).unwrap(), Cell::BoxLeft | Cell::Box)
        })
        .map(|(column, row)| column + row * 100)
        .sum()
}

fn find_all_shifting(
    robot_position: Coord,
    map: &Grid<Cell>,
    delta: Delta,
) -> Option<HashSet<Coord>> {
    let mut queue = VecDeque::new();
    queue.push_back((delta.0(robot_position.0), delta.1(robot_position.1)));
    let mut shifting = HashSet::new();

    while let Some(next @ (column, row)) = queue.pop_front() {
        match *map.get(column, row).unwrap() {
            Cell::Wall => return None, // We're running into a wall, short circuit!
            Cell::Empty => {
                shifting.insert(next);
            }
            Cell::Box => {
                shifting.insert(next);
                let next_to_queue = (delta.0(column), delta.1(row));
                if next_to_queue != next
                    && (0..map.width()).contains(&next_to_queue.0)
                    && (0..map.height()).contains(&next_to_queue.1)
                {
                    queue.push_back(next_to_queue);
                }
            }
            Cell::BoxLeft => {
                let right = (column + 1, row);
                shifting.insert(next);
                shifting.insert(right);

                let next_to_queue_left = (delta.0(column), delta.1(row));
                if next_to_queue_left != next
                    && next_to_queue_left != right
                    && (0..map.width()).contains(&next_to_queue_left.0)
                    && (0..map.height()).contains(&next_to_queue_left.1)
                {
                    queue.push_back(next_to_queue_left);
                }
                let next_to_queue_right = (delta.0(right.0), delta.1(right.1));
                if next_to_queue_right != right
                    && next_to_queue_right != next
                    && (0..map.width()).contains(&next_to_queue_right.0)
                    && (0..map.height()).contains(&next_to_queue_right.1)
                {
                    queue.push_back(next_to_queue_right);
                }
            }
            Cell::BoxRight => {
                let left = (column - 1, row);
                shifting.insert(left);
                shifting.insert(next);

                let next_to_queue_left = (delta.0(left.0), delta.1(left.1));
                if next_to_queue_left != left
                    && next_to_queue_left != next
                    && (0..map.width()).contains(&next_to_queue_left.0)
                    && (0..map.height()).contains(&next_to_queue_left.1)
                {
                    queue.push_back(next_to_queue_left);
                }
                let next_to_queue_right = (delta.0(column), delta.1(row));
                if next_to_queue_right != next
                    && next_to_queue_right != left
                    && (0..map.width()).contains(&next_to_queue_right.0)
                    && (0..map.height()).contains(&next_to_queue_right.1)
                {
                    queue.push_back(next_to_queue_right);
                }
            }
        }
    }

    Some(shifting)
}

fn part_1(input: &Input) -> Output {
    let map = input.map.clone();
    let robot_position = input.robot_position;

    do_the_shifting(map, robot_position, &input.movements)
}

fn part_2(input: &Input) -> Output {
    let map = Grid::new(
        (0..input.map.height())
            .map(|row| {
                (0..input.map.width())
                    .flat_map(|column| match input.map.get(column, row).unwrap() {
                        Cell::Wall => vec![Cell::Wall, Cell::Wall],
                        Cell::Empty => vec![Cell::Empty, Cell::Empty],
                        Cell::Box => vec![Cell::BoxLeft, Cell::BoxRight],
                        cell => panic!("Wrong cell found in original input: {cell:?}"),
                    })
                    .collect()
            })
            .collect(),
    )
    .unwrap();

    let robot_position = (input.robot_position.0 * 2, input.robot_position.1);

    do_the_shifting(map, robot_position, &input.movements)
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

        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 9021);
    }

    #[test]
    fn test_find_all_shifting() {
        //     01234567
        //    0########
        //    1#v.##..#
        //    2#[]..[]#
        //    3#.[][]<#
        //    4#.>[]..#
        //    5#..^...#
        //    6########
        use Cell::*;

        let grid = Grid::new(vec![
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Empty, Empty, Wall, Wall, Empty, Empty, Wall],
            vec![
                Wall, BoxLeft, BoxRight, Empty, Empty, BoxLeft, BoxRight, Wall,
            ],
            vec![
                Wall, Empty, BoxLeft, BoxRight, BoxLeft, BoxRight, Empty, Wall,
            ],
            vec![Wall, Empty, Empty, BoxLeft, BoxRight, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
        ])
        .unwrap();

        let all_shifting_up = find_all_shifting((3, 5), &grid, (NOP, SUB)).unwrap();
        let expected_shifting_up = [
            (1, 1),
            (2, 1),
            (5, 1),
            (6, 1),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (2, 3),
            (3, 3),
            (4, 3),
            (5, 3),
            (3, 4),
            (4, 4),
        ];
        assert!(expected_shifting_up
            .iter()
            .all(|shifting| all_shifting_up.contains(shifting)));
        assert_eq!(expected_shifting_up.len(), all_shifting_up.len());

        let all_shifting_left = find_all_shifting((6, 3), &grid, (SUB, NOP)).unwrap();
        let expected_shifting_left = [(1, 3), (2, 3), (3, 3), (4, 3), (5, 3)];
        assert!(expected_shifting_left
            .iter()
            .all(|shifting| all_shifting_left.contains(shifting)));
        assert_eq!(expected_shifting_left.len(), all_shifting_left.len());

        let all_shifting_down = find_all_shifting((1, 1), &grid, (NOP, ADD)).unwrap();
        let expected_shifting_down = [
            (1, 2),
            (2, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (2, 4),
            (3, 4),
            (4, 4),
            (3, 5),
            (4, 5),
        ];
        assert!(expected_shifting_down
            .iter()
            .all(|shifting| all_shifting_down.contains(shifting)));
        assert_eq!(expected_shifting_down.len(), all_shifting_down.len());

        let all_shifting_right = find_all_shifting((2, 4), &grid, (ADD, NOP)).unwrap();
        let expected_shifting_right = [(3, 4), (4, 4), (5, 4)];
        assert!(expected_shifting_right
            .iter()
            .all(|shifting| all_shifting_right.contains(shifting)));
        assert_eq!(expected_shifting_right.len(), all_shifting_right.len());
    }
}
