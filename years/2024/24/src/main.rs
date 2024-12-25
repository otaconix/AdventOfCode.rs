use std::collections::HashMap;
use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InstructionType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    ty: InstructionType,
    left: String,
    right: String,
    output: String,
}

impl Instruction {
    fn evaluate(
        &self,
        outputs_to_instructions: &HashMap<&str, &Instruction>,
        wires: &mut HashMap<String, u8>,
    ) -> u8 {
        if let Some(result) = wires.get(&self.output) {
            *result
        } else {
            let left = outputs_to_instructions
                .get(self.left.as_str())
                .map(|instruction| instruction.evaluate(outputs_to_instructions, wires))
                .unwrap_or(wires[&self.left]);
            let right = outputs_to_instructions
                .get(self.right.as_str())
                .map(|instruction| instruction.evaluate(outputs_to_instructions, wires))
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
    wires: HashMap<String, u8>,
    instructions: Vec<Instruction>,
}
type Output1 = usize;
type Output2 = String;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum ParsingState {
        Wires(HashMap<String, u8>),
        Instructions(HashMap<String, u8>, Vec<Instruction>),
    }
    let end_state = input.fold(ParsingState::Wires(HashMap::default()), |state, line| {
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
    });

    match end_state {
        ParsingState::Instructions(wires, instructions) => Input {
            wires,
            instructions,
        },
        _ => panic!("Haven't reached instructions stage while parsing"),
    }
}

fn evaluate_prefix(
    prefix: char,
    outputs_to_instructions: &HashMap<&str, &Instruction>,
    wires: &mut HashMap<String, u8>,
) -> usize {
    outputs_to_instructions
        .iter()
        .filter(|(output, _)| output.starts_with(prefix))
        .map(|(_, instruction)| {
            (
                instruction.output[1..].parse::<usize>().unwrap(),
                instruction.evaluate(outputs_to_instructions, wires),
            )
        })
        .fold(0usize, |acc, (index, value)| {
            let value = value as usize;

            acc | (value << index)
        })
}

fn part_1(input: &Input) -> Output1 {
    let mut wires = input.wires.clone();
    let outputs_to_instructions = input
        .instructions
        .iter()
        .map(|instruction| (instruction.output.as_str(), instruction))
        .collect();

    evaluate_prefix('z', &outputs_to_instructions, &mut wires)
}

fn is_full_adder(bit: u8, instructions_by_input: &HashMap<&str, Vec<&Instruction>>) -> bool {
    use InstructionType::*;

    let x_wire = format!("x{bit:02}");
    let y_wire = format!("y{bit:02}");
    let z_wire = format!("z{bit:02}");
    let xy_xor = instructions_by_input[x_wire.as_str()]
        .iter()
        .find(|instruction| instruction.ty == Xor);
    let xy_and = instructions_by_input[x_wire.as_str()]
        .iter()
        .find(|instruction| instruction.ty == And);

    match (xy_xor, xy_and) {
        (Some(xy_xor), Some(xy_and)) => {
            if !((xy_xor.left == x_wire && xy_xor.right == y_wire)
                || (xy_xor.left == y_wire && xy_xor.right == x_wire))
            {
                println!("{bit} wrong because first XOR doesn't have x & y as inputs");
                false
            } else if !((xy_and.left == x_wire && xy_and.right == y_wire)
                || (xy_and.left == y_wire && xy_and.right == x_wire))
            {
                println!("{bit} wrong because first AND doesn't have x & y as inputs");
                false
            } else {
                let second_and =
                    instructions_by_input
                        .get(xy_xor.output.as_str())
                        .and_then(|instructions| {
                            instructions
                                .iter()
                                .find(|instruction| instruction.ty == And)
                        });
                let second_xor =
                    instructions_by_input
                        .get(xy_xor.output.as_str())
                        .and_then(|instructions| {
                            instructions
                                .iter()
                                .find(|instruction| instruction.ty == Xor)
                        });
                let or =
                    instructions_by_input
                        .get(xy_and.output.as_str())
                        .and_then(|instructions| {
                            instructions.iter().find(|instruction| instruction.ty == Or)
                        });

                match (second_and, second_xor, or) {
                    (Some(second_and), Some(second_xor), Some(or)) => {
                        if second_xor.output != z_wire {
                            println!(
                                "{bit} wrong because its second XOR doesn't output into {z_wire}"
                            );
                            false
                        } else if !((or.left == xy_and.output && or.right == second_and.output)
                            || (or.left == second_and.output && or.right == xy_and.output))
                        {
                            println!("{bit} wrong because its second XOR doesn't have the first and second ANDs as inputs");
                            false
                        } else {
                            true
                        }
                    }
                    _ => {
                        println!(
                            "{bit} wrong because it has no second AND ({}), second XOR ({}) or final OR ({})", second_and.is_none(), second_xor.is_none(), or.is_none()
                        );
                        false
                    }
                }
            }
        }
        _ => {
            println!(
                "{bit} wrong because x and y aren't inputs to an XOR ({}) and AND ({})",
                xy_xor.is_none(),
                xy_and.is_none()
            );
            false
        }
    }
}

fn swap_instructions(instructions: &[Instruction], swap: &[String; 2]) -> Vec<Instruction> {
    instructions
        .iter()
        .cloned()
        .map(|instruction| {
            if instruction.output == swap[0] {
                Instruction {
                    output: swap[1].clone(),
                    ..instruction
                }
            } else if instruction.output == swap[1] {
                Instruction {
                    output: swap[0].clone(),
                    ..instruction
                }
            } else {
                instruction
            }
        })
        .collect()
}

fn swapped_outputs(
    bit: u8,
    instructions: &[Instruction],
    instructions_by_input: &HashMap<&str, Vec<&Instruction>>,
) -> [String; 2] {
    let x_wire = format!("x{bit:02}");
    let first_instructions = instructions_by_input.get(x_wire.as_str()).unwrap();
    let second_instructions = first_instructions
        .iter()
        .flat_map(|instruction| instructions_by_input.get(instruction.output.as_str()))
        .flatten()
        .collect_vec();
    let third_instructions = second_instructions
        .iter()
        .flat_map(|instruction| instructions_by_input.get(instruction.output.as_str()))
        .flatten()
        .collect_vec();

    first_instructions
        .iter()
        .chain(second_instructions)
        .chain(third_instructions)
        .map(|instruction| instruction.output.as_str())
        .combinations(2)
        .map(|swap| [swap[0].to_string(), swap[1].to_string()])
        .find(|swap| {
            let instructions: Vec<Instruction> = swap_instructions(instructions, swap);
            let instructions_by_input = instructions.iter().fold(
                HashMap::<&str, Vec<&Instruction>>::default(),
                |mut map, instruction| {
                    map.entry(&instruction.left).or_default().push(instruction);
                    map.entry(&instruction.right).or_default().push(instruction);

                    map
                },
            );

            is_full_adder(bit, &instructions_by_input)
        })
        .unwrap()
}

fn part_2(input: &Input) -> Output2 {
    let input_bits = input
        .wires
        .keys()
        .filter(|wire| wire.starts_with('x'))
        .map(|wire| wire[1..].parse::<u8>().unwrap())
        .max()
        .unwrap();
    let instructions_by_input = input.instructions.iter().fold(
        HashMap::<&str, Vec<&Instruction>>::default(),
        |mut map, instruction| {
            map.entry(&instruction.left).or_default().push(instruction);
            map.entry(&instruction.right).or_default().push(instruction);

            map
        },
    );

    (1..=input_bits)
        .filter(|bit| !is_full_adder(*bit, &instructions_by_input))
        .flat_map(|bit| swapped_outputs(bit, &input.instructions, &instructions_by_input))
        .sorted()
        .join(",")
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

    // #[test]
    // fn test_part_2() {
    //     let input = parse(INPUT.lines());
    //     let result = part_2(&input, 2, |x, y| x & y);
    //
    //     assert_eq!(result, "z00,z01,z02,z05");
    // }
}
