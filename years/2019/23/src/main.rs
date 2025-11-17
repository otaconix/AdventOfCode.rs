use std::collections::VecDeque;
use std::io;
use std::ops::Not;

use aoc_timing::trace::log_run;
use intcode::Computer;
use intcode::OpCode;
use intcode::SplitIO;

type Input = Computer;
type Output1 = i64;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap();

    Computer::parse(line.as_ref())
}

fn part_1(input: &Input) -> Output1 {
    let mut computers_with_inputs: Vec<_> = (0..50)
        .map(|n| {
            let computer = input.clone();
            let mut input = VecDeque::new();
            input.push_back(n);
            (computer, input)
        })
        .collect();
    let mut output = VecDeque::new();

    loop {
        for (computer, input) in computers_with_inputs.iter_mut() {
            if computer.run(&mut SplitIO::new(input, &mut output)) == OpCode::Input {
                input.push_back(-1);
            }
        }

        while output.len() >= 3 {
            let dst = output.pop_front().unwrap() as usize;
            let x = output.pop_front().unwrap();
            let y = output.pop_front().unwrap();

            if dst < computers_with_inputs.len() {
                computers_with_inputs[dst].1.push_back(x);
                computers_with_inputs[dst].1.push_back(y);
            }

            if dst == 255 {
                return y;
            }
        }
    }
}

fn part_2(input: &Input) -> Output2 {
    let mut computers_with_ios: Vec<_> = (0..50)
        .map(|n| {
            let computer = input.clone();
            let mut input = VecDeque::new();
            input.push_back(n);

            (computer, input)
        })
        .collect();
    let mut output = VecDeque::new();
    let mut idle_computers = 0u64;
    let mut nat_packet = None;
    let mut last_sent_y_nat = -1;

    loop {
        for n in 0..computers_with_ios.len() {
            let computer_with_io = &mut computers_with_ios[n];
            let input_had_length = computer_with_io.1.len();

            if computer_with_io
                .0
                .run(&mut SplitIO::new(&mut computer_with_io.1, &mut output))
                == OpCode::Input
            {
                computer_with_io.1.push_back(-1);
                idle_computers |= 1 << n;
                // println!("{n} is now idle");
            }

            if (idle_computers & 1 << n != 0) && !output.is_empty()
                || input_had_length > computer_with_io.1.len()
            {
                // println!("{n} is no longer idle");
                idle_computers &= (1u64 << n).not();
            }

            while output.len() >= 3 {
                let dst = output.pop_front().unwrap() as usize;
                let x = output.pop_front().unwrap();
                let y = output.pop_front().unwrap();

                if dst < 50 {
                    computers_with_ios[dst].1.push_back(x);
                    computers_with_ios[dst].1.push_back(y);
                }

                if dst == 255 {
                    // println!("Updating NAT packet to {x},{y}");
                    nat_packet = Some((x, y));
                }
            }
        }

        // println!("Idle computer count: {}", idle_computers.count_ones());

        if idle_computers.count_ones() == 50
            && let Some((x, y)) = nat_packet
        {
            // println!("All computers idle, sending NAT packet {x},{y}");
            computers_with_ios[0].1.push_back(x);
            computers_with_ios[0].1.push_back(y);

            if last_sent_y_nat == y {
                return last_sent_y_nat;
            }
            last_sent_y_nat = y;
        }
    }
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
