use std::collections::VecDeque;
use std::io;

use aoc_timing::trace::log_run;
use intcode::Computer;

type Input = Computer;
type Output1 = usize;
type Output2 = i64;

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

    for y in 0..50 {
        for x in 0..50 {
            if is_in_tractor_beam(computer, x, y) {
                print!("#");
                in_tractor_beam += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }

    in_tractor_beam
}

fn part_2(computer: &Input) -> Output2 {
    let mut min_x = 0;
    // There are some empty rows at the top, so let's skip the first 50
    for y in 50.. {
        if let Some(x) = (min_x..)
            .skip_while(|&x| !is_in_tractor_beam(computer, x, y))
            .take_while(|&x| is_in_tractor_beam(computer, x, y))
            .last()
        {
            min_x = x;

            // Gotta do ±99, not 100. Silly me!
            if is_in_tractor_beam(computer, x - 99, y)
                && is_in_tractor_beam(computer, x - 99, y + 99)
            {
                return (x - 99) * 10_000 + y;
            }
        }
    }

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
