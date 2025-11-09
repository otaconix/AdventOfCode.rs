use std::io;

use aoc_timing::trace::log_run;
use intcode::{Computer, NullIO};

type Input = Computer;
type Output1 = Vec<i64>;
type Output2 = i64;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let memory = input
        .map(|line| {
            let line = line.as_ref();

            line.split(',')
                .map(|n| n.parse::<i64>().expect("Invalid i64"))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    Computer::new(memory)
}

fn part_1(computer: &Input) -> Output1 {
    let mut computer = computer.clone();
    computer.write(1, 12);
    computer.write(2, 2);

    computer.run(&mut NullIO);

    computer.memory().to_vec()
}

fn part_2(input: &Input) -> Output2 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = input.clone();
            computer.write(1, noun);
            computer.write(2, verb);

            computer.run(&mut NullIO);

            if computer.read(0, &intcode::ParameterMode::Immediate) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No solution found!");
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {}", part_1[0]);

        let part_2 = log_run("Part 2", || part_2(&input));
        println!("Part 2: {part_2}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 4] = [
        "1,0,0,0,99",
        "2,3,0,3,99",
        "2,4,4,5,99,0",
        "1,1,1,4,99,5,6,0,99",
    ];

    #[test]
    fn test_run_computer() {
        let results = INPUT
            .iter()
            .map(|line| parse(std::iter::once(line)))
            .map(|mut input| {
                input.run(&mut NullIO);
                input.memory().to_owned()
            })
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                vec![2, 0, 0, 0, 99],
                vec![2, 3, 0, 6, 99],
                vec![2, 4, 4, 5, 99, 9801],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ]
        );
    }

    // #[test]
    // fn test_part_1() {
    //     let results = INPUT
    //         .iter()
    //         .map(|line| parse(std::iter::once(line)))
    //         .map(|input| part_1(&input))
    //         .collect::<Vec<_>>();
    //
    //     assert_eq!(
    //         results,
    //         vec![
    //             vec![2, 0, 0, 0, 99],
    //             vec![2, 3, 0, 6, 99],
    //             vec![2, 4, 4, 5, 99, 9801],
    //             vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
    //         ]
    //     );
    // }

    // #[test]
    // fn test_part_2() {
    //     let input = parse(INPUT.lines());
    //     let result = part_2(&input);
    //
    //     assert_eq!(result, 0);
    // }
}
