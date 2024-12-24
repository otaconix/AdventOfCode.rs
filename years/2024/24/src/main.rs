use std::io;

use aoc_timing::trace::log_run;
use rapidhash::RapidHashMap;

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: u8,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    And,
    Or,
    Xor,
}

impl Instruction {
    fn evaluate(&self, left: u8, right: u8) -> u8 {
        match self {
            Instruction::And => left & right,
            Instruction::Or => left | right,
            Instruction::Xor => left ^ right,
        }
    }
}

struct Input {
    wires: RapidHashMap<String, u8>,
    instructions: Vec<((String, String), (Instruction, String))>,
}
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum ParsingState {
        Wires(RapidHashMap<String, u8>),
        Instructions(
            RapidHashMap<String, u8>,
            Vec<((String, String), (Instruction, String))>,
        ),
    }
    let end_state = input.fold(
        ParsingState::Wires(RapidHashMap::default()),
        |state, line| {
            let line = line.as_ref();

            match state {
                ParsingState::Wires(wires) if line.is_empty() => {
                    ParsingState::Instructions(wires, Vec::new())
                }
                ParsingState::Wires(mut wires) => {
                    let (name, value) = line.split_once(": ").unwrap();
                    let value = match value {
                        "0" => 0,
                        "1" => 1,
                        _ => panic!("Invalid binary value {value}"),
                    };

                    wires.insert(name.to_string(), value);

                    ParsingState::Wires(wires)
                }
                ParsingState::Instructions(wires, mut instructions) => {
                    let (instruction, output) = line.split_once(" -> ").unwrap();
                    let (left, instruction) = instruction.split_once(' ').unwrap();
                    let (instruction, right) = instruction.split_once(' ').unwrap();

                    instructions.push((
                        (left.to_string(), right.to_string()),
                        (
                            match instruction {
                                "AND" => Instruction::And,
                                "OR" => Instruction::Or,
                                "XOR" => Instruction::Xor,
                                _ => panic!("Unknown instruction {instruction}"),
                            },
                            output.to_string(),
                        ),
                    ));

                    ParsingState::Instructions(wires, instructions)
                }
            }
        },
    );

    match end_state {
        ParsingState::Instructions(wires, instructions) => Input {
            wires,
            instructions,
        },
        _ => panic!("Haven't reached instructions stage while parsing"),
    }
}

fn part_1(input: &Input) -> Output1 {
    let mut wires = input.wires.clone();
    let mut instructions = input.instructions.clone();

    while !instructions.is_empty() {
        let instruction_index = instructions
            .iter()
            .enumerate()
            .find(|(_index, ((left, right), _))| {
                wires.contains_key(left) && wires.contains_key(right)
            })
            .unwrap()
            .0;
        let ((left, right), (instruction, output)) = instructions[instruction_index].clone();

        instructions.remove(instruction_index);

        wires.insert(
            output.to_string(),
            instruction.evaluate(wires[&left], wires[&right]),
        );
    }

    wires
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .fold(0usize, |acc, (name, value)| {
            let index: usize = name[1..].parse().unwrap();
            let value = *value as usize;

            acc | (value << index)
        })
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

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 2024);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 0);
    }
}
