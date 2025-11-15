use std::collections::VecDeque;
use std::io;

use aoc_timing::trace::log_run;
use intcode::Computer;
use itertools::Itertools;
use itertools::MinMaxResult;

type Input = Computer;
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap();

    Computer::parse(line.as_ref())
}

fn is_in_tractor_beam(computer: &Computer, x: i64, y: i64) -> bool {
    let mut computer = computer.clone();
    let mut io = VecDeque::new();

    io.push_back(x);
    io.push_back(y);

    computer.run(&mut io);

    io[0] == 1
}

fn part_1(computer: &Input) -> Output1 {
    let mut in_tractor_beam = 0;
    let mut min_x = 0;

    for y in 0..5000 {
        in_tractor_beam += match (min_x..5000)
            .skip_while(|&x| !is_in_tractor_beam(computer, x, y))
            .take_while(|&x| is_in_tractor_beam(computer, x, y))
            .minmax()
        {
            MinMaxResult::NoElements => 0,
            MinMaxResult::OneElement(x) => {
                min_x = x;

                1
            }
            MinMaxResult::MinMax(first_x, last_x) => {
                min_x = first_x;

                1usize + (last_x - first_x) as usize
            }
        }
    }

    in_tractor_beam
}

fn part_2(input: &Input) -> Output2 {
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
