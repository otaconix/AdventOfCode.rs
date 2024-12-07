use std::io;

use aoc_timing::trace::log_run;

type Input = Vec<Equation>;
type Output = u64;

#[derive(Debug)]
struct Equation {
    test_value: Output,
    operands: Vec<Output>,
}

enum Operator {
    Multiply,
    Add,
    Concatenate,
}

impl Operator {
    fn evaluate(&self, left: Output, right: Output) -> Output {
        match *self {
            Operator::Multiply => left * right,
            Operator::Add => left + right,
            Operator::Concatenate => {
                let mut right_copy = right;
                let mut left_copy = left;
                while right_copy > 0 {
                    left_copy *= 10;
                    right_copy /= 10;
                }

                left_copy + right
            }
        }
    }
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            let (test_value, operands) = line.split_once(": ").expect("No ': ' in equation?");
            let operands = operands
                .split(' ')
                .map(|operand| operand.parse().expect("Invalid operand"))
                .collect();

            Equation {
                test_value: test_value.parse().expect("Invalid operand"),
                operands,
            }
        })
        .collect()
}

fn try_solve(
    test_value: Output,
    operators: &[Operator],
    result: Output,
    operands: &[Output],
) -> bool {
    if result > test_value {
        false
    } else if operands.is_empty() {
        test_value == result
    } else {
        operators.iter().any(|operator| {
            try_solve(
                test_value,
                operators,
                operator.evaluate(result, operands[0]),
                &operands[1..],
            )
        })
    }
}

fn part_with_operators(input: &Input, operators: &[Operator]) -> Output {
    input
        .iter()
        .filter(|equation| {
            try_solve(
                equation.test_value,
                operators,
                equation.operands[0],
                &equation.operands[1..],
            )
        })
        .map(|equation| equation.test_value)
        .sum()
}

fn part_1(input: &Input) -> Output {
    part_with_operators(input, &[Operator::Add, Operator::Multiply])
}

fn part_2(input: &Input) -> Output {
    part_with_operators(
        input,
        &[Operator::Add, Operator::Multiply, Operator::Concatenate],
    )
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

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 11387);
    }
}
