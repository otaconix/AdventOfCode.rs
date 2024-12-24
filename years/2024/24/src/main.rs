use std::io;

use aoc_timing::trace::log_run;
use rapidhash::RapidHashMap;

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: u8,
}

#[derive(Debug, Clone, Copy)]
enum InstructionType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Instruction {
    ty: InstructionType,
    left: String,
    right: String,
    output: String,
}

impl Instruction {
    fn evaluate(&self, input: &Input, wires: &mut RapidHashMap<String, u8>) -> u8 {
        if let Some(result) = wires.get(&self.output) {
            *result
        } else {
            let left = input
                .outputs_to_instructions
                .get(&self.left)
                .map(|instruction| instruction.evaluate(input, wires))
                .unwrap_or(wires[&self.left]);
            let right = input
                .outputs_to_instructions
                .get(&self.right)
                .map(|instruction| instruction.evaluate(input, wires))
                .unwrap_or(wires[&self.right]);

            let result = match self.ty {
                InstructionType::And => left & right,
                InstructionType::Or => left | right,
                InstructionType::Xor => left ^ right,
            };

            wires.insert(self.output.clone(), result);

            result
        }
    }
}

struct Input {
    wires: RapidHashMap<String, u8>,
    outputs_to_instructions: RapidHashMap<String, Instruction>,
}
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum ParsingState {
        Wires(RapidHashMap<String, u8>),
        Instructions(RapidHashMap<String, u8>, Vec<Instruction>),
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

                    instructions.push(Instruction {
                        ty: match instruction {
                            "AND" => InstructionType::And,
                            "OR" => InstructionType::Or,
                            "XOR" => InstructionType::Xor,
                            _ => panic!("Unknown instruction {instruction}"),
                        },
                        left: left.to_string(),
                        right: right.to_string(),
                        output: output.to_string(),
                    });

                    ParsingState::Instructions(wires, instructions)
                }
            }
        },
    );

    match end_state {
        ParsingState::Instructions(wires, instructions) => Input {
            wires,
            outputs_to_instructions: instructions.into_iter().fold(
                RapidHashMap::default(),
                |mut map, instruction| {
                    map.insert(instruction.output.clone(), instruction);

                    map
                },
            ),
        },
        _ => panic!("Haven't reached instructions stage while parsing"),
    }
}

fn part_1(input: &Input) -> Output1 {
    let mut wires = input.wires.clone();

    input
        .outputs_to_instructions
        .iter()
        .filter(|(output, _)| output.starts_with('z'))
        .map(|(_, instruction)| {
            (
                instruction.output[1..].parse::<usize>().unwrap(),
                instruction.evaluate(input, &mut wires),
            )
        })
        .fold(0usize, |acc, (index, value)| {
            let value = value as usize;

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
