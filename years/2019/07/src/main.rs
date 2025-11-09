use std::{collections::VecDeque, io};

use aoc_timing::trace::log_run;
use intcode::Computer;
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
        // .inspect(|(permutation, thrust)| {
        //     println!("Max thrust: {thrust} (phase setting sequence: {permutation:?})")
        // })
        .expect("No solution found.")
        .1
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

    // #[test]
    // fn test_part_2() {
    //     let input = parse(INPUT.lines());
    //     let result = part_2(&input);
    //
    //     assert_eq!(result, 0);
    // }
}
