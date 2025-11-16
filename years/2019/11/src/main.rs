use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

use aoc_timing::trace::log_run;
use intcode::Computer;
use intcode::OpCode;
use intcode::SplitIO;
use itertools::Itertools;

type Input = Computer;
type Output1 = usize;
type Output2 = String;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap();

    Computer::parse(line.as_ref())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // 0 => left; 1 => right
    fn turn(&self, left_right: i64) -> Self {
        let left = left_right == 0;

        match (self, left) {
            (Direction::Up, true) | (Direction::Down, false) => Direction::Left,
            (Direction::Up, false) | (Direction::Down, true) => Direction::Right,
            (Direction::Left, false) | (Direction::Right, true) => Direction::Up,
            (Direction::Left, true) | (Direction::Right, false) => Direction::Down,
        }
    }

    fn step(&self, x: i64, y: i64) -> (i64, i64) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

fn part_1(input: &Input) -> Output1 {
    let mut computer = input.clone();
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();
    let mut painted_panels = HashSet::new();
    let mut white_panels = HashSet::new();
    let mut direction = Direction::Up;
    let mut x = 0;
    let mut y = 0;

    input.push_back(0);

    while let OpCode::Input = computer.run(&mut SplitIO::new(&mut input, &mut output)) {
        painted_panels.insert((x, y));

        let color = output.pop_front().expect("No color?");

        if color == 1 {
            white_panels.insert((x, y));
        } else {
            white_panels.remove(&(x, y));
        }

        let turn = output.pop_front().expect("No turn?");

        direction = direction.turn(turn);
        (x, y) = direction.step(x, y);

        input.push_back(if white_panels.contains(&(x, y)) { 1 } else { 0 });
    }

    painted_panels.len()
}

fn part_2(input: &Input) -> Output2 {
    let mut computer = input.clone();
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();
    let mut painted_panels = HashSet::new();
    let mut white_panels = HashSet::from([(0, 0)]);
    let mut direction = Direction::Up;
    let mut x = 0;
    let mut y = 0;

    input.push_back(1);

    while let OpCode::Input = computer.run(&mut SplitIO::new(&mut input, &mut output)) {
        painted_panels.insert((x, y));

        let color = output.pop_front().expect("No color?");

        if color == 1 {
            white_panels.insert((x, y));
        } else {
            white_panels.remove(&(x, y));
        }

        let turn = output.pop_front().expect("No turn?");

        direction = direction.turn(turn);
        (x, y) = direction.step(x, y);

        input.push_back(if white_panels.contains(&(x, y)) { 1 } else { 0 });
    }

    let ((min_x, min_y), (max_x, max_y)) = match white_panels.iter().minmax() {
        itertools::MinMaxResult::NoElements => panic!("No white panels?"),
        itertools::MinMaxResult::OneElement(_) => panic!("Only a single white panel?"),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };

    (*min_y..=*max_y + 1)
        .map(|y| {
            (*min_x..=*max_x)
                .map(|x| {
                    if white_panels.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .join("\n")
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
        println!("Part 2:\n{part_2}");
    });
}
