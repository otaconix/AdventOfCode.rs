use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;
use log::debug;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Wall,
    Empty,
    Box,
}

#[derive(Clone, Copy)]
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

fn print_map(map: &Grid<Cell>, robot_position: Coord) {
    if log::log_enabled!(log::Level::Debug) {
        for row in 0..map.height() {
            let row: String = map
                .row(row)
                .enumerate()
                .map(|(column, cell)| {
                    if robot_position == (column, row) {
                        '@'
                    } else if let Cell::Empty = cell {
                        '.'
                    } else if let Cell::Box = cell {
                        'O'
                    } else {
                        '#'
                    }
                })
                .collect();
            debug!("{row}");
        }
    }
}

fn part_1(input: &Input) -> Output {
    let mut map = input.map.clone();
    let mut robot_position = input.robot_position;

    for movement in &input.movements {
        print_map(&map, robot_position);

        match *movement {
            Direction::Up => {
                let wall_or_empty = map
                    .column(robot_position.0)
                    .enumerate()
                    .take(robot_position.1)
                    .rev()
                    .find(|(_, cell)| matches!(cell, Cell::Empty | Cell::Wall));

                debug!("{robot_position:?} ^ => {wall_or_empty:?}");

                if let Some((empty_row, Cell::Empty)) = wall_or_empty {
                    for update_row in empty_row..robot_position.1 {
                        map.update(
                            robot_position.0,
                            update_row,
                            map.get(robot_position.0, update_row + 1)
                                .unwrap()
                                .to_owned(),
                        );
                    }

                    robot_position.1 -= 1;
                }
            }
            Direction::Right => {
                let wall_or_empty = map
                    .row(robot_position.1)
                    .enumerate()
                    .skip(robot_position.0 + 1)
                    .find(|(_, cell)| matches!(cell, Cell::Empty | Cell::Wall));

                debug!("{robot_position:?} > => {wall_or_empty:?}");

                if let Some((empty_column, Cell::Empty)) = wall_or_empty {
                    for update_column in (robot_position.0 + 1..=empty_column).rev() {
                        map.update(
                            update_column,
                            robot_position.1,
                            map.get(update_column - 1, robot_position.1)
                                .unwrap()
                                .to_owned(),
                        );
                    }

                    robot_position.0 += 1;
                }
            }
            Direction::Down => {
                let wall_or_empty = map
                    .column(robot_position.0)
                    .enumerate()
                    .skip(robot_position.1 + 1)
                    .find(|(_, cell)| matches!(cell, Cell::Empty | Cell::Wall));

                debug!("{robot_position:?} v => {wall_or_empty:?}");

                if let Some((empty_row, Cell::Empty)) = wall_or_empty {
                    for update_row in (robot_position.1 + 1..=empty_row).rev() {
                        map.update(
                            robot_position.0,
                            update_row,
                            map.get(robot_position.0, update_row - 1)
                                .unwrap()
                                .to_owned(),
                        );
                    }

                    robot_position.1 += 1;
                }
            }
            Direction::Left => {
                let wall_or_empty = map
                    .row(robot_position.1)
                    .enumerate()
                    .take(robot_position.0)
                    .rev()
                    .find(|(_, cell)| matches!(cell, Cell::Empty | Cell::Wall));

                debug!("{robot_position:?} < => {wall_or_empty:?}");

                if let Some((empty_column, Cell::Empty)) = wall_or_empty {
                    for update_column in empty_column..robot_position.0 {
                        map.update(
                            update_column,
                            robot_position.1,
                            map.get(update_column + 1, robot_position.1)
                                .unwrap()
                                .to_owned(),
                        );
                    }

                    robot_position.0 -= 1;
                }
            }
        }

        debug!("");
    }

    print_map(&map, robot_position);

    map.coordinates()
        .filter(|(column, row)| matches!(map.get(*column, *row).unwrap(), Cell::Box))
        .map(|(column, row)| column + row * 100)
        .sum()
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

        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 0);
    }
}
