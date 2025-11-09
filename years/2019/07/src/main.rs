use std::{collections::VecDeque, io};

use aoc_timing::trace::log_run;
use intcode::{Computer, OpCode, SplitIO};
use itertools::Itertools;

type Input = Computer;
type Output1 = i64;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap();

    Computer::parse(line.as_ref())
}

fn part_1(input: &Input) -> Output1 {
    (0..5)
        .permutations(5)
        .map(|permutation| {
            (
                permutation.clone(),
                permutation
                    .iter()
                    .zip(std::iter::successors(Some(input.clone()), |_| {
                        Some(input.clone())
                    }))
                    .fold(0, |last_output, (input, mut computer)| {
                        let mut io = VecDeque::from([*input, last_output]);
                        computer.run(&mut io);
                        computer.diagnostic_code(&io)
                    }),
            )
        })
        .max_by_key(|(_, thrust)| *thrust)
        .inspect(|(permutation, thrust)| {
            println!("Max thrust: {thrust} (phase setting sequence: {permutation:?})")
        })
        .expect("No solution found.")
        .1
}

fn part_2(input: &Input) -> Output2 {
    (5i64..10)
        .permutations(5)
        .map(|permutation| {
            log::info!("Trying permutation {permutation:?}");
            let mut computers: Vec<_> = (0..5).map(|_| input.clone()).collect();
            let mut ios: Vec<_> = permutation
                .iter()
                .map(|input| VecDeque::from([*input]))
                .collect();
            ios[0].push_back(0);

            let mut last_opcodes = [
                OpCode::Input,
                OpCode::Input,
                OpCode::Input,
                OpCode::Input,
                OpCode::Input,
            ];

            while last_opcodes
                .iter()
                .any(|opcode| *opcode != OpCode::Terminate)
            {
                for n in 0..5 {
                    let [input, output] = ios.get_disjoint_mut([n, (n + 1) % 5]).unwrap();
                    last_opcodes[n] = computers[n].run(&mut SplitIO::new(input, output));
                    log::info!(
                        "Stopped running computer {n} at opcode {:?} (IP: {})",
                        last_opcodes[n],
                        computers[n].instruction_pointer
                    );
                }
            }

            log::info!("I/O's after finishing: {ios:#?}");
            (permutation.clone(), *ios[0].iter().last().unwrap())
        })
        .max_by_key(|(_, thrust)| *thrust)
        .inspect(|(permutation, thrust)| {
            println!("Max thrust: {thrust} (phase setting sequence: {permutation:?})")
        })
        .expect("No solution found.")
        .1
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

    #[test]
    fn test_part_1() {
        for (input, expected) in [
            ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210),
            (
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
                54321,
            ),
            (
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
                65210,
            ),
        ] {
            let input = Computer::parse(input);
            let result = part_1(&input);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_2() {
        for (input, expected) in [
            (
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
                139629729,
            ),
            (
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
                18216,
            ),
        ] {
            let input = Computer::parse(input);
            let result = part_2(&input);

            assert_eq!(result, expected);
        }
    }
}
